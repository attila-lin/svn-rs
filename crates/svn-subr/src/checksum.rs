use md5::{Digest, Md5};

/// Various types of checksums.
pub enum ChecksumKind {
    /// The checksum is (or should be set to) an MD5 checksum.
    Md5,
    /// The checksum is (or should be set to) a SHA1 checksum.
    Sha1,
    /// The checksum is (or should be set to) a FNV-1a 32 bit checksum,
    /// in big endian byte order.
    Fnv1a32,
    /// The checksum is (or should be set to) a modified FNV-1a 32 bit,
    /// in big endian byte order.
    Fnv1a32Modified,
}

impl ChecksumKind {
    /// empty string digest array of checksum kind.
    pub const fn empty_string_digest_array(&self) -> &'static [u8] {
        match self {
            ChecksumKind::Md5 => MD5_EMPTY_STRING_DIGEST_ARRAY,
            ChecksumKind::Sha1 => SHA1_EMPTY_STRING_DIGEST_ARRAY,
            ChecksumKind::Fnv1a32 => FNV1A32_EMPTY_STRING_DIGEST_ARRAY,
            ChecksumKind::Fnv1a32Modified => FNV1A32_MODIFIED_EMPTY_STRING_DIGEST_ARRAY,
        }
    }

    /// Checksum type prefixes used in serialized checksums.
    pub const fn prefix(&self) -> &'static str {
        match self {
            ChecksumKind::Md5 => "$md5 $",
            ChecksumKind::Sha1 => "$sha1$",
            ChecksumKind::Fnv1a32 => "$fnv1$",
            ChecksumKind::Fnv1a32Modified => "$fnvm$",
        }
    }
}

/// A generic checksum representation.
pub struct Checksum {
    /// The bytes of the checksum.
    digest: Vec<u8>,
    /// The type of the checksum.  This should never be changed by consumers
    /// of the APIs.
    kind: ChecksumKind,
}

/// The MD5 digest for the empty string.
#[rustfmt::skip]
const MD5_EMPTY_STRING_DIGEST_ARRAY: &[u8; 16] = &[
    0xd4, 0x1d, 0x8c, 0xd9, 0x8f, 0x00, 0xb2, 0x04,
    0xe9, 0x80, 0x09, 0x98, 0xec, 0xf8, 0x42, 0x7e
];

/// The SHA1 digest for the empty string.
#[rustfmt::skip]
const SHA1_EMPTY_STRING_DIGEST_ARRAY: &[u8; 20] = &[
  0xda, 0x39, 0xa3, 0xee, 0x5e, 0x6b, 0x4b, 0x0d, 0x32, 0x55,
  0xbf, 0xef, 0x95, 0x60, 0x18, 0x90, 0xaf, 0xd8, 0x07, 0x09
];

/// The FNV-1a digest for the empty string.
#[rustfmt::skip]
const FNV1A32_EMPTY_STRING_DIGEST_ARRAY: &[u8; 4] = &[
    0x81, 0x1c, 0x9d, 0xc5
];

/// The FNV-1a digest for the empty string.
#[rustfmt::skip]
const FNV1A32_MODIFIED_EMPTY_STRING_DIGEST_ARRAY: &[u8; 4] = &[
    0xcd, 0x6d, 0x9a, 0x85
];

impl Checksum {
    /// Get the bytes of the checksum.
    pub fn digest(&self) -> &[u8] {
        &self.digest
    }

    /// Get the kind of the checksum.
    pub fn kind(&self) -> &ChecksumKind {
        &self.kind
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_md5_empty_string() {
        let mut hasher = Md5::new();
        hasher.update(b"");
        let result = hasher.finalize();
        assert_eq!(result.as_slice(), MD5_EMPTY_STRING_DIGEST_ARRAY);
    }
}
