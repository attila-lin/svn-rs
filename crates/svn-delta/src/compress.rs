/// `SVN_DELTA_COMPRESSION_LEVEL_*`
pub struct DeltaCompressLevel(u8);

impl DeltaCompressLevel {
    pub fn max() -> Self {
        DeltaCompressLevel(9) // Maximum compression level
    }
    pub fn min() -> Self {
        DeltaCompressLevel(0) // Minimum compression level
    }
}

impl Default for DeltaCompressLevel {
    fn default() -> Self {
        // This is the default compression level we pass to zlib.  It
        // should be between 0 and 9, with higher numbers resulting in
        // better compression rates but slower operation.
        DeltaCompressLevel(5) // Default compression level
    }
}
