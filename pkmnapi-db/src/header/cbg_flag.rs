/// Flag for CGB support
///
/// # Example
///
/// ```
/// use pkmnapi_db::header::*;
///
/// let flag = CGBFlag::NONE;
/// ```
#[derive(Debug, PartialEq)]
pub enum CGBFlag {
    NONE,
    SOME,
    ONLY,
}

impl From<u8> for CGBFlag {
    /// Convert u8 to CGBFlag
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::header::*;
    ///
    /// let flag = CGBFlag::from(0x80);
    ///
    /// assert_eq!(flag, CGBFlag::SOME);
    ///
    /// let flag = CGBFlag::from(0xC0);
    ///
    /// assert_eq!(flag, CGBFlag::ONLY);
    ///
    /// let flag = CGBFlag::from(0x00);
    ///
    /// assert_eq!(flag, CGBFlag::NONE);
    /// ```
    fn from(cgb_flag: u8) -> Self {
        match cgb_flag {
            0x80 => CGBFlag::SOME,
            0xC0 => CGBFlag::ONLY,
            _ => CGBFlag::NONE,
        }
    }
}
