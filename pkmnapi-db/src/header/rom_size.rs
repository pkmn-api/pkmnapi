/// ROM size
///
/// # Example
///
/// ```
/// use pkmnapi_db::header::*;
///
/// let size = RomSize::MBYTE_1;
/// ```
#[derive(Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum RomSize {
    KBYTE_32,
    KBYTE_64,
    KBYTE_128,
    KBYTE_256,
    KBYTE_512,
    MBYTE_1,
    MBYTE_2,
    MBYTE_4,
    MBYTE_8,
    MBYTE_1_1,
    MBYTE_1_2,
    MBYTE_1_5,
}

impl From<u8> for RomSize {
    /// Convert u8 to RomSize
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::header::*;
    ///
    /// let size = RomSize::from(0x05);
    ///
    /// assert_eq!(size, RomSize::MBYTE_1);
    /// ```
    fn from(rom_size: u8) -> Self {
        match rom_size {
            0x01 => RomSize::KBYTE_64,
            0x02 => RomSize::KBYTE_128,
            0x03 => RomSize::KBYTE_256,
            0x04 => RomSize::KBYTE_512,
            0x05 => RomSize::MBYTE_1,
            0x06 => RomSize::MBYTE_2,
            0x07 => RomSize::MBYTE_4,
            0x08 => RomSize::MBYTE_8,
            0x52 => RomSize::MBYTE_1_1,
            0x53 => RomSize::MBYTE_1_2,
            0x54 => RomSize::MBYTE_1_5,
            _ => RomSize::KBYTE_32,
        }
    }
}
