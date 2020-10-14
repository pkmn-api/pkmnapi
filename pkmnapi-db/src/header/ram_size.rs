/// RAM size
///
/// # Example
///
/// ```
/// use pkmnapi_db::header::*;
///
/// let size = RamSize::KBYTES_32;
/// ```
#[derive(Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum RamSize {
    NONE,
    KBYTES_2,
    KBYTES_8,
    KBYTES_32,
    KBYTES_128,
    KBYTES_64,
}

impl From<u8> for RamSize {
    /// Convert u8 to RamSize
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::header::*;
    ///
    /// let size = RamSize::from(0x03);
    ///
    /// assert_eq!(size, RamSize::KBYTES_32);
    /// ```
    fn from(ram_size: u8) -> Self {
        match ram_size {
            0x01 => RamSize::KBYTES_2,
            0x02 => RamSize::KBYTES_8,
            0x03 => RamSize::KBYTES_32,
            0x04 => RamSize::KBYTES_128,
            0x05 => RamSize::KBYTES_64,
            _ => RamSize::NONE,
        }
    }
}
