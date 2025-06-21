use svn_types::RevisionNumber;

/// A revision, specified in one of @c svn_opt_revision_kind ways.
///
/// `svn_opt_revision_t`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OptRevision(OptRevisionKind);

/// Various ways of specifying revisions.
///
/// @note
/// In contexts where local mods are relevant, the `working' kind
/// refers to the uncommitted "working" revision, which may be modified
/// with respect to its base revision.  In other contexts, `working'
/// should behave the same as `committed' or `current'.
///
/// `svn_opt_revision_kind`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptRevisionKind {
    Unspecified(RevisionNumber),
    Number(RevisionNumber),
    Date(i64),
    Committed(RevisionNumber),
    Previous(RevisionNumber),
    Base(RevisionNumber),
    Working(RevisionNumber),
    Head,
}

/* Extract the peg revision, if any, from UTF8_TARGET.
 *
 * If PEG_REVISION is not NULL, return the peg revision in *PEG_REVISION.
 * *PEG_REVISION will be an empty string if no peg revision is found.
 * Return the true target portion in *TRUE_TARGET.
 *
 * UTF8_TARGET need not be canonical. *TRUE_TARGET will not be canonical
 * unless UTF8_TARGET is.
 *
 * Note that *PEG_REVISION will still contain the '@' symbol as the first
 * character if a peg revision was found. If a trailing '@' symbol was
 * used to escape other '@' characters in UTF8_TARGET, *PEG_REVISION will
 * point to the string "@", containing only a single character.
 *
 * All allocations are done in POOL.
 */
pub fn split_arg_at_peg_revision(utf8_target: &str) -> (&str, &str) {
    let mut peg_start = None;

    // Search backwards from the end of the string
    let chars: Vec<char> = utf8_target.chars().collect();
    for (i, &ch) in chars.iter().enumerate().rev() {
        // If we hit a path separator, stop looking.  This is OK
        // only because our revision specifiers can't contain '/'.
        if ch == '/' {
            break;
        }

        if ch == '@' {
            peg_start = Some(i);
            break;
        }
    }

    if let Some(peg_idx) = peg_start {
        // Convert char index to byte index
        let byte_idx = utf8_target
            .char_indices()
            .nth(peg_idx)
            .map(|(i, _)| i)
            .unwrap_or(0);
        let true_target = &utf8_target[..byte_idx];
        let peg_revision = &utf8_target[byte_idx..];
        (true_target, peg_revision)
    } else {
        (utf8_target, "")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_arg_at_peg_revision() {
        // Test case 1: No peg revision
        let (true_target, peg_rev) = split_arg_at_peg_revision("path/to/file.txt");
        assert_eq!(true_target, "path/to/file.txt");
        assert_eq!(peg_rev, "");

        // Test case 2: Simple peg revision with number
        let (true_target, peg_rev) = split_arg_at_peg_revision("path/to/file.txt@123");
        assert_eq!(true_target, "path/to/file.txt");
        assert_eq!(peg_rev, "@123");

        // Test case 3: Peg revision with HEAD
        let (true_target, peg_rev) = split_arg_at_peg_revision("path/to/file.txt@HEAD");
        assert_eq!(true_target, "path/to/file.txt");
        assert_eq!(peg_rev, "@HEAD");

        // Test case 4: Peg revision with date
        let (true_target, peg_rev) = split_arg_at_peg_revision("path/to/file.txt@{2023-01-01}");
        assert_eq!(true_target, "path/to/file.txt");
        assert_eq!(peg_rev, "@{2023-01-01}");

        // Test case 5: Multiple @ symbols, only last one counts
        let (true_target, peg_rev) = split_arg_at_peg_revision("path@with@at/file.txt@123");
        assert_eq!(true_target, "path@with@at/file.txt");
        assert_eq!(peg_rev, "@123");

        // Test case 6: @ in directory name, but after path separator
        let (true_target, peg_rev) = split_arg_at_peg_revision("path@dir/file.txt");
        assert_eq!(true_target, "path@dir/file.txt");
        assert_eq!(peg_rev, "");

        // Test case 7: URL with peg revision
        let (true_target, peg_rev) =
            split_arg_at_peg_revision("http://svn.example.com/repo/trunk@456");
        assert_eq!(true_target, "http://svn.example.com/repo/trunk");
        assert_eq!(peg_rev, "@456");

        // Test case 8: Empty string
        let (true_target, peg_rev) = split_arg_at_peg_revision("");
        assert_eq!(true_target, "");
        assert_eq!(peg_rev, "");

        // Test case 9: Just @ symbol
        let (true_target, peg_rev) = split_arg_at_peg_revision("@");
        assert_eq!(true_target, "");
        assert_eq!(peg_rev, "@");

        // Test case 10: Trailing @ to escape other @ symbols
        let (true_target, peg_rev) = split_arg_at_peg_revision("file@name@");
        assert_eq!(true_target, "file@name");
        assert_eq!(peg_rev, "@");

        // Test case 11: Only @ at the end (no path)
        let (true_target, peg_rev) = split_arg_at_peg_revision("@123");
        assert_eq!(true_target, "");
        assert_eq!(peg_rev, "@123");
    }
}
