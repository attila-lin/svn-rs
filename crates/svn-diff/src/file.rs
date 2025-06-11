use std::fs::File;

/// Open @a patch_file at @a local_abspath.
///
/// TODO: simplify this struct to just a file handle.
///
/// `svn_patch_file_t`
pub struct SvnPatchFile {
    /// The APR file handle to the patch file.
    file: File,
    // /// Inner parser for svn_diff_patch_parser_next()
    // parser: DiffPatchParser,
}

impl SvnPatchFile {
    /// Open @a patch_file at @a local_abspath.
    pub fn open(local_abspath: &str) -> Self {
        todo!()
    }
}

/// Possible states of the diff header parser.
/// `parse_state`
pub enum ParseState {
    /// initial
    Start,
    /// diff --git
    GitDiffSeen,
    /// a tree operation, rather than content change
    GitTreeSeen,
    /// --- /dev/null; or --- a/
    GitMinusSeen,
    /// +++ /dev/null; or +++ a/
    GitPlusSeen,
    /// old mode 100644
    OldModeSeen,
    /// new mode 100644
    GitModeSeen,
    /// rename from foo.c
    MoveFromSeen,
    /// copy from foo.c
    CopyFromSeen,
    /// --- foo.c
    MinusSeen,
    /// valid start of a regular unidiff header
    UnidiffFound,
    /// valid start of a --git diff header
    GitHeaderFound,
    /// valid start of binary patch
    BinaryPatchFound,
}
