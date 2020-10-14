/// Cartridge type
///
/// # Example
///
/// ```
/// use pkmnapi_db::header::*;
///
/// let cartridge_type = CartridgeType::ROM_ONLY;
/// ```
#[derive(Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum CartridgeType {
    ROM_ONLY,
    MBC1,
    MBC1_RAM,
    MBC1_RAM_BATTERY,
    MBC2,
    MBC2_BATTERY,
    ROM_RAM,
    ROM_RAM_BATTERY,
    MMM01,
    MMM01_RAM,
    MMM01_RAM_BATTERY,
    MBC3_TIMER_BATTERY,
    MBC3_TIMER_RAM_BATTERY,
    MBC3,
    MBC3_RAM,
    MBC3_RAM_BATTERY,
    MBC5,
    MBC5_RAM,
    MBC5_RAM_BATTERY,
    MBC5_RUMBLE,
    MBC5_RUMBLE_RAM,
    MBC5_RUMBLE_RAM_BATTERY,
    MBC6,
    MBC7_SENSOR_RUMBLE_RAM_BATTERY,
    POCKET_CAMERA,
    BANDAI_TAMA5,
    HUC3,
    HUC1_RAM_BATTERY,
}

impl From<u8> for CartridgeType {
    /// Convert u8 to CartridgeType
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::header::*;
    ///
    /// let cartridge_type = CartridgeType::from(0x01);
    ///
    /// assert_eq!(cartridge_type, CartridgeType::MBC1);
    /// ```
    fn from(cartridge_type: u8) -> Self {
        match cartridge_type {
            0x01 => CartridgeType::MBC1,
            0x02 => CartridgeType::MBC1_RAM,
            0x03 => CartridgeType::MBC1_RAM_BATTERY,
            0x05 => CartridgeType::MBC2,
            0x06 => CartridgeType::MBC2_BATTERY,
            0x08 => CartridgeType::ROM_RAM,
            0x09 => CartridgeType::ROM_RAM_BATTERY,
            0x0B => CartridgeType::MMM01,
            0x0C => CartridgeType::MMM01_RAM,
            0x0D => CartridgeType::MMM01_RAM_BATTERY,
            0x0F => CartridgeType::MBC3_TIMER_BATTERY,
            0x10 => CartridgeType::MBC3_TIMER_RAM_BATTERY,
            0x11 => CartridgeType::MBC3,
            0x12 => CartridgeType::MBC3_RAM,
            0x13 => CartridgeType::MBC3_RAM_BATTERY,
            0x19 => CartridgeType::MBC5,
            0x1A => CartridgeType::MBC5_RAM,
            0x1B => CartridgeType::MBC5_RAM_BATTERY,
            0x1C => CartridgeType::MBC5_RUMBLE,
            0x1D => CartridgeType::MBC5_RUMBLE_RAM,
            0x1E => CartridgeType::MBC5_RUMBLE_RAM_BATTERY,
            0x20 => CartridgeType::MBC6,
            0x22 => CartridgeType::MBC7_SENSOR_RUMBLE_RAM_BATTERY,
            0xFC => CartridgeType::POCKET_CAMERA,
            0xFD => CartridgeType::BANDAI_TAMA5,
            0xFE => CartridgeType::HUC3,
            0xFF => CartridgeType::HUC1_RAM_BATTERY,
            _ => CartridgeType::ROM_ONLY,
        }
    }
}
