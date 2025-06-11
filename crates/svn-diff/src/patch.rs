use std::fs::File;
use std::sync::Arc;

use crate::DiffOperationKind;

/// This struct describes a range within a file, as well as the
/// current cursor position within the range. All numbers are in bytes.
///
/// `svn_diff__hunk_range`
#[derive(Default, Debug)]
pub struct DiffHunkRange {
    start: usize,
    end: usize,
    current: usize,
}

/// Data type to manage parsing of patches.
///
/// Represents a patch to one target file.
///
/// API users should not allocate structures of this type directly.
///
/// `svn_patch_t`
pub struct SvnPatch {
    // The old and new file names as retrieved from the patch file.
    // These paths are UTF-8 encoded and canonicalized, but otherwise
    // left unchanged from how they appeared in the patch file.
    old_filename: String,
    new_filename: String,

    /// An array containing an svn_diff_hunk_t * for each hunk parsed
    /// from the patch.
    hunks: Vec<SvnDiffHunk>,
}

//// A single hunk inside a patch.
///
/// The lines of text comprising the hunk can be interpreted in three ways:
/// - diff text       The hunk as it appears in the unidiff patch file,
///                   including the hunk header line ("@@ ... @@")
/// - original text   The text the patch was based on.
/// - modified text   The result of patching the original text.
///
/// For example, consider a hunk with the following diff text:
///
///  @verbatim
///    @@ -1,5 +1,5 @@
///     #include <stdio.h>
///     int main(int argc, char *argv[]) {
/// -        printf("Hello World!\n");
/// +        printf("I like Subversion!\n");
///  } @endverbatim
///
/// The original text of this hunk is:
///
/// @verbatim
/// #include <stdio.h>
/// int main(int argc, char *argv[]) {
///         printf("Hello World!\n");
/// } @endverbatim
///
/// And the modified text is:
///
/// @verbatim
/// #include <stdio.h>
/// int main(int argc, char *argv[]) {
///         printf("I like Subversion!\n");
/// } @endverbatim
///
/// @see svn_diff_hunk_readline_diff_text()
/// @see svn_diff_hunk_readline_original_text()
/// @see svn_diff_hunk_readline_modified_text()
///
/// `svn_diff_hunk_t`
pub struct SvnDiffHunk {
    /// The patch this hunk belongs to.
    patch: SvnPatch,
    /// APR file handle to the patch file this hunk came from.
    file: Arc<File>,
    /// Whether the hunk was interpreted as pretty-print mergeinfo. If so,
    /// the hunk content is in PATCH and the rest of this hunk object is
    /// mostly uninitialized.
    is_pretty_print_mergeinfo: bool,

    // Ranges used to keep track of this hunk's texts positions within
    // the patch file.
    diff_text_range: DiffHunkRange,
    original_text_range: DiffHunkRange,
    modified_text_range: DiffHunkRange,

    // Hunk ranges as they appeared in the patch file.
    // All numbers are lines, not bytes.
    original_start: u64,
    original_length: u64,
    modified_start: u64,
    modified_length: u64,

    // Number of lines of leading and trailing hunk context.
    leading_context: u64,
    trailing_context: u64,

    //  Did we see a 'file does not end with eol' marker in this hunk?
    original_no_final_eol: bool,
    modified_no_final_eol: bool,

    // Fuzz penalty, triggered by bad patch targets
    original_fuzz: u64,
    modified_fuzz: u64,
}

impl SvnDiffHunk {
    pub fn add_single_line(&mut self, line: &str, patch: SvnPatch) {
        self._add_or_delete_single_line(line, patch, true)
    }

    pub fn delete_single_line(&mut self, line: &str, patch: SvnPatch) {
        self._add_or_delete_single_line(line, patch, false)
    }

    /// `add_or_delete_single_line`
    fn _add_or_delete_single_line(&mut self, line: &str, patch: SvnPatch, is_add: bool) {
        const HUNK_HEADER: [&str; 2] = ["@@ -1 +0,0 @@\n", "@@ -0,0 +1 @@\n"];

        let header_len = HUNK_HEADER[if is_add { 1usize } else { 0usize }].len();
        let len = line.len();
        let end = header_len + 1 + len; // The +1 is for the \n.

        self.patch = patch;

        self.diff_text_range.start = header_len;
        self.diff_text_range.current = header_len;

        if is_add {
            // There's no "original" text.
            self.original_text_range = DiffHunkRange::default();
            self.original_no_final_eol = false;

            // self.modified
            todo!()
        } else {
            todo!()
        }
    }

    ///  Allocate @a *stringbuf in @a result_pool, and read into it one line
    /// of the diff text of @a hunk. The hunk header is not returned only the
    /// unidiff data lines (starting with '+', '-', or ' ') are returned.
    /// If the @a hunk is being interpreted in reverse (i.e. the reverse
    /// parameter of svn_diff_parse_next_patch() was @c TRUE), the diff
    /// text will be returned in reversed form.
    /// The line-terminator is detected automatically and stored in @a *eol
    /// if @a eol is not NULL.
    /// If EOF is reached, set @a *eof to TRUE, and set @a *eol to NULL if the
    /// hunk does not end with a newline character and @a eol is not NULL.
    /// Temporary allocations will be performed in @a scratch_pool.
    ///
    /// @note The hunk header information can be retrieved with the following
    /// functions:
    /// @see svn_diff_hunk_get_original_start()
    /// @see svn_diff_hunk_get_original_length()
    /// @see svn_diff_hunk_get_modified_start()
    /// @see svn_diff_hunk_get_modified_length()
    /// `svn_diff_hunk_readline_diff_text`
    pub fn readline_diff_text(&self) {
        todo!()
    }
}

/// Data type to manage parsing of properties in patches.
/// API users should not allocate structures of this type directly.
///
/// `svn_prop_patch_t`
pub struct SvnPropPatch {
    name: String,
    /// Represents the operation performed on the property
    operation: DiffOperationKind,
    /// An array containing a `svn_diff_hunk_t` object for each hunk parsed
    /// from the patch associated with our property name
    hunks: Vec<SvnDiffHunk>,
}
