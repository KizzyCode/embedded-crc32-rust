use embedded_crc32c::crc32c;
use std::iter;

/// A CRC32C test vector
struct TestVector {
    /// The input data
    data: Vec<u8>,
    /// The expected CRC32C checksum
    expected: u32,
}
impl TestVector {
    /// Creates a new test vector
    #[must_use]
    pub fn new<I, IT>(bytes: I, expected: u32) -> Self
    where
        I: IntoIterator<Item = IT>,
        IT: Into<u8>,
    {
        let data: Vec<u8> = bytes.into_iter().map(|element| element.into()).collect();
        Self { data: data.into(), expected }
    }

    /// Asserts that the given data produces the expected checksum
    pub fn assert(self) {
        let crc = crc32c(&self.data);
        assert!(crc == self.expected, "invalid CRC32C checksum (expected 0x{:08X}, got 0x{crc:08X})", self.expected);
    }
}

#[test]
fn test_vectors() {
    // Test well-defined test vectors
    TestVector::new(iter::repeat(0x00).take(32), 0x8A9136AA).assert();
    TestVector::new(iter::repeat(0xFF).take(32), 0x62A8AB43).assert();
    TestVector::new(0x00..=0x1F, 0x46DD794E).assert();
    TestVector::new((0x00..=0x1F).rev(), 0x113FDB5C).assert();
}
