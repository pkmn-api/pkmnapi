/// Destination code
///
/// # Example
///
/// ```
/// use pkmnapi_db::header::*;
///
/// let code = DestinationCode::NON_JAPANESE;
/// ```
#[derive(Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum DestinationCode {
    JAPANESE,
    NON_JAPANESE,
}

impl From<u8> for DestinationCode {
    /// Convert u8 to DestinationCode
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::header::*;
    ///
    /// let code = DestinationCode::from(0x01);
    ///
    /// assert_eq!(code, DestinationCode::NON_JAPANESE);
    /// ```
    fn from(destination_code: u8) -> Self {
        match destination_code {
            0x01 => DestinationCode::NON_JAPANESE,
            _ => DestinationCode::JAPANESE,
        }
    }
}
