//! Pkmnapi database header module
//!
//! ```
//! use pkmnapi_db::header::*;
//! use std::fs;
//! # use std::fs::File;
//! # use std::io::prelude::*;
//! # use std::env;
//! # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
//!
//! let rom = fs::read(rom_path).unwrap();
//! let header = Header::from(&rom).unwrap();
//!
//! assert_eq!(header.title, "POKEMON RED");
//! ```

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
    cgb_flag: PkmnapiCGBFlag,
    new_licensee_code: PkmnapiNewLicenseeCode,
    sgb_flag: PkmnapiSGBFlag,
    cartridge_type: PkmnapiCartridgeType,
    rom_size: PkmnapiRomSize,
    ram_size: PkmnapiRamSize,
    destination_code: PkmnapiDestinationCode,
    old_licensee_code: PkmnapiOldLicenseeCode,
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
    /// # use std::fs::File;
    /// # use std::io::prelude::*;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let header = Header::from(&rom).unwrap();
    ///
    /// assert_eq!(header.title, "POKEMON RED");
    /// ```
    pub fn from(rom: &[u8]) -> Result<Header, String> {
        if rom.len() < 0x150 {
            return Err("Header too small".to_string());
        }

        let raw = rom[0x100..=0x14F].to_vec();
        let entry_point = raw[0x000..=0x003].to_vec();
        let logo = raw[0x004..=0x033].to_vec();
        let title = match str::from_utf8(&raw[0x034..=0x043]) {
            Ok(title) => title.trim_matches(char::from(0)).to_string(),
            Err(e) => return Err(e.to_string()),
        };
        let manufacturer_code = raw[0x03F..=0x042].to_vec();
        let cgb_flag = PkmnapiCGBFlag::from(raw[0x043]);
        let new_licensee_code = PkmnapiNewLicenseeCode::from(raw[0x044..=0x045].to_vec());
        let sgb_flag = PkmnapiSGBFlag::from(raw[0x046]);
        let cartridge_type = PkmnapiCartridgeType::from(raw[0x047]);
        let rom_size = PkmnapiRomSize::from(raw[0x048]);
        let ram_size = PkmnapiRamSize::from(raw[0x049]);
        let destination_code = PkmnapiDestinationCode::from(raw[0x04A]);
        let old_licensee_code = PkmnapiOldLicenseeCode::from(raw[0x04B]);
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
    /// # use std::fs::File;
    /// # use std::io::prelude::*;
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

/// Flag for CGB support
///
/// # Example
///
/// ```
/// use pkmnapi_db::header::*;
///
/// let flag = PkmnapiCGBFlag::NONE;
/// ```
#[derive(Debug, PartialEq)]
pub enum PkmnapiCGBFlag {
    NONE,
    SOME,
    ONLY,
}

impl From<u8> for PkmnapiCGBFlag {
    /// Convert u8 to PkmnapiCGBFlag
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::header::*;
    ///
    /// let flag = PkmnapiCGBFlag::from(0x80);
    ///
    /// assert_eq!(flag, PkmnapiCGBFlag::SOME);
    ///
    /// let flag = PkmnapiCGBFlag::from(0xC0);
    ///
    /// assert_eq!(flag, PkmnapiCGBFlag::ONLY);
    ///
    /// let flag = PkmnapiCGBFlag::from(0x00);
    ///
    /// assert_eq!(flag, PkmnapiCGBFlag::NONE);
    /// ```
    fn from(cgb_flag: u8) -> Self {
        match cgb_flag {
            0x80 => PkmnapiCGBFlag::SOME,
            0xC0 => PkmnapiCGBFlag::ONLY,
            _ => PkmnapiCGBFlag::NONE,
        }
    }
}

/// New licensee code
///
/// # Example
///
/// ```
/// use pkmnapi_db::header::*;
///
/// let code = PkmnapiNewLicenseeCode::NINTENDO_RND1;
/// ```
#[derive(Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum PkmnapiNewLicenseeCode {
    NONE,
    NINTENDO_RND1,
    CAPCOM,
    ELECTRONIC_ARTS,
    HUDSON_SOFT,
    B_AI,
    KSS,
    POW,
    PCM_COMPLETE,
    SAN_X,
    KEMCO_JAPAN,
    SETA,
    VIACOM,
    NINTENDO,
    BANDAI,
    OCEAN_ACCLAIM,
    KONAMI,
    HECTOR,
    TAITO,
    HUDSON,
    BANPRESTO,
    UBI_SOFT,
    ATLUS,
    MALIBU,
    ANGEL,
    BULLET_PROOF,
    IREM,
    ABSOLUTE,
    ACCLAIM,
    ACTIVISION,
    AMERICAN_SAMMY,
    KONAMI_2,
    HI_TECH_ENTERTAINMENT,
    LJN,
    MATCHBOX,
    MATTEL,
    MILTON_BRADLEY,
    TITUS,
    VIRGIN,
    LUCASARTS,
    OCEAN,
    ELECTRONIC_ARTS_2,
    INFOGRAMES,
    INTERPLAY,
    BRODERBUND,
    SCULPTURED,
    SCI,
    THQ,
    ACCOLADE,
    MISAWA,
    LOZC,
    TOKUMA_SHOTEN_INTERMEDIA,
    TSUKUDA_ORIGINAL,
    CHUNSOFT,
    VIDEO_SYSTEM,
    OCEAN_ACCLAIM_2,
    VARIE,
    YONEZAWA_S_PAL,
    KANEKO,
    PACK_IN_SOFT,
    KONAMI_YU_GI_OH,
}

impl From<Vec<u8>> for PkmnapiNewLicenseeCode {
    /// Convert Vec<u8> to PkmnapiNewLicenseeCode
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::header::*;
    ///
    /// let code = PkmnapiNewLicenseeCode::from(vec![0x30, 0x31]);
    ///
    /// assert_eq!(code, PkmnapiNewLicenseeCode::NINTENDO_RND1);
    /// ```
    fn from(new_licensee_code: Vec<u8>) -> Self {
        let code: String = new_licensee_code.iter().map(|x| *x as char).collect();

        match code.as_str() {
            "01" => PkmnapiNewLicenseeCode::NINTENDO_RND1,
            "08" => PkmnapiNewLicenseeCode::CAPCOM,
            "13" => PkmnapiNewLicenseeCode::ELECTRONIC_ARTS,
            "18" => PkmnapiNewLicenseeCode::HUDSON_SOFT,
            "19" => PkmnapiNewLicenseeCode::B_AI,
            "20" => PkmnapiNewLicenseeCode::KSS,
            "22" => PkmnapiNewLicenseeCode::POW,
            "24" => PkmnapiNewLicenseeCode::PCM_COMPLETE,
            "25" => PkmnapiNewLicenseeCode::SAN_X,
            "28" => PkmnapiNewLicenseeCode::KEMCO_JAPAN,
            "29" => PkmnapiNewLicenseeCode::SETA,
            "30" => PkmnapiNewLicenseeCode::VIACOM,
            "31" => PkmnapiNewLicenseeCode::NINTENDO,
            "32" => PkmnapiNewLicenseeCode::BANDAI,
            "33" => PkmnapiNewLicenseeCode::OCEAN_ACCLAIM,
            "34" => PkmnapiNewLicenseeCode::KONAMI,
            "35" => PkmnapiNewLicenseeCode::HECTOR,
            "37" => PkmnapiNewLicenseeCode::TAITO,
            "38" => PkmnapiNewLicenseeCode::HUDSON,
            "39" => PkmnapiNewLicenseeCode::BANPRESTO,
            "41" => PkmnapiNewLicenseeCode::UBI_SOFT,
            "42" => PkmnapiNewLicenseeCode::ATLUS,
            "44" => PkmnapiNewLicenseeCode::MALIBU,
            "46" => PkmnapiNewLicenseeCode::ANGEL,
            "47" => PkmnapiNewLicenseeCode::BULLET_PROOF,
            "49" => PkmnapiNewLicenseeCode::IREM,
            "50" => PkmnapiNewLicenseeCode::ABSOLUTE,
            "51" => PkmnapiNewLicenseeCode::ACCLAIM,
            "52" => PkmnapiNewLicenseeCode::ACTIVISION,
            "53" => PkmnapiNewLicenseeCode::AMERICAN_SAMMY,
            "54" => PkmnapiNewLicenseeCode::KONAMI_2,
            "55" => PkmnapiNewLicenseeCode::HI_TECH_ENTERTAINMENT,
            "56" => PkmnapiNewLicenseeCode::LJN,
            "57" => PkmnapiNewLicenseeCode::MATCHBOX,
            "58" => PkmnapiNewLicenseeCode::MATTEL,
            "59" => PkmnapiNewLicenseeCode::MILTON_BRADLEY,
            "60" => PkmnapiNewLicenseeCode::TITUS,
            "61" => PkmnapiNewLicenseeCode::VIRGIN,
            "64" => PkmnapiNewLicenseeCode::LUCASARTS,
            "67" => PkmnapiNewLicenseeCode::OCEAN,
            "69" => PkmnapiNewLicenseeCode::ELECTRONIC_ARTS_2,
            "70" => PkmnapiNewLicenseeCode::INFOGRAMES,
            "71" => PkmnapiNewLicenseeCode::INTERPLAY,
            "72" => PkmnapiNewLicenseeCode::BRODERBUND,
            "73" => PkmnapiNewLicenseeCode::SCULPTURED,
            "75" => PkmnapiNewLicenseeCode::SCI,
            "78" => PkmnapiNewLicenseeCode::THQ,
            "79" => PkmnapiNewLicenseeCode::ACCOLADE,
            "80" => PkmnapiNewLicenseeCode::MISAWA,
            "83" => PkmnapiNewLicenseeCode::LOZC,
            "86" => PkmnapiNewLicenseeCode::TOKUMA_SHOTEN_INTERMEDIA,
            "87" => PkmnapiNewLicenseeCode::TSUKUDA_ORIGINAL,
            "91" => PkmnapiNewLicenseeCode::CHUNSOFT,
            "92" => PkmnapiNewLicenseeCode::VIDEO_SYSTEM,
            "93" => PkmnapiNewLicenseeCode::OCEAN_ACCLAIM_2,
            "95" => PkmnapiNewLicenseeCode::VARIE,
            "96" => PkmnapiNewLicenseeCode::YONEZAWA_S_PAL,
            "97" => PkmnapiNewLicenseeCode::KANEKO,
            "99" => PkmnapiNewLicenseeCode::PACK_IN_SOFT,
            "A4" => PkmnapiNewLicenseeCode::KONAMI_YU_GI_OH,
            _ => PkmnapiNewLicenseeCode::NONE,
        }
    }
}

/// Flag for SGB support
///
/// # Example
///
/// ```
/// use pkmnapi_db::header::*;
///
/// let flag = PkmnapiSGBFlag::YES(0x03);
/// ```
#[derive(Debug, PartialEq)]
pub enum PkmnapiSGBFlag {
    NO(u8),
    YES(u8),
}

impl From<u8> for PkmnapiSGBFlag {
    /// Convert u8 to PkmnapiSGBFlag
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::header::*;
    ///
    /// let flag = PkmnapiSGBFlag::from(0x03);
    ///
    /// assert_eq!(flag, PkmnapiSGBFlag::YES(0x03));
    ///
    /// let flag = PkmnapiSGBFlag::from(0x00);
    ///
    /// assert_eq!(flag, PkmnapiSGBFlag::NO(0x00));
    /// ```
    fn from(sgb_flag: u8) -> Self {
        match sgb_flag {
            0x03 => PkmnapiSGBFlag::YES(sgb_flag),
            _ => PkmnapiSGBFlag::NO(0x00),
        }
    }
}

/// Cartridge type
///
/// # Example
///
/// ```
/// use pkmnapi_db::header::*;
///
/// let cartridge_type = PkmnapiCartridgeType::ROM_ONLY;
/// ```
#[derive(Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum PkmnapiCartridgeType {
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

impl From<u8> for PkmnapiCartridgeType {
    /// Convert u8 to PkmnapiCartridgeType
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::header::*;
    ///
    /// let cartridge_type = PkmnapiCartridgeType::from(0x01);
    ///
    /// assert_eq!(cartridge_type, PkmnapiCartridgeType::MBC1);
    /// ```
    fn from(cartridge_type: u8) -> Self {
        match cartridge_type {
            0x01 => PkmnapiCartridgeType::MBC1,
            0x02 => PkmnapiCartridgeType::MBC1_RAM,
            0x03 => PkmnapiCartridgeType::MBC1_RAM_BATTERY,
            0x05 => PkmnapiCartridgeType::MBC2,
            0x06 => PkmnapiCartridgeType::MBC2_BATTERY,
            0x08 => PkmnapiCartridgeType::ROM_RAM,
            0x09 => PkmnapiCartridgeType::ROM_RAM_BATTERY,
            0x0B => PkmnapiCartridgeType::MMM01,
            0x0C => PkmnapiCartridgeType::MMM01_RAM,
            0x0D => PkmnapiCartridgeType::MMM01_RAM_BATTERY,
            0x0F => PkmnapiCartridgeType::MBC3_TIMER_BATTERY,
            0x10 => PkmnapiCartridgeType::MBC3_TIMER_RAM_BATTERY,
            0x11 => PkmnapiCartridgeType::MBC3,
            0x12 => PkmnapiCartridgeType::MBC3_RAM,
            0x13 => PkmnapiCartridgeType::MBC3_RAM_BATTERY,
            0x19 => PkmnapiCartridgeType::MBC5,
            0x1A => PkmnapiCartridgeType::MBC5_RAM,
            0x1B => PkmnapiCartridgeType::MBC5_RAM_BATTERY,
            0x1C => PkmnapiCartridgeType::MBC5_RUMBLE,
            0x1D => PkmnapiCartridgeType::MBC5_RUMBLE_RAM,
            0x1E => PkmnapiCartridgeType::MBC5_RUMBLE_RAM_BATTERY,
            0x20 => PkmnapiCartridgeType::MBC6,
            0x22 => PkmnapiCartridgeType::MBC7_SENSOR_RUMBLE_RAM_BATTERY,
            0xFC => PkmnapiCartridgeType::POCKET_CAMERA,
            0xFD => PkmnapiCartridgeType::BANDAI_TAMA5,
            0xFE => PkmnapiCartridgeType::HUC3,
            0xFF => PkmnapiCartridgeType::HUC1_RAM_BATTERY,
            _ => PkmnapiCartridgeType::ROM_ONLY,
        }
    }
}

/// ROM size
///
/// # Example
///
/// ```
/// use pkmnapi_db::header::*;
///
/// let size = PkmnapiRomSize::MBYTE_1;
/// ```
#[derive(Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum PkmnapiRomSize {
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

impl From<u8> for PkmnapiRomSize {
    /// Convert u8 to PkmnapiRomSize
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::header::*;
    ///
    /// let size = PkmnapiRomSize::from(0x05);
    ///
    /// assert_eq!(size, PkmnapiRomSize::MBYTE_1);
    /// ```
    fn from(rom_size: u8) -> Self {
        match rom_size {
            0x01 => PkmnapiRomSize::KBYTE_64,
            0x02 => PkmnapiRomSize::KBYTE_128,
            0x03 => PkmnapiRomSize::KBYTE_256,
            0x04 => PkmnapiRomSize::KBYTE_512,
            0x05 => PkmnapiRomSize::MBYTE_1,
            0x06 => PkmnapiRomSize::MBYTE_2,
            0x07 => PkmnapiRomSize::MBYTE_4,
            0x08 => PkmnapiRomSize::MBYTE_8,
            0x52 => PkmnapiRomSize::MBYTE_1_1,
            0x53 => PkmnapiRomSize::MBYTE_1_2,
            0x54 => PkmnapiRomSize::MBYTE_1_5,
            _ => PkmnapiRomSize::KBYTE_32,
        }
    }
}

/// RAM size
///
/// # Example
///
/// ```
/// use pkmnapi_db::header::*;
///
/// let size = PkmnapiRamSize::KBYTES_32;
/// ```
#[derive(Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum PkmnapiRamSize {
    NONE,
    KBYTES_2,
    KBYTES_8,
    KBYTES_32,
    KBYTES_128,
    KBYTES_64,
}

impl From<u8> for PkmnapiRamSize {
    /// Convert u8 to PkmnapiRamSize
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::header::*;
    ///
    /// let size = PkmnapiRamSize::from(0x03);
    ///
    /// assert_eq!(size, PkmnapiRamSize::KBYTES_32);
    /// ```
    fn from(ram_size: u8) -> Self {
        match ram_size {
            0x01 => PkmnapiRamSize::KBYTES_2,
            0x02 => PkmnapiRamSize::KBYTES_8,
            0x03 => PkmnapiRamSize::KBYTES_32,
            0x04 => PkmnapiRamSize::KBYTES_128,
            0x05 => PkmnapiRamSize::KBYTES_64,
            _ => PkmnapiRamSize::NONE,
        }
    }
}

/// Destination code
///
/// # Example
///
/// ```
/// use pkmnapi_db::header::*;
///
/// let code = PkmnapiDestinationCode::NON_JAPANESE;
/// ```
#[derive(Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum PkmnapiDestinationCode {
    JAPANESE,
    NON_JAPANESE,
}

impl From<u8> for PkmnapiDestinationCode {
    /// Convert u8 to PkmnapiDestinationCode
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::header::*;
    ///
    /// let code = PkmnapiDestinationCode::from(0x01);
    ///
    /// assert_eq!(code, PkmnapiDestinationCode::NON_JAPANESE);
    /// ```
    fn from(destination_code: u8) -> Self {
        match destination_code {
            0x01 => PkmnapiDestinationCode::NON_JAPANESE,
            _ => PkmnapiDestinationCode::JAPANESE,
        }
    }
}

/// Old licensee code
///
/// # Example
///
/// ```
/// use pkmnapi_db::header::*;
///
/// let code = PkmnapiOldLicenseeCode::NINTENDO;
/// ```
#[derive(Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum PkmnapiOldLicenseeCode {
    NONE,
    NINTENDO,
    CAPCOM,
    HOT_B,
    JALECO,
    COCONUTS,
    ELITE_SYSTEMS,
    ELECTRONIC_ARTS,
    HUDSONSOFT,
    ITC_ENTERTAINMENT,
    YANOMAN,
    CLARY,
    VIRGIN,
    PCM_COMPLETE,
    SAN_X,
    KOTOBUKI_SYSTEMS,
    SETA,
    INFOGRAMES,
    NINTENDO_2,
    BANDAI,
    GBC,
    KONAMI,
    HECTOR,
    CAPCOM_2,
    BANPRESTO,
    ENTERTAINMENT_I,
    GREMLIN,
    UBI_SOFT,
    ATLUS,
    MALIBU,
    ANGEL,
    SPECTRUM_HOLOBY,
    IREM,
    VIRGIN_2,
    MALIBU_2,
    US_GOLD,
    ABSOLUTE,
    ACCLAIM,
    ACTIVISION,
    AMERICAN_SAMMY,
    GAMETEK,
    PARK_PLACE,
    LJN,
    MATCHBOX,
    MILTON_BRADLEY,
    MINDSCAPE,
    ROMSTAR,
    NAXAT_SOFT,
    TRADEWEST,
    TITUS,
    VIRGIN_3,
    OCEAN,
    ELECTRONIC_ARTS_2,
    ELITE_SYSTEMS_2,
    ELECTRO_BRAIN,
    INFOGRAMES_3,
    INTERPLAY,
    BRODERBUND,
    SCULPTERED_SOFT,
    THE_SALES_CURVE,
    THQ,
    ACCOLADE,
    TRIFFIX_ENTERTAINMENT,
    MICROPROSE,
    KEMCO,
    MISAWA_ENTERTAINMENT,
    LOZC,
    TOKUMA_SHOTEN_I,
    BULLET_PROOF_SOFTWARE,
    VIC_TOKAI,
    APE,
    I_MAX,
    CHUN_SOFT,
    VIDEO_SYSTEM,
    TSUBURAVA,
    VARIE,
    YONEZAWA_S_PAL,
    KANEKO,
    ARC,
    NIHON_BUSSAN,
    TECMO,
    IMAGINEER,
    BANPRESTO_2,
    NOVA,
    HORI_ELECTRIC,
    BANDAI_2,
    KONAMI_2,
    KAWADA,
    TAKARA,
    TECHNOS_JAPAN,
    BRODERBUND_2,
    TOEI_ANIMATION,
    TOHO,
    NAMCO,
    ACCLAIM_2,
    ASCII_OR_NEXOFT,
    BANDAI_3,
    ENIX,
    HAL,
    SNK,
    PONY_CANYON,
    CULTURE_BRAIN_O,
    SUNSOFT,
    SONY_IMAGESOFT,
    SAMMY,
    TAITO,
    KEMCO_2,
    SQUARESOFT,
    TOKUMA_SHOTEN_I_2,
    DATA_EAST,
    TONKIN_HOUSE,
    KOEI,
    UFL,
    ULTRA,
    VAP,
    USE,
    MELDAC,
    PONY_CANYON_OR,
    ANGEL_2,
    TAITO_2,
    SOFEL,
    QUEST,
    SIGMA_ENTERPRISES,
    ASK_KODANSHA,
    NAXAT_SOFT_2,
    COPYA_SYSTEMS,
    BANPRESTO_3,
    TOMY,
    LJN_2,
    NCS,
    HUMAN,
    ALTRON,
    JALECO_2,
    TOWACHIKI,
    UUTAKA,
    VARIE_2,
    EPOCH,
    ATHENA,
    ASMIK,
    NATSUME,
    KING_RECORDS,
    ATLUS_2,
    EPIC_SONY_RECORDS,
    IGS,
    A_WAVE,
    EXTREME_ENTERTAINMENT,
    LJN_3,
}

impl From<u8> for PkmnapiOldLicenseeCode {
    /// Convert u8 to PkmnapiOldLicenseeCode
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::header::*;
    ///
    /// let code = PkmnapiOldLicenseeCode::from(0x01);
    ///
    /// assert_eq!(code, PkmnapiOldLicenseeCode::NINTENDO);
    /// ```
    fn from(old_licensee_code: u8) -> Self {
        match old_licensee_code {
            0x01 => PkmnapiOldLicenseeCode::NINTENDO,
            0x08 => PkmnapiOldLicenseeCode::CAPCOM,
            0x09 => PkmnapiOldLicenseeCode::HOT_B,
            0x0A => PkmnapiOldLicenseeCode::JALECO,
            0x0B => PkmnapiOldLicenseeCode::COCONUTS,
            0x0C => PkmnapiOldLicenseeCode::ELITE_SYSTEMS,
            0x13 => PkmnapiOldLicenseeCode::ELECTRONIC_ARTS,
            0x18 => PkmnapiOldLicenseeCode::HUDSONSOFT,
            0x19 => PkmnapiOldLicenseeCode::ITC_ENTERTAINMENT,
            0x1A => PkmnapiOldLicenseeCode::YANOMAN,
            0x1D => PkmnapiOldLicenseeCode::CLARY,
            0x1F => PkmnapiOldLicenseeCode::VIRGIN,
            0x24 => PkmnapiOldLicenseeCode::PCM_COMPLETE,
            0x25 => PkmnapiOldLicenseeCode::SAN_X,
            0x28 => PkmnapiOldLicenseeCode::KOTOBUKI_SYSTEMS,
            0x29 => PkmnapiOldLicenseeCode::SETA,
            0x30 => PkmnapiOldLicenseeCode::INFOGRAMES,
            0x31 => PkmnapiOldLicenseeCode::NINTENDO_2,
            0x32 => PkmnapiOldLicenseeCode::BANDAI,
            0x33 => PkmnapiOldLicenseeCode::GBC,
            0x34 => PkmnapiOldLicenseeCode::KONAMI,
            0x35 => PkmnapiOldLicenseeCode::HECTOR,
            0x38 => PkmnapiOldLicenseeCode::CAPCOM_2,
            0x39 => PkmnapiOldLicenseeCode::BANPRESTO,
            0x3C => PkmnapiOldLicenseeCode::ENTERTAINMENT_I,
            0x3E => PkmnapiOldLicenseeCode::GREMLIN,
            0x41 => PkmnapiOldLicenseeCode::UBI_SOFT,
            0x42 => PkmnapiOldLicenseeCode::ATLUS,
            0x44 => PkmnapiOldLicenseeCode::MALIBU,
            0x46 => PkmnapiOldLicenseeCode::ANGEL,
            0x47 => PkmnapiOldLicenseeCode::SPECTRUM_HOLOBY,
            0x49 => PkmnapiOldLicenseeCode::IREM,
            0x4A => PkmnapiOldLicenseeCode::VIRGIN_2,
            0x4D => PkmnapiOldLicenseeCode::MALIBU_2,
            0x4F => PkmnapiOldLicenseeCode::US_GOLD,
            0x50 => PkmnapiOldLicenseeCode::ABSOLUTE,
            0x51 => PkmnapiOldLicenseeCode::ACCLAIM,
            0x52 => PkmnapiOldLicenseeCode::ACTIVISION,
            0x53 => PkmnapiOldLicenseeCode::AMERICAN_SAMMY,
            0x54 => PkmnapiOldLicenseeCode::GAMETEK,
            0x55 => PkmnapiOldLicenseeCode::PARK_PLACE,
            0x56 => PkmnapiOldLicenseeCode::LJN,
            0x57 => PkmnapiOldLicenseeCode::MATCHBOX,
            0x59 => PkmnapiOldLicenseeCode::MILTON_BRADLEY,
            0x5A => PkmnapiOldLicenseeCode::MINDSCAPE,
            0x5B => PkmnapiOldLicenseeCode::ROMSTAR,
            0x5C => PkmnapiOldLicenseeCode::NAXAT_SOFT,
            0x5D => PkmnapiOldLicenseeCode::TRADEWEST,
            0x60 => PkmnapiOldLicenseeCode::TITUS,
            0x61 => PkmnapiOldLicenseeCode::VIRGIN_3,
            0x67 => PkmnapiOldLicenseeCode::OCEAN,
            0x69 => PkmnapiOldLicenseeCode::ELECTRONIC_ARTS_2,
            0x6E => PkmnapiOldLicenseeCode::ELITE_SYSTEMS_2,
            0x6F => PkmnapiOldLicenseeCode::ELECTRO_BRAIN,
            0x70 => PkmnapiOldLicenseeCode::INFOGRAMES_3,
            0x71 => PkmnapiOldLicenseeCode::INTERPLAY,
            0x72 => PkmnapiOldLicenseeCode::BRODERBUND,
            0x73 => PkmnapiOldLicenseeCode::SCULPTERED_SOFT,
            0x75 => PkmnapiOldLicenseeCode::THE_SALES_CURVE,
            0x78 => PkmnapiOldLicenseeCode::THQ,
            0x79 => PkmnapiOldLicenseeCode::ACCOLADE,
            0x7A => PkmnapiOldLicenseeCode::TRIFFIX_ENTERTAINMENT,
            0x7C => PkmnapiOldLicenseeCode::MICROPROSE,
            0x7F => PkmnapiOldLicenseeCode::KEMCO,
            0x80 => PkmnapiOldLicenseeCode::MISAWA_ENTERTAINMENT,
            0x83 => PkmnapiOldLicenseeCode::LOZC,
            0x86 => PkmnapiOldLicenseeCode::TOKUMA_SHOTEN_I,
            0x8B => PkmnapiOldLicenseeCode::BULLET_PROOF_SOFTWARE,
            0x8C => PkmnapiOldLicenseeCode::VIC_TOKAI,
            0x8E => PkmnapiOldLicenseeCode::APE,
            0x8F => PkmnapiOldLicenseeCode::I_MAX,
            0x91 => PkmnapiOldLicenseeCode::CHUN_SOFT,
            0x92 => PkmnapiOldLicenseeCode::VIDEO_SYSTEM,
            0x93 => PkmnapiOldLicenseeCode::TSUBURAVA,
            0x95 => PkmnapiOldLicenseeCode::VARIE,
            0x96 => PkmnapiOldLicenseeCode::YONEZAWA_S_PAL,
            0x97 => PkmnapiOldLicenseeCode::KANEKO,
            0x99 => PkmnapiOldLicenseeCode::ARC,
            0x9A => PkmnapiOldLicenseeCode::NIHON_BUSSAN,
            0x9B => PkmnapiOldLicenseeCode::TECMO,
            0x9C => PkmnapiOldLicenseeCode::IMAGINEER,
            0x9D => PkmnapiOldLicenseeCode::BANPRESTO_2,
            0x9F => PkmnapiOldLicenseeCode::NOVA,
            0xA1 => PkmnapiOldLicenseeCode::HORI_ELECTRIC,
            0xA2 => PkmnapiOldLicenseeCode::BANDAI_2,
            0xA4 => PkmnapiOldLicenseeCode::KONAMI_2,
            0xA6 => PkmnapiOldLicenseeCode::KAWADA,
            0xA7 => PkmnapiOldLicenseeCode::TAKARA,
            0xA9 => PkmnapiOldLicenseeCode::TECHNOS_JAPAN,
            0xAA => PkmnapiOldLicenseeCode::BRODERBUND_2,
            0xAC => PkmnapiOldLicenseeCode::TOEI_ANIMATION,
            0xAD => PkmnapiOldLicenseeCode::TOHO,
            0xAF => PkmnapiOldLicenseeCode::NAMCO,
            0xB0 => PkmnapiOldLicenseeCode::ACCLAIM_2,
            0xB1 => PkmnapiOldLicenseeCode::ASCII_OR_NEXOFT,
            0xB2 => PkmnapiOldLicenseeCode::BANDAI_3,
            0xB4 => PkmnapiOldLicenseeCode::ENIX,
            0xB6 => PkmnapiOldLicenseeCode::HAL,
            0xB7 => PkmnapiOldLicenseeCode::SNK,
            0xB9 => PkmnapiOldLicenseeCode::PONY_CANYON,
            0xBA => PkmnapiOldLicenseeCode::CULTURE_BRAIN_O,
            0xBB => PkmnapiOldLicenseeCode::SUNSOFT,
            0xBD => PkmnapiOldLicenseeCode::SONY_IMAGESOFT,
            0xBF => PkmnapiOldLicenseeCode::SAMMY,
            0xC0 => PkmnapiOldLicenseeCode::TAITO,
            0xC2 => PkmnapiOldLicenseeCode::KEMCO_2,
            0xC3 => PkmnapiOldLicenseeCode::SQUARESOFT,
            0xC4 => PkmnapiOldLicenseeCode::TOKUMA_SHOTEN_I_2,
            0xC5 => PkmnapiOldLicenseeCode::DATA_EAST,
            0xC6 => PkmnapiOldLicenseeCode::TONKIN_HOUSE,
            0xC8 => PkmnapiOldLicenseeCode::KOEI,
            0xC9 => PkmnapiOldLicenseeCode::UFL,
            0xCA => PkmnapiOldLicenseeCode::ULTRA,
            0xCB => PkmnapiOldLicenseeCode::VAP,
            0xCC => PkmnapiOldLicenseeCode::USE,
            0xCD => PkmnapiOldLicenseeCode::MELDAC,
            0xCE => PkmnapiOldLicenseeCode::PONY_CANYON_OR,
            0xCF => PkmnapiOldLicenseeCode::ANGEL_2,
            0xD0 => PkmnapiOldLicenseeCode::TAITO_2,
            0xD1 => PkmnapiOldLicenseeCode::SOFEL,
            0xD2 => PkmnapiOldLicenseeCode::QUEST,
            0xD3 => PkmnapiOldLicenseeCode::SIGMA_ENTERPRISES,
            0xD4 => PkmnapiOldLicenseeCode::ASK_KODANSHA,
            0xD6 => PkmnapiOldLicenseeCode::NAXAT_SOFT_2,
            0xD7 => PkmnapiOldLicenseeCode::COPYA_SYSTEMS,
            0xD9 => PkmnapiOldLicenseeCode::BANPRESTO_3,
            0xDA => PkmnapiOldLicenseeCode::TOMY,
            0xDB => PkmnapiOldLicenseeCode::LJN_2,
            0xDD => PkmnapiOldLicenseeCode::NCS,
            0xDE => PkmnapiOldLicenseeCode::HUMAN,
            0xDF => PkmnapiOldLicenseeCode::ALTRON,
            0xE0 => PkmnapiOldLicenseeCode::JALECO_2,
            0xE1 => PkmnapiOldLicenseeCode::TOWACHIKI,
            0xE2 => PkmnapiOldLicenseeCode::UUTAKA,
            0xE3 => PkmnapiOldLicenseeCode::VARIE_2,
            0xE5 => PkmnapiOldLicenseeCode::EPOCH,
            0xE7 => PkmnapiOldLicenseeCode::ATHENA,
            0xE8 => PkmnapiOldLicenseeCode::ASMIK,
            0xE9 => PkmnapiOldLicenseeCode::NATSUME,
            0xEA => PkmnapiOldLicenseeCode::KING_RECORDS,
            0xEB => PkmnapiOldLicenseeCode::ATLUS_2,
            0xEC => PkmnapiOldLicenseeCode::EPIC_SONY_RECORDS,
            0xEE => PkmnapiOldLicenseeCode::IGS,
            0xF0 => PkmnapiOldLicenseeCode::A_WAVE,
            0xF3 => PkmnapiOldLicenseeCode::EXTREME_ENTERTAINMENT,
            0xFF => PkmnapiOldLicenseeCode::LJN_3,
            _ => PkmnapiOldLicenseeCode::NONE,
        }
    }
}
