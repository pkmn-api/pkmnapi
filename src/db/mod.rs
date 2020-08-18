pub mod header;
pub mod patch;
pub mod string;

use byteorder::{LittleEndian, ReadBytesExt};
use header::*;
use patch::*;
use std::cmp::{self, Ordering};
use std::fmt;
use std::io::{Cursor, Read};
use std::num::Wrapping;
use std::ops::{Add, Sub};
use string::*;

const ROM_PAGE: usize = 0x2000;
const POKEMON_INTERNAL_MAX: usize = 190;

/// Pkmnapi database
///
/// # Example
///
/// ```
/// use pkmnapi::db::*;
/// use std::fs;
/// # use std::fs::File;
/// # use std::io::prelude::*;
/// # let mut file = File::create("rom.db").unwrap();
/// # file.write_all(&vec![0x00; 0x150]).unwrap();
///
/// let rom = fs::read("rom.db").unwrap();
/// let db = PkmnapiDB::new(&rom).unwrap();
/// # fs::remove_file("rom.db");
/// ```
#[derive(Debug)]
pub struct PkmnapiDB {
    pub rom: Vec<u8>,
    pub hash: String,
    pub header: PkmnapiDBHeader,
}

impl PkmnapiDB {
    /// Create new database
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi::db::*;
    /// use std::fs;
    /// # use std::fs::File;
    /// # use std::io::prelude::*;
    /// # let mut file = File::create("rom.db").unwrap();
    /// # file.write_all(&vec![0x00; 0x150]).unwrap();
    ///
    /// let rom = fs::read("rom.db").unwrap();
    /// let db = PkmnapiDB::new(&rom).unwrap();
    /// # fs::remove_file("rom.db");
    /// ```
    pub fn new(rom: &Vec<u8>) -> Result<PkmnapiDB, String> {
        let hash = format!("{:x}", md5::compute(&rom));
        let header = PkmnapiDBHeader::from(&rom)?;

        Ok(PkmnapiDB {
            rom: rom[..].to_vec(),
            hash,
            header,
        })
    }

    /// Verify global checksum
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi::db::*;
    /// use std::fs;
    /// # use std::fs::File;
    /// # use std::io::prelude::*;
    /// # let mut file = File::create("rom.db").unwrap();
    /// # file.write_all(&vec![0x00; 0x150]).unwrap();
    ///
    /// let rom = fs::read("rom.db").unwrap();
    /// let db = PkmnapiDB::new(&rom).unwrap();
    ///
    /// assert_eq!(db.verify_checksum(), true);
    /// # fs::remove_file("rom.db");
    /// ```
    pub fn verify_checksum(&self) -> bool {
        let rom = [&self.rom[..0x014E], &self.rom[0x0150..]].concat();
        let checksum = rom
            .iter()
            .fold(Wrapping(0u16), |acc, x| acc + Wrapping(*x as u16));

        checksum.0 == self.header.global_checksum
    }

    /// Generate global checksum
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi::db::patch::*;
    /// use pkmnapi::db::*;
    /// use std::fs;
    /// # use std::fs::File;
    /// # use std::io::prelude::*;
    /// # let mut file = File::create("rom.db").unwrap();
    /// # file.write_all(&vec![0x00; 0x150]).unwrap();
    ///
    /// let rom = fs::read("rom.db").unwrap();
    /// let mut db = PkmnapiDB::new(&rom).unwrap();
    ///
    /// db.rom[0x100] = 0x42;
    ///
    /// assert_eq!(db.verify_checksum(), false);
    ///
    /// let patch = db.generate_checksum();
    ///
    /// assert_eq!(
    ///     patch,
    ///     PkmnapiDBPatch {
    ///         offset: 0x014E,
    ///         length: 0x02,
    ///         data: vec![0x00, 0x42]
    ///     }
    /// );
    ///
    /// assert_eq!(db.verify_checksum(), true);
    /// # fs::remove_file("rom.db");
    /// ```
    pub fn generate_checksum(&mut self) -> PkmnapiDBPatch {
        let rom = [&self.rom[..0x014E], &self.rom[0x0150..]].concat();
        let checksum = rom
            .iter()
            .fold(Wrapping(0u16), |acc, x| acc + Wrapping(*x as u16));

        self.header.global_checksum = checksum.0;

        let checksum = checksum.0.to_be_bytes().to_vec();

        PkmnapiDBPatch::new(0x014E, checksum)
    }

    /// Verify ROM hash
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi::db::*;
    /// use std::fs;
    /// # use std::fs::File;
    /// # use std::io::prelude::*;
    /// # let mut file = File::create("rom.db").unwrap();
    /// # file.write_all(&vec![0x00; 0x150]).unwrap();
    ///
    /// let rom = fs::read("rom.db").unwrap();
    /// let db = PkmnapiDB::new(&rom).unwrap();
    ///
    /// assert_eq!(db.verify_hash("6923685781779ac0b69c77ec08ce0479"), true);
    /// # fs::remove_file("rom.db");
    /// ```
    pub fn verify_hash<S: Into<String>>(&self, hash: S) -> bool {
        self.hash == hash.into()
    }

    /// Apply ROM patch
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi::db::patch::*;
    /// use pkmnapi::db::*;
    /// use std::fs;
    /// # use std::fs::File;
    /// # use std::io::prelude::*;
    /// # let mut file = File::create("rom.db").unwrap();
    /// # file.write_all(&vec![0x00; 0x150]).unwrap();
    ///
    /// let rom = fs::read("rom.db").unwrap();
    /// let mut db = PkmnapiDB::new(&rom).unwrap();
    ///
    /// assert_eq!(db.rom[..4], [0x00, 0x00, 0x00, 0x00]);
    ///
    /// let patch = PkmnapiDBPatch::new(0x00, vec![0x13, 0x37]);
    ///
    /// db.apply_patch(patch);
    ///
    /// assert_eq!(db.rom[..4], [0x13, 0x37, 0x00, 0x00]);
    /// # fs::remove_file("rom.db");
    /// ```
    pub fn apply_patch(&mut self, patch: PkmnapiDBPatch) {
        let rom = [
            &self.rom[..patch.offset],
            &patch.data[..],
            &self.rom[(patch.offset + patch.length)..],
        ]
        .concat();

        self.rom = rom;
    }

    /// Pokédex ID to internal ID
    ///
    /// # Example
    ///
    /// ```
    /// use std::fs;
    /// use pkmnapi::db::*;
    /// # use std::fs::File;
    /// # use std::io::prelude::*;
    /// # let mut file = File::create("rom.db").unwrap();
    /// # let data: Vec<u8> = [
    /// #     vec![0x00; 0x41024],
    /// #     vec![0x70, 0x73, 0x20, 0x23, 0x15, 0x64, 0x22, 0x50, 0x02, 0x67, 0x6C, 0x66, 0x58, 0x5E, 0x1D, 0x1F, 0x68, 0x6F, 0x83, 0x3B, 0x97],
    /// #     vec![0x00; 0xA9]
    /// # ].concat();
    /// # file.write_all(&data).unwrap();
    ///
    /// let rom = fs::read("rom.db").unwrap();
    /// let db = PkmnapiDB::new(&rom).unwrap();
    ///
    /// let pokedex_id = 151;
    /// let internal_id = db.pokedex_id_to_internal_id(pokedex_id).unwrap();
    ///
    /// assert_eq!(internal_id, 0x14);
    /// # fs::remove_file("rom.db");
    /// ```
    pub fn pokedex_id_to_internal_id<S: Into<PkmnapiDBPokedexID>>(
        &self,
        pokedex_id: S,
    ) -> Result<PkmnapiDBInternalID, String> {
        let pokedex_id = pokedex_id.into();

        if pokedex_id < 1 {
            return Err(format!("Invalid Pokédex ID: {}", pokedex_id));
        }

        let offset_base = ROM_PAGE * 0x20;
        let offset = offset_base + 0x1024;

        let internal_id = match (&self.rom[offset..(offset + POKEMON_INTERNAL_MAX)])
            .iter()
            .position(|&r| pokedex_id == r)
        {
            Some(internal_id) => internal_id,
            None => return Err(format!("Invalid Pokédex ID: {}", pokedex_id)),
        };

        Ok(PkmnapiDBInternalID::from(internal_id as u8))
    }

    /// Internal ID to Pokédex ID
    ///
    /// # Example
    ///
    /// ```
    /// use std::fs;
    /// use pkmnapi::db::*;
    /// # use std::fs::File;
    /// # use std::io::prelude::*;
    /// # let mut file = File::create("rom.db").unwrap();
    /// # let data: Vec<u8> = [
    /// #     vec![0x00; 0x41024],
    /// #     vec![0x70, 0x73, 0x20, 0x23, 0x15, 0x64, 0x22, 0x50, 0x02, 0x67, 0x6C, 0x66, 0x58, 0x5E, 0x1D, 0x1F, 0x68, 0x6F, 0x83, 0x3B, 0x97],
    /// # ].concat();
    /// # file.write_all(&data).unwrap();
    ///
    /// let rom = fs::read("rom.db").unwrap();
    /// let db = PkmnapiDB::new(&rom).unwrap();
    ///
    /// let internal_id = 0x14;
    /// let pokedex_id = db.internal_id_to_pokedex_id(internal_id).unwrap();
    ///
    /// assert_eq!(pokedex_id, 151);
    /// # fs::remove_file("rom.db");
    /// ```
    pub fn internal_id_to_pokedex_id<S: Into<PkmnapiDBInternalID>>(
        &self,
        internal_id: S,
    ) -> Result<PkmnapiDBPokedexID, String> {
        let internal_id = internal_id.into();

        if internal_id >= POKEMON_INTERNAL_MAX as u8 {
            return Err(format!("Invalid internal ID: {}", internal_id));
        }

        let offset_base = ROM_PAGE * 0x20;
        let offset = internal_id + (offset_base + 0x1024);

        Ok(PkmnapiDBPokedexID(self.rom[offset]))
    }

    /// Get type name by type ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi::db::string::*;
    /// use pkmnapi::db::*;
    /// use std::fs;
    /// # use std::fs::File;
    /// # use std::io::prelude::*;
    /// # let mut file = File::create("rom.db").unwrap();
    /// # let data: Vec<u8> = [
    /// #     vec![0x00; 0x27DAE],
    /// #     vec![0xB0, 0x7D],
    /// #     vec![0x8D, 0x8E, 0x91, 0x8C, 0x80, 0x8B, 0x50],
    /// #     vec![0x00; 0x03]
    /// # ].concat();
    /// # file.write_all(&data).unwrap();
    ///
    /// let rom = fs::read("rom.db").unwrap();
    /// let db = PkmnapiDB::new(&rom).unwrap();
    ///
    /// let type_name = db.get_type_name_by_id(0).unwrap();
    ///
    /// assert_eq!(type_name.name.decode_trimmed(), "NORMAL");
    /// # fs::remove_file("rom.db");
    /// ```
    pub fn get_type_name_by_id(&self, type_id: u8) -> Result<PkmnapiDBTypeName, String> {
        let offset_base = ROM_PAGE * 0x10;
        let pointer_offset = (offset_base + 0x7DAE) + (type_id as usize * 2);
        let pointer = offset_base + {
            let mut cursor = Cursor::new(&self.rom[pointer_offset..(pointer_offset + 2)]);

            cursor.read_u16::<LittleEndian>().unwrap_or(0) as usize
        };

        let type_name = PkmnapiDBTypeName::from(&self.rom[pointer..=(pointer + 9)]);

        Ok(type_name)
    }

    /// Set type name by type ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi::db::patch::*;
    /// use pkmnapi::db::string::*;
    /// use pkmnapi::db::*;
    /// use std::fs;
    /// # use std::fs::File;
    /// # use std::io::prelude::*;
    /// # let mut file = File::create("rom.db").unwrap();
    /// # let data: Vec<u8> = [
    /// #     vec![0x00; 0x27DAE],
    /// #     vec![0xB0, 0x7D],
    /// #     vec![0x8D, 0x8E, 0x91, 0x8C, 0x80, 0x8B, 0x50],
    /// #     vec![0x00; 0x03]
    /// # ].concat();
    /// # file.write_all(&data).unwrap();
    ///
    /// let rom = fs::read("rom.db").unwrap();
    /// let db = PkmnapiDB::new(&rom).unwrap();
    ///
    /// let patch = db
    ///     .set_type_name_by_id(
    ///         0,
    ///         PkmnapiDBTypeName {
    ///             name: PkmnapiDBString::from_string("BORING"),
    ///         },
    ///     )
    ///     .unwrap();
    ///
    /// assert_eq!(
    ///     patch,
    ///     PkmnapiDBPatch {
    ///         offset: 0x27DB0,
    ///         length: 6,
    ///         data: vec![0x81, 0x8E, 0x91, 0x88, 0x8D, 0x86]
    ///     }
    /// );
    /// # fs::remove_file("rom.db");
    /// ```
    pub fn set_type_name_by_id(
        &self,
        type_id: u8,
        type_name: PkmnapiDBTypeName,
    ) -> Result<PkmnapiDBPatch, String> {
        let old_type_name = self.get_type_name_by_id(type_id)?;
        let old_type_name = old_type_name.name.decode_trimmed();
        let old_type_name_len = old_type_name.len();
        let type_name_raw = type_name.to_raw();
        let type_name_len = type_name_raw.len();

        if old_type_name_len < type_name_len {
            return Err(format!(
                "Length mismatch: should be {} characters or fewer, found {}",
                old_type_name_len, type_name_len
            ));
        }

        let offset_base = ROM_PAGE * 0x10;
        let pointer_offset = (offset_base + 0x7DAE) + (type_id as usize * 2);
        let pointer = offset_base + {
            let mut cursor = Cursor::new(&self.rom[pointer_offset..(pointer_offset + 2)]);

            cursor.read_u16::<LittleEndian>().unwrap_or(0) as usize
        };

        let data = [type_name_raw, vec![0x50; old_type_name_len - type_name_len]].concat();

        Ok(PkmnapiDBPatch::new(pointer, data))
    }

    /// Get type effect by type effect ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi::db::*;
    /// use std::fs;
    /// # use std::fs::File;
    /// # use std::io::prelude::*;
    /// # let mut file = File::create("rom.db").unwrap();
    /// # let data: Vec<u8> = [
    /// #     vec![0x00; 0x3E474],
    /// #     vec![0x01, 0x02, 0x14],
    /// #     vec![0xFF]
    /// # ].concat();
    /// # file.write_all(&data).unwrap();
    ///
    /// let rom = fs::read("rom.db").unwrap();
    /// let db = PkmnapiDB::new(&rom).unwrap();
    ///
    /// let type_effect = db.get_type_effect_by_id(0).unwrap();
    ///
    /// assert_eq!(
    ///     type_effect,
    ///     PkmnapiDBTypeEffect {
    ///         attacking_type_id: 0x01,
    ///         defending_type_id: 0x02,
    ///         multiplier: 2.0
    ///     }
    /// );
    /// # fs::remove_file("rom.db");
    /// ```
    pub fn get_type_effect_by_id(&self, type_effect_id: u8) -> Result<PkmnapiDBTypeEffect, String> {
        let offset_base = ROM_PAGE * 0x1F;
        let pointer = offset_base + 0x0474;

        let max_index = (&self.rom[pointer..])
            .iter()
            .position(|&r| r == 0xFF)
            .unwrap();
        let max_id = ((max_index as f32) / 3.0) as u8;

        if type_effect_id >= max_id {
            return Err(format!("Invalid ID: valid range is 0-{}", max_id));
        }

        let pointer = pointer + (type_effect_id as usize * 3);

        let type_effect = PkmnapiDBTypeEffect::from(&self.rom[pointer..(pointer + 3)]);

        Ok(type_effect)
    }

    /// Set type effect by type effect ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi::db::patch::*;
    /// use pkmnapi::db::*;
    /// use std::fs;
    /// # use std::fs::File;
    /// # use std::io::prelude::*;
    /// # let mut file = File::create("rom.db").unwrap();
    /// # let data: Vec<u8> = [
    /// #     vec![0x00; 0x3E474],
    /// #     vec![0x01, 0x02, 0x14],
    /// #     vec![0xFF]
    /// # ].concat();
    /// # file.write_all(&data).unwrap();
    ///
    /// let rom = fs::read("rom.db").unwrap();
    /// let db = PkmnapiDB::new(&rom).unwrap();
    ///
    /// let patch = db
    ///     .set_type_effect_by_id(
    ///         0,
    ///         PkmnapiDBTypeEffect {
    ///             attacking_type_id: 0x13,
    ///             defending_type_id: 0x37,
    ///             multiplier: 0.5,
    ///         },
    ///     )
    ///     .unwrap();
    ///
    /// assert_eq!(
    ///     patch,
    ///     PkmnapiDBPatch {
    ///         offset: 0x3E474,
    ///         length: 0x03,
    ///         data: vec![0x13, 0x37, 0x05]
    ///     }
    /// );
    /// # fs::remove_file("rom.db");
    /// ```
    pub fn set_type_effect_by_id(
        &self,
        type_effect_id: u8,
        type_effect: PkmnapiDBTypeEffect,
    ) -> Result<PkmnapiDBPatch, String> {
        let offset_base = ROM_PAGE * 0x1F;
        let pointer = offset_base + 0x0474;

        let max_index = (&self.rom[pointer..])
            .iter()
            .position(|&r| r == 0xFF)
            .unwrap();
        let max_id = ((max_index as f32) / 3.0) as u8;

        if type_effect_id >= max_id {
            return Err(format!("Invalid ID: valid range is 0-{}", max_id));
        }

        let pointer = pointer + (type_effect_id as usize * 3);

        let type_effect_raw = type_effect.to_raw();

        Ok(PkmnapiDBPatch::new(pointer, type_effect_raw))
    }

    /// Get stats by Pokédex ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi::db::*;
    /// use std::fs;
    /// # use std::fs::File;
    /// # use std::io::prelude::*;
    /// # let mut file = File::create("rom.db").unwrap();
    /// # let data: Vec<u8> = [
    /// #     vec![0x00; 0x383DE],
    /// #     vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A],
    /// #     vec![0x00; 0x12]
    /// # ].concat();
    /// # file.write_all(&data).unwrap();
    ///
    /// let rom = fs::read("rom.db").unwrap();
    /// let db = PkmnapiDB::new(&rom).unwrap();
    ///
    /// let type_effect = db.get_stats_by_id(1).unwrap();
    ///
    /// assert_eq!(
    ///     type_effect,
    ///     PkmnapiDBStats {
    ///         pokedex_id: PkmnapiDBPokedexID::from(0x01),
    ///         base_hp: 0x02,
    ///         base_attack: 0x03,
    ///         base_defence: 0x04,
    ///         base_speed: 0x05,
    ///         base_special: 0x06,
    ///         type_ids: vec![0x07, 0x08],
    ///         catch_rate: 0x09,
    ///         base_exp_yield: 0x0A
    ///     }
    /// );
    /// # fs::remove_file("rom.db");
    /// ```
    pub fn get_stats_by_id<S: Into<PkmnapiDBPokedexID>>(
        &self,
        pokedex_id: S,
    ) -> Result<PkmnapiDBStats, String> {
        let pokedex_id = pokedex_id.into();

        if pokedex_id < 1 {
            return Err(format!("Pokédex ID too low: {}", pokedex_id));
        }

        let offset_base = ROM_PAGE * 0x1C;
        let offset = (offset_base + 0x03DE) + ((pokedex_id - 1) * 0x1C);

        let stats = PkmnapiDBStats::from(&self.rom[offset..(offset + 0x1C)]);

        Ok(stats)
    }

    /// Set stats by Pokédex ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi::db::patch::*;
    /// use pkmnapi::db::*;
    /// use std::fs;
    /// # use std::fs::File;
    /// # use std::io::prelude::*;
    /// # let mut file = File::create("rom.db").unwrap();
    /// # let data: Vec<u8> = [
    /// #     vec![0x00; 0x383DE],
    /// #     vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A],
    /// #     vec![0x00; 0x12]
    /// # ].concat();
    /// # file.write_all(&data).unwrap();
    ///
    /// let rom = fs::read("rom.db").unwrap();
    /// let db = PkmnapiDB::new(&rom).unwrap();
    ///
    /// let type_effect = db
    ///     .set_stats_by_id(
    ///         1,
    ///         PkmnapiDBStats {
    ///             pokedex_id: PkmnapiDBPokedexID::from(0x01),
    ///             base_hp: 0x42,
    ///             base_attack: 0x13,
    ///             base_defence: 0x37,
    ///             base_speed: 0x13,
    ///             base_special: 0x37,
    ///             type_ids: vec![0x13, 0x37],
    ///             catch_rate: 0x13,
    ///             base_exp_yield: 0x37,
    ///         },
    ///     )
    ///     .unwrap();
    ///
    /// assert_eq!(
    ///     type_effect,
    ///     PkmnapiDBPatch {
    ///         offset: 0x383DE,
    ///         length: 0x0A,
    ///         data: vec![0x01, 0x42, 0x13, 0x37, 0x13, 0x37, 0x13, 0x37, 0x13, 0x37]
    ///     }
    /// );
    /// # fs::remove_file("rom.db");
    /// ```
    pub fn set_stats_by_id(
        &self,
        pokedex_id: u8,
        stats: PkmnapiDBStats,
    ) -> Result<PkmnapiDBPatch, String> {
        if pokedex_id < 1 {
            return Err(format!("Pokédex ID too low: {}", pokedex_id));
        }

        let offset_base = ROM_PAGE * 0x1C;
        let offset = (offset_base + 0x03DE) + ((pokedex_id as usize - 1) * 0x1C);

        let stats_raw = stats.to_raw();

        Ok(PkmnapiDBPatch::new(offset, stats_raw))
    }
}

/// Pokédex ID
///
/// # Example
///
/// ```
/// use pkmnapi::db::*;
///
/// let pokedex_id = PkmnapiDBPokedexID::from(151);
///
/// assert_eq!(pokedex_id, 151);
/// ```
#[derive(Debug, PartialEq)]
pub struct PkmnapiDBPokedexID(u8);

impl PkmnapiDBPokedexID {
    pub fn value(&self) -> u8 {
        self.0
    }
}

impl From<u8> for PkmnapiDBPokedexID {
    fn from(pokedex_id: u8) -> Self {
        PkmnapiDBPokedexID(pokedex_id)
    }
}

impl PartialEq<u8> for PkmnapiDBPokedexID {
    fn eq(&self, other: &u8) -> bool {
        self.0 == *other
    }
}

impl PartialOrd<u8> for PkmnapiDBPokedexID {
    fn partial_cmp(&self, other: &u8) -> Option<Ordering> {
        self.0.partial_cmp(&other)
    }
}

impl fmt::Display for PkmnapiDBPokedexID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Sub<usize> for PkmnapiDBPokedexID {
    type Output = usize;

    fn sub(self, other: usize) -> usize {
        self.0 as usize - other
    }
}

/// Internal ID
///
/// # Example
///
/// ```
/// use pkmnapi::db::*;
///
/// let internal_id = PkmnapiDBInternalID::from(0x14);
///
/// assert_eq!(internal_id, 0x14);
/// ```
#[derive(Debug)]
pub struct PkmnapiDBInternalID(u8);

impl PkmnapiDBInternalID {
    pub fn value(&self) -> u8 {
        self.0
    }
}

impl From<u8> for PkmnapiDBInternalID {
    fn from(internal_id: u8) -> Self {
        PkmnapiDBInternalID(internal_id)
    }
}

impl fmt::Display for PkmnapiDBInternalID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialEq<u8> for PkmnapiDBInternalID {
    fn eq(&self, other: &u8) -> bool {
        self.0 == *other
    }
}

impl PartialOrd<u8> for PkmnapiDBInternalID {
    fn partial_cmp(&self, other: &u8) -> Option<Ordering> {
        self.0.partial_cmp(&other)
    }
}

impl Add<usize> for PkmnapiDBInternalID {
    type Output = usize;

    fn add(self, other: usize) -> usize {
        self.0 as usize + other
    }
}

/// Type name
///
/// # Example
///
/// ```
/// use pkmnapi::db::string::*;
/// use pkmnapi::db::*;
///
/// let rom = vec![0x80, 0x81, 0x82, 0x50];
/// let type_name = PkmnapiDBTypeName::from(&rom[..]);
///
/// assert_eq!(
///     type_name,
///     PkmnapiDBTypeName {
///         name: PkmnapiDBString::from_string("ABC@")
///     }
/// );
/// ```
#[derive(Debug, PartialEq)]
pub struct PkmnapiDBTypeName {
    pub name: PkmnapiDBString,
}

impl From<&[u8]> for PkmnapiDBTypeName {
    /// Convert &[u8] to PkmnapiDBTypeName
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi::db::string::*;
    /// use pkmnapi::db::*;
    ///
    /// let rom = vec![0x80, 0x81, 0x82, 0x50];
    /// let type_name = PkmnapiDBTypeName::from(&rom[..]);
    ///
    /// assert_eq!(
    ///     type_name,
    ///     PkmnapiDBTypeName {
    ///         name: PkmnapiDBString::from_string("ABC@")
    ///     }
    /// );
    /// ```
    fn from(rom: &[u8]) -> Self {
        let name = PkmnapiDBString::new(rom);

        PkmnapiDBTypeName { name }
    }
}

impl PkmnapiDBTypeName {
    /// Type name to raw bytes
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi::db::string::*;
    /// use pkmnapi::db::*;
    ///
    /// let type_name = PkmnapiDBTypeName {
    ///     name: PkmnapiDBString::from_string("ABC@"),
    /// };
    ///
    /// let raw = type_name.to_raw();
    ///
    /// assert_eq!(raw, vec![0x80, 0x81, 0x82, 0x50]);
    /// ```
    pub fn to_raw(&self) -> Vec<u8> {
        self.name.value[..].to_vec()
    }
}

/// Type effect (attacker, defender, multiplier)
///
/// # Example
///
/// ```
/// use pkmnapi::db::*;
///
/// let rom = vec![0x01, 0x02, 0x14];
/// let type_effect = PkmnapiDBTypeEffect::from(&rom[..]);
///
/// assert_eq!(
///     type_effect,
///     PkmnapiDBTypeEffect {
///         attacking_type_id: 0x01,
///         defending_type_id: 0x02,
///         multiplier: 2.0
///     }
/// );
/// ```
#[derive(Debug, PartialEq)]
pub struct PkmnapiDBTypeEffect {
    pub attacking_type_id: u8,
    pub defending_type_id: u8,
    pub multiplier: f32,
}

impl From<&[u8]> for PkmnapiDBTypeEffect {
    /// Convert &[u8] to PkmnapiDBTypeEffect
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi::db::*;
    ///
    /// let rom = vec![0x01, 0x02, 0x14];
    /// let type_effect = PkmnapiDBTypeEffect::from(&rom[..]);
    ///
    /// assert_eq!(
    ///     type_effect,
    ///     PkmnapiDBTypeEffect {
    ///         attacking_type_id: 0x01,
    ///         defending_type_id: 0x02,
    ///         multiplier: 2.0
    ///     }
    /// );
    /// ```
    fn from(rom: &[u8]) -> Self {
        let mut cursor = Cursor::new(rom);

        let attacking_type_id = cursor.read_u8().unwrap_or(0);
        let defending_type_id = cursor.read_u8().unwrap_or(0);
        let multiplier = (cursor.read_u8().unwrap_or(0) as f32) / 10.0;

        PkmnapiDBTypeEffect {
            attacking_type_id,
            defending_type_id,
            multiplier,
        }
    }
}

impl PkmnapiDBTypeEffect {
    /// Convert type effect to raw bytes
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi::db::*;
    ///
    /// let type_effect = PkmnapiDBTypeEffect {
    ///     attacking_type_id: 0x01,
    ///     defending_type_id: 0x02,
    ///     multiplier: 2.0,
    /// };
    ///
    /// let raw = type_effect.to_raw();
    ///
    /// assert_eq!(raw, vec![0x01, 0x02, 0x14]);
    /// ```
    pub fn to_raw(&self) -> Vec<u8> {
        vec![
            self.attacking_type_id,
            self.defending_type_id,
            cmp::min((self.multiplier * 10.0) as u8, 254),
        ]
    }
}

/// Pokemon stats (HP, attack, defence, etc)
///
/// # Example
///
/// ```
/// use pkmnapi::db::*;
///
/// let rom = vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A];
/// let stats = PkmnapiDBStats::from(&rom[..]);
///
/// assert_eq!(
///     stats,
///     PkmnapiDBStats {
///         pokedex_id: PkmnapiDBPokedexID::from(0x01),
///         base_hp: 0x02,
///         base_attack: 0x03,
///         base_defence: 0x04,
///         base_speed: 0x05,
///         base_special: 0x06,
///         type_ids: vec![0x07, 0x08],
///         catch_rate: 0x09,
///         base_exp_yield: 0x0A
///     }
/// );
/// ```
#[derive(Debug, PartialEq)]
pub struct PkmnapiDBStats {
    pub pokedex_id: PkmnapiDBPokedexID,
    pub base_hp: u8,
    pub base_attack: u8,
    pub base_defence: u8,
    pub base_speed: u8,
    pub base_special: u8,
    pub type_ids: Vec<u8>,
    pub catch_rate: u8,
    pub base_exp_yield: u8,
}

impl From<&[u8]> for PkmnapiDBStats {
    /// Convert &[u8] to PkmnapiDBStats
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi::db::*;
    ///
    /// let rom = vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A];
    /// let stats = PkmnapiDBStats::from(&rom[..]);
    ///
    /// assert_eq!(
    ///     stats,
    ///     PkmnapiDBStats {
    ///         pokedex_id: PkmnapiDBPokedexID::from(0x01),
    ///         base_hp: 0x02,
    ///         base_attack: 0x03,
    ///         base_defence: 0x04,
    ///         base_speed: 0x05,
    ///         base_special: 0x06,
    ///         type_ids: vec![0x07, 0x08],
    ///         catch_rate: 0x09,
    ///         base_exp_yield: 0x0A
    ///     }
    /// );
    /// ```
    fn from(rom: &[u8]) -> Self {
        let mut cursor = Cursor::new(rom);

        let pokedex_id = PkmnapiDBPokedexID::from(cursor.read_u8().unwrap_or(0));
        let base_hp = cursor.read_u8().unwrap_or(0);
        let base_attack = cursor.read_u8().unwrap_or(0);
        let base_defence = cursor.read_u8().unwrap_or(0);
        let base_speed = cursor.read_u8().unwrap_or(0);
        let base_special = cursor.read_u8().unwrap_or(0);
        let type_ids = {
            let mut type_ids = vec![0x00; 2];

            cursor.read_exact(&mut type_ids).unwrap();

            type_ids
        };
        let catch_rate = cursor.read_u8().unwrap_or(0);
        let base_exp_yield = cursor.read_u8().unwrap_or(0);

        PkmnapiDBStats {
            pokedex_id,
            base_hp,
            base_attack,
            base_defence,
            base_speed,
            base_special,
            type_ids,
            catch_rate,
            base_exp_yield,
        }
    }
}

impl PkmnapiDBStats {
    /// Convert stats to raw bytes
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi::db::*;
    ///
    /// let stats = PkmnapiDBStats {
    ///     pokedex_id: PkmnapiDBPokedexID::from(0x01),
    ///     base_hp: 0x02,
    ///     base_attack: 0x03,
    ///     base_defence: 0x04,
    ///     base_speed: 0x05,
    ///     base_special: 0x06,
    ///     type_ids: vec![0x07, 0x08],
    ///     catch_rate: 0x09,
    ///     base_exp_yield: 0x0A,
    /// };
    ///
    /// let raw = stats.to_raw();
    ///
    /// assert_eq!(
    ///     raw,
    ///     vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A]
    /// );
    /// ```
    pub fn to_raw(&self) -> Vec<u8> {
        [
            &[
                self.pokedex_id.value(),
                self.base_hp,
                self.base_attack,
                self.base_defence,
                self.base_speed,
                self.base_special,
            ],
            &self.type_ids[..],
            &[self.catch_rate, self.base_exp_yield],
        ]
        .concat()
    }
}

#[cfg(test)]
mod tests {
    use crate::db::*;

    #[test]
    fn new_success() {
        let rom = vec![0u8; 1024];

        PkmnapiDB::new(&rom).unwrap();
    }

    #[test]
    fn new_failure() {
        let rom = vec![];

        match PkmnapiDB::new(&rom) {
            Err(e) => assert_eq!(e, "Header too small"),
            _ => {}
        };
    }

    #[test]
    fn header_success() {
        let rom = [
            vec![0u8; 0x100],             // padding
            vec![0x00, 0xC3, 0x50, 0x01], // entry_point
            vec![
                // logo
                0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C,
                0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6,
                0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC,
                0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
            ],
            "GAMEBOYGAME".chars().map(|c| c as u8).collect::<Vec<u8>>(), // title
            vec![0u8; 5],                                                // title padding
            vec![0x30u8, 0x31u8],                                        // new_licensee_code
            vec![0u8],                                                   // sgb_flag
            vec![0u8],                                                   // cartridge_type
            vec![0x05u8],                                                // rom_size
            vec![0u8],                                                   // ram_size
            vec![0x01u8],                                                // destination_code
            vec![0x01u8],                                                // old_licensee_code
            vec![0x01u8],                                                // mask_rom_version_number
            vec![0x60],                                                  // header_checksum
            vec![0x1A, 0x41],                                            // global_checksum
        ]
        .concat();
        let db = PkmnapiDB::new(&rom).unwrap();
        let header = PkmnapiDBHeader::from(&rom).unwrap();

        assert_eq!(db.header, header);
    }

    #[test]
    fn header_verify_success() {
        let rom = [
            vec![0u8; 0x100],             // padding
            vec![0x00, 0xC3, 0x50, 0x01], // entry_point
            vec![
                // logo
                0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C,
                0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6,
                0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC,
                0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
            ],
            "GAMEBOYGAME".chars().map(|c| c as u8).collect::<Vec<u8>>(), // title
            vec![0u8; 5],                                                // title padding
            vec![0x30u8, 0x31u8],                                        // new_licensee_code
            vec![0u8],                                                   // sgb_flag
            vec![0u8],                                                   // cartridge_type
            vec![0x05u8],                                                // rom_size
            vec![0u8],                                                   // ram_size
            vec![0x01u8],                                                // destination_code
            vec![0x01u8],                                                // old_licensee_code
            vec![0x01u8],                                                // mask_rom_version_number
            vec![0x60],                                                  // header_checksum
            vec![0x1A, 0x41],                                            // global_checksum
        ]
        .concat();
        let db = PkmnapiDB::new(&rom).unwrap();

        assert_eq!(db.header.verify_checksum(), true);
    }

    #[test]
    fn header_verify_fail() {
        let rom = [
            vec![0u8; 0x100],             // padding
            vec![0x00, 0xC3, 0x50, 0x01], // entry_point
            vec![
                // logo
                0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C,
                0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6,
                0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC,
                0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
            ],
            "GAMEBOYGAME".chars().map(|c| c as u8).collect::<Vec<u8>>(), // title
            vec![0u8; 5],                                                // title padding
            vec![0x30u8, 0x31u8],                                        // new_licensee_code
            vec![0u8],                                                   // sgb_flag
            vec![0u8],                                                   // cartridge_type
            vec![0x05u8],                                                // rom_size
            vec![0u8],                                                   // ram_size
            vec![0x01u8],                                                // destination_code
            vec![0x01u8],                                                // old_licensee_code
            vec![0x01u8],                                                // mask_rom_version_number
            vec![0x42],                                                  // (wrong) header_checksum
            vec![0x1A, 0x41],                                            // global_checksum
        ]
        .concat();
        let db = PkmnapiDB::new(&rom).unwrap();

        assert_eq!(db.header.verify_checksum(), false);
    }

    #[test]
    fn verify_checksum_success() {
        let rom = [
            vec![0u8; 0x100],             // padding
            vec![0x00, 0xC3, 0x50, 0x01], // entry_point
            vec![
                // logo
                0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C,
                0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6,
                0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC,
                0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
            ],
            "GAMEBOYGAME".chars().map(|c| c as u8).collect::<Vec<u8>>(), // title
            vec![0u8; 5],                                                // title padding
            vec![0x30u8, 0x31u8],                                        // new_licensee_code
            vec![0u8],                                                   // sgb_flag
            vec![0u8],                                                   // cartridge_type
            vec![0x05u8],                                                // rom_size
            vec![0u8],                                                   // ram_size
            vec![0x01u8],                                                // destination_code
            vec![0x01u8],                                                // old_licensee_code
            vec![0x01u8],                                                // mask_rom_version_number
            vec![0x60],                                                  // header_checksum
            vec![0x1A, 0x41],                                            // global_checksum
        ]
        .concat();
        let mut db = PkmnapiDB::new(&rom).unwrap();

        db.header.global_checksum = 0x1A41;

        assert_eq!(db.verify_checksum(), true);
    }

    #[test]
    fn verify_checksum_fail() {
        let rom = [
            vec![0u8; 0x100],             // padding
            vec![0x00, 0xC3, 0x50, 0x01], // entry_point
            vec![
                // logo
                0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C,
                0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6,
                0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC,
                0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
            ],
            "GAMEBOYGAME".chars().map(|c| c as u8).collect::<Vec<u8>>(), // title
            vec![0u8; 5],                                                // title padding
            vec![0x30u8, 0x31u8],                                        // new_licensee_code
            vec![0u8],                                                   // sgb_flag
            vec![0u8],                                                   // cartridge_type
            vec![0x05u8],                                                // rom_size
            vec![0u8],                                                   // ram_size
            vec![0x01u8],                                                // destination_code
            vec![0x01u8],                                                // old_licensee_code
            vec![0x01u8],                                                // mask_rom_version_number
            vec![0x60],                                                  // header_checksum
            vec![0x1A, 0x41],                                            // global_checksum
        ]
        .concat();
        let mut db = PkmnapiDB::new(&rom).unwrap();

        db.header.global_checksum = 0x1234;

        assert_eq!(db.verify_checksum(), false);
    }

    #[test]
    fn verify_hash_success() {
        let rom = [
            vec![0u8; 0x100],             // padding
            vec![0x00, 0xC3, 0x50, 0x01], // entry_point
            vec![
                // logo
                0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C,
                0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6,
                0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC,
                0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
            ],
            "GAMEBOYGAME".chars().map(|c| c as u8).collect::<Vec<u8>>(), // title
            vec![0u8; 5],                                                // title padding
            vec![0x30u8, 0x31u8],                                        // new_licensee_code
            vec![0u8],                                                   // sgb_flag
            vec![0u8],                                                   // cartridge_type
            vec![0x05u8],                                                // rom_size
            vec![0u8],                                                   // ram_size
            vec![0x01u8],                                                // destination_code
            vec![0x01u8],                                                // old_licensee_code
            vec![0x01u8],                                                // mask_rom_version_number
            vec![0x60],                                                  // header_checksum
            vec![0x1A, 0x41],                                            // global_checksum
        ]
        .concat();
        let db = PkmnapiDB::new(&rom).unwrap();

        assert_eq!(db.verify_hash("b933af3d953bedd6ed3911ef6724cfa2"), true);
    }

    #[test]
    fn verify_hash_fail() {
        let rom = [
            vec![0u8; 0x100],             // padding
            vec![0x00, 0xC3, 0x50, 0x01], // entry_point
            vec![
                // logo
                0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C,
                0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6,
                0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC,
                0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
            ],
            "GAMEBOYGAME".chars().map(|c| c as u8).collect::<Vec<u8>>(), // title
            vec![0u8; 5],                                                // title padding
            vec![0x30u8, 0x31u8],                                        // new_licensee_code
            vec![0u8],                                                   // sgb_flag
            vec![0u8],                                                   // cartridge_type
            vec![0x05u8],                                                // rom_size
            vec![0u8],                                                   // ram_size
            vec![0x01u8],                                                // destination_code
            vec![0x01u8],                                                // old_licensee_code
            vec![0x01u8],                                                // mask_rom_version_number
            vec![0x60],                                                  // header_checksum
            vec![0x1A, 0x41],                                            // global_checksum
        ]
        .concat();
        let db = PkmnapiDB::new(&rom).unwrap();

        assert_eq!(db.verify_hash("0123456789abcdef0123456789abcdef"), false);
    }
}
