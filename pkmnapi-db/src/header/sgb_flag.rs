/// Flag for SGB support
///
/// # Example
///
/// ```
/// use pkmnapi_db::header::*;
///
/// let flag = SGBFlag::YES(0x03);
/// ```
#[derive(Debug, PartialEq)]
pub enum SGBFlag {
    NO(u8),
    YES(u8),
}

impl From<u8> for SGBFlag {
    /// Convert u8 to SGBFlag
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::header::*;
    ///
    /// let flag = SGBFlag::from(0x03);
    ///
    /// assert_eq!(flag, SGBFlag::YES(0x03));
    ///
    /// let flag = SGBFlag::from(0x00);
    ///
    /// assert_eq!(flag, SGBFlag::NO(0x00));
    /// ```
    fn from(sgb_flag: u8) -> Self {
        match sgb_flag {
            0x03 => SGBFlag::YES(sgb_flag),
            _ => SGBFlag::NO(0x00),
        }
    }
}
