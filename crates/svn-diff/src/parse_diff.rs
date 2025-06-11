use std::fs::File;

use crate::patch::SvnPatch;
///
/// `svn_diff_binary_patch_t`
pub struct DiffBinaryPatch {
    /// The patch this hunk belongs to.
    patch: SvnPatch,

    /// APR file handle to the patch file this hunk came from.
    file: File,

    // FIXME: make a new struct

    // Offsets inside APR_FILE representing the location of the patch
    src_start: u64,
    src_end: u64,
    /// Expanded/final size
    src_filesize: i64,

    // Offsets inside APR_FILE representing the location of the patch
    dst_start: u64,
    dst_end: u64,
    ///  Expanded/final size
    dst_filesize: i64,
}
