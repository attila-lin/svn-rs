use crate::patch::SvnDiffHunk;

/// svn_diff_patch_parser_t implementation
///
/// `svn_diff_patch_parser_t`
pub struct DiffPatchParser;

/// `last_line_type`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LastLineType {
    Noise,
    Original,
    Modified,
    Context,
}

#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum ParseError {}

impl DiffPatchParser {
    /// Return the next *HUNK from a PATCH in APR_FILE.
    /// If no hunk can be found, set *HUNK to NULL.
    /// Set IS_PROPERTY to TRUE if we have a property hunk. If the returned HUNK
    /// is the first belonging to a certain property, then PROP_NAME and
    /// PROP_OPERATION will be set too. If we have a text hunk, PROP_NAME will be
    /// NULL.  If IGNORE_WHITESPACE is TRUE, lines without leading spaces will be
    /// treated as context lines.  Allocate results in RESULT_POOL.
    ///
    /// `parse_next_hunk`
    pub fn parse(input: &str) -> Result<SvnDiffHunk, ParseError> {
        const MINUS: &str = "--- ";
        const TEXT_ATAT: &str = "@@";
        const PROP_ATAT: &str = "##";

        let mut last_line_type = LastLineType::Noise;

        let lines: Vec<&str> = input.lines().collect();

        for line in lines {}

        todo!()
    }
}
