//! Pkmnapi header module
//!
//! ```
//! use pkmnapi_db::header::*;
//! use std::fs;
//! # use std::env;
//! # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
//!
//! let rom = fs::read(rom_path).unwrap();
//! let header = Header::from(&rom).unwrap();
//!
//! // RED
//! # #[cfg(feature = "PKMN_RED")]
//! assert_eq!(header.title, "POKEMON RED");
//!
//! // BLUE
//! # #[cfg(not(feature = "PKMN_RED"))]
//! assert_eq!(header.title, "POKEMON BLUE");
//! ```

mod cartridge_type;
mod cbg_flag;
mod destination_code;
mod new_licensee_code;
mod old_licensee_code;
mod ram_size;
mod rom_size;
mod sgb_flag;

pub use cartridge_type::*;
pub use cbg_flag::*;
pub use destination_code::*;
pub use new_licensee_code::*;
pub use old_licensee_code::*;
pub use ram_size::*;
pub use rom_size::*;
pub use sgb_flag::*;

use crate::error::{self, Result};
use byteorder::{BigEndian, ReadBytesExt};
use std::io::Cursor;
use std::num::Wrapping;
use std::str;

/// Representation of a ROM header
#[derive(Debug, PartialEq)]
pub struct Header {
    raw: Vec<u8>,
    entry_point: Vec<u8>,
    logo: Vec<u8>,
    pub title: String,
    manufacturer_code: Vec<u8>,
    cgb_flag: CGBFlag,
    new_licensee_code: NewLicenseeCode,
    sgb_flag: SGBFlag,
    cartridge_type: CartridgeType,
    rom_size: RomSize,
    ram_size: RamSize,
    destination_code: DestinationCode,
    old_licensee_code: OldLicenseeCode,
    mask_rom_version_number: u8,
    header_checksum: u8,
    pub global_checksum: u16,
}

impl Header {
    /// Create header from an array of bytes
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::header::*;
    /// use std::fs;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let header = Header::from(&rom).unwrap();
    ///
    /// // RED
    /// # #[cfg(feature = "PKMN_RED")]
    /// assert_eq!(header.title, "POKEMON RED");
    ///
    /// // BLUE
    /// # #[cfg(not(feature = "PKMN_RED"))]
    /// assert_eq!(header.title, "POKEMON BLUE");
    /// ```
    pub fn from(rom: &[u8]) -> Result<Header> {
        if rom.len() < 0x150 {
            return Err(error::Error::HeaderTooSmall);
        }

        let raw = rom[0x100..=0x14F].to_vec();
        let entry_point = raw[0x000..=0x003].to_vec();
        let logo = raw[0x004..=0x033].to_vec();
        let title = match str::from_utf8(&raw[0x034..=0x043]) {
            Ok(title) => title.trim_matches(char::from(0)).to_string(),
            Err(e) => return Err(error::Error::HeaderParseError(e.to_string())),
        };
        let manufacturer_code = raw[0x03F..=0x042].to_vec();
        let cgb_flag = CGBFlag::from(raw[0x043]);
        let new_licensee_code = NewLicenseeCode::from(raw[0x044..=0x045].to_vec());
        let sgb_flag = SGBFlag::from(raw[0x046]);
        let cartridge_type = CartridgeType::from(raw[0x047]);
        let rom_size = RomSize::from(raw[0x048]);
        let ram_size = RamSize::from(raw[0x049]);
        let destination_code = DestinationCode::from(raw[0x04A]);
        let old_licensee_code = OldLicenseeCode::from(raw[0x04B]);
        let mask_rom_version_number = raw[0x04C];
        let header_checksum = raw[0x04D];
        let global_checksum = {
            let mut cursor = Cursor::new(&raw[0x04E..=0x04F]);

            cursor.read_u16::<BigEndian>().unwrap_or(0)
        };

        Ok(Header {
            raw,
            entry_point,
            logo,
            title,
            manufacturer_code,
            cgb_flag,
            new_licensee_code,
            sgb_flag,
            cartridge_type,
            rom_size,
            ram_size,
            destination_code,
            old_licensee_code,
            mask_rom_version_number,
            header_checksum,
            global_checksum,
        })
    }

    /// Verify header checksum
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::header::*;
    /// use std::fs;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let header = Header::from(&rom).unwrap();
    ///
    /// assert_eq!(header.verify_checksum(), true);
    /// ```
    pub fn verify_checksum(&self) -> bool {
        let checksum = self.raw[0x034..=0x04C]
            .iter()
            .fold(Wrapping(0u8), |acc, x| acc - Wrapping(*x) - Wrapping(1u8));

        checksum.0 == self.header_checksum
    }
}
