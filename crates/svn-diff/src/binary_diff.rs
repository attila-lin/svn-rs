//! binary_diff.c:  handling of git like binary diffs

use std::fs::File;

/// Copies the data from ORIGINAL_STREAM to a temporary file, returning both
/// the original and compressed size.
/// `create_compressed`
pub fn create_compressed(original_stream: &impl std::io::Read) -> File {
    todo!()
}
