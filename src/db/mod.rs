//! Pkmnapi database module
//!
//! # Example
//!
//! ```
//! use pkmnapi::db::*;
//! use std::fs;
//! # use std::fs::File;
//! # use std::io::prelude::*;
//! # let mut file = File::create("rom.db").unwrap();
//! # let data = vec![0x00; 0x150];
//! # file.write_all(&data).unwrap();
//!
//! let rom = fs::read("rom.db").unwrap();
//! let db = PkmnapiDB::new(&rom).unwrap();
//! # fs::remove_file("rom.db");
//! ```

pub mod header;
pub mod patch;
pub mod string;
pub mod types;

use byteorder::{LittleEndian, ReadBytesExt};
use header::*;
use patch::*;
use std::io::Cursor;
use std::num::Wrapping;
use types::*;

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
/// # let data = vec![0x00; 0x150];
/// # file.write_all(&data).unwrap();
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
    /// # let data = vec![0x00; 0x150];
    /// # file.write_all(&data).unwrap();
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
    /// # let data = vec![0x00; 0x150];
    /// # file.write_all(&data).unwrap();
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
    /// # let data = vec![0x00; 0x150];
    /// # file.write_all(&data).unwrap();
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
    /// # let data = vec![0x00; 0x150];
    /// # file.write_all(&data).unwrap();
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
    /// # let data = vec![0x00; 0x150];
    /// # file.write_all(&data).unwrap();
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

    /// Pokémon name to Pokédex ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi::db::string::*;
    /// use pkmnapi::db::types::*;
    /// use pkmnapi::db::*;
    /// use std::fs;
    /// # use std::fs::File;
    /// # use std::io::prelude::*;
    /// # let mut file = File::create("rom.db").unwrap();
    /// # let data: Vec<u8> = [
    /// #     vec![0x00; 0x1C21E],
    /// #     vec![0x91, 0x87, 0x98, 0x83, 0x8E, 0x8D, 0x50, 0x50, 0x50, 0x50],
    /// #     vec![0x00; 0x24DFC],
    /// #     vec![0x70],
    /// #     vec![0x00; 0xBD]
    /// # ].concat();
    /// # file.write_all(&data).unwrap();
    ///
    /// let rom = fs::read("rom.db").unwrap();
    /// let db = PkmnapiDB::new(&rom).unwrap();
    ///
    /// let pokemon_name = PkmnapiDBPokemonName {
    ///     name: PkmnapiDBString::from("RHYDON"),
    /// };
    ///
    /// let pokedex_id = db.pokemon_name_to_pokedex_id(pokemon_name).unwrap();
    ///
    /// assert_eq!(pokedex_id, PkmnapiDBPokedexID::from(112));
    /// # fs::remove_file("rom.db");
    /// ```
    pub fn pokemon_name_to_pokedex_id(
        &self,
        pokemon_name: PkmnapiDBPokemonName,
    ) -> Option<PkmnapiDBPokedexID> {
        let offset_base = ROM_PAGE * 0x0E;
        let offset = offset_base + 0x021E;

        return (0..POKEMON_INTERNAL_MAX)
            .map(|i| offset + (i * 0x0A))
            .enumerate()
            .filter_map(|(internal_id, offset)| {
                let internal_id = internal_id as u8;
                let name = PkmnapiDBPokemonName::from(&self.rom[offset..(offset + 0x0A)]);

                if name == pokemon_name {
                    let pokedex_id = self.internal_id_to_pokedex_id(internal_id).unwrap();

                    return Some(pokedex_id);
                }

                None
            })
            .take(1)
            .next();
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

        Ok(PkmnapiDBPokedexID::from(self.rom[offset]))
    }

    /// Get type name by type ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi::db::string::*;
    /// use pkmnapi::db::types::*;
    /// use pkmnapi::db::*;
    /// use std::fs;
    /// # use std::fs::File;
    /// # use std::io::prelude::*;
    /// # let mut file = File::create("rom.db").unwrap();
    /// # let data: Vec<u8> = [
    /// #     vec![0x00; 0x27DAE],
    /// #     vec![0xB0, 0x7D],
    /// #     vec![0x8D, 0x8E, 0x91, 0x8C, 0x80, 0x8B, 0x50],
    /// #     vec![0x50; 0x03]
    /// # ].concat();
    /// # file.write_all(&data).unwrap();
    ///
    /// let rom = fs::read("rom.db").unwrap();
    /// let db = PkmnapiDB::new(&rom).unwrap();
    ///
    /// let type_name = db.get_type_name(0).unwrap();
    ///
    /// assert_eq!(
    ///     type_name,
    ///     PkmnapiDBTypeName {
    ///         name: PkmnapiDBString::from("NORMAL")
    ///     }
    /// );
    /// # fs::remove_file("rom.db");
    /// ```
    pub fn get_type_name<S: Into<PkmnapiDBTypeID>>(
        &self,
        type_id: S,
    ) -> Result<PkmnapiDBTypeName, String> {
        let type_id = type_id.into();
        let offset_base = ROM_PAGE * 0x10;
        let pointer_offset = (offset_base + 0x7DAE) + (type_id * 2);
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
    /// use pkmnapi::db::types::*;
    /// use pkmnapi::db::*;
    /// use std::fs;
    /// # use std::fs::File;
    /// # use std::io::prelude::*;
    /// # let mut file = File::create("rom.db").unwrap();
    /// # let data: Vec<u8> = [
    /// #     vec![0x00; 0x27DAE],
    /// #     vec![0xB0, 0x7D],
    /// #     vec![0x8D, 0x8E, 0x91, 0x8C, 0x80, 0x8B, 0x50],
    /// #     vec![0x50; 0x03]
    /// # ].concat();
    /// # file.write_all(&data).unwrap();
    ///
    /// let rom = fs::read("rom.db").unwrap();
    /// let db = PkmnapiDB::new(&rom).unwrap();
    ///
    /// let patch = db
    ///     .set_type_name(
    ///         0,
    ///         PkmnapiDBTypeName {
    ///             name: PkmnapiDBString::from("BORING"),
    ///         },
    ///     )
    ///     .unwrap();
    ///
    /// assert_eq!(
    ///     patch,
    ///     PkmnapiDBPatch {
    ///         offset: 0x27DB0,
    ///         length: 0x06,
    ///         data: vec![0x81, 0x8E, 0x91, 0x88, 0x8D, 0x86]
    ///     }
    /// );
    /// # fs::remove_file("rom.db");
    /// ```
    pub fn set_type_name<S: Into<PkmnapiDBTypeID>>(
        &self,
        type_id: S,
        type_name: PkmnapiDBTypeName,
    ) -> Result<PkmnapiDBPatch, String> {
        let type_id = type_id.into();
        let old_type_name = self.get_type_name(type_id.clone())?;
        let old_type_name = old_type_name.name.to_string();
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
        let pointer_offset = (offset_base + 0x7DAE) + (type_id * 2);
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
    /// use pkmnapi::db::types::*;
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
    /// let type_effect = db.get_type_effect(0).unwrap();
    ///
    /// assert_eq!(
    ///     type_effect,
    ///     PkmnapiDBTypeEffect {
    ///         attacking_type_id: PkmnapiDBTypeID::from(0x01),
    ///         defending_type_id: PkmnapiDBTypeID::from(0x02),
    ///         multiplier: 2.0
    ///     }
    /// );
    /// # fs::remove_file("rom.db");
    /// ```
    pub fn get_type_effect<S: Into<PkmnapiDBTypeEffectID>>(
        &self,
        type_effect_id: S,
    ) -> Result<PkmnapiDBTypeEffect, String> {
        let type_effect_id = type_effect_id.into();
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

        let pointer = pointer + (type_effect_id * 0x03);

        let type_effect = PkmnapiDBTypeEffect::from(&self.rom[pointer..(pointer + 3)]);

        Ok(type_effect)
    }

    /// Set type effect by type effect ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi::db::patch::*;
    /// use pkmnapi::db::types::*;
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
    ///     .set_type_effect(
    ///         0,
    ///         PkmnapiDBTypeEffect {
    ///             attacking_type_id: PkmnapiDBTypeID::from(0x13),
    ///             defending_type_id: PkmnapiDBTypeID::from(0x37),
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
    pub fn set_type_effect<S: Into<PkmnapiDBTypeEffectID>>(
        &self,
        type_effect_id: S,
        type_effect: PkmnapiDBTypeEffect,
    ) -> Result<PkmnapiDBPatch, String> {
        let type_effect_id = type_effect_id.into();
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

        let pointer = pointer + (type_effect_id * 3);

        let type_effect_raw = type_effect.to_raw();

        Ok(PkmnapiDBPatch::new(pointer, type_effect_raw))
    }

    /// Get stats by Pokédex ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi::db::types::*;
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
    /// let type_effect = db.get_stats(1).unwrap();
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
    ///         type_ids: vec![PkmnapiDBTypeID::from(0x07), PkmnapiDBTypeID::from(0x08)],
    ///         catch_rate: 0x09,
    ///         base_exp_yield: 0x0A
    ///     }
    /// );
    /// # fs::remove_file("rom.db");
    /// ```
    pub fn get_stats<S: Into<PkmnapiDBPokedexID>>(
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
    /// use pkmnapi::db::types::*;
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
    /// let patch = db
    ///     .set_stats(
    ///         1,
    ///         PkmnapiDBStats {
    ///             pokedex_id: PkmnapiDBPokedexID::from(0x01),
    ///             base_hp: 0x42,
    ///             base_attack: 0x13,
    ///             base_defence: 0x37,
    ///             base_speed: 0x13,
    ///             base_special: 0x37,
    ///             type_ids: vec![PkmnapiDBTypeID::from(0x13), PkmnapiDBTypeID::from(0x37)],
    ///             catch_rate: 0x13,
    ///             base_exp_yield: 0x37,
    ///         },
    ///     )
    ///     .unwrap();
    ///
    /// assert_eq!(
    ///     patch,
    ///     PkmnapiDBPatch {
    ///         offset: 0x383DE,
    ///         length: 0x0A,
    ///         data: vec![0x01, 0x42, 0x13, 0x37, 0x13, 0x37, 0x13, 0x37, 0x13, 0x37]
    ///     }
    /// );
    /// # fs::remove_file("rom.db");
    /// ```
    pub fn set_stats<S: Into<PkmnapiDBPokedexID>>(
        &self,
        pokedex_id: S,
        stats: PkmnapiDBStats,
    ) -> Result<PkmnapiDBPatch, String> {
        let pokedex_id = pokedex_id.into();

        if pokedex_id < 1 {
            return Err(format!("Pokédex ID too low: {}", pokedex_id));
        }

        let offset_base = ROM_PAGE * 0x1C;
        let offset = (offset_base + 0x03DE) + ((pokedex_id - 1) * 0x1C);

        let stats_raw = stats.to_raw();

        Ok(PkmnapiDBPatch::new(offset, stats_raw))
    }

    /// Get Pokémon name by Pokédex ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi::db::string::*;
    /// use pkmnapi::db::types::*;
    /// use pkmnapi::db::*;
    /// use std::fs;
    /// # use std::fs::File;
    /// # use std::io::prelude::*;
    /// # let mut file = File::create("rom.db").unwrap();
    /// # let data: Vec<u8> = [
    /// #     vec![0x00; 0x1C21E],
    /// #     vec![0x91, 0x87, 0x98, 0x83, 0x8E, 0x8D, 0x50, 0x50, 0x50, 0x50],
    /// #     vec![0x00; 0x24DFC],
    /// #     vec![0x70],
    /// #     vec![0x00; 0xBD]
    /// # ].concat();
    /// # file.write_all(&data).unwrap();
    ///
    /// let rom = fs::read("rom.db").unwrap();
    /// let db = PkmnapiDB::new(&rom).unwrap();
    ///
    /// let pokemon_name = db.get_pokemon_name(112).unwrap();
    ///
    /// assert_eq!(
    ///     pokemon_name,
    ///     PkmnapiDBPokemonName {
    ///         name: PkmnapiDBString::from("RHYDON")
    ///     }
    /// );
    /// # fs::remove_file("rom.db");
    /// ```
    pub fn get_pokemon_name<S: Into<PkmnapiDBPokedexID>>(
        &self,
        pokedex_id: S,
    ) -> Result<PkmnapiDBPokemonName, String> {
        let pokedex_id = pokedex_id.into();

        if pokedex_id < 1 {
            return Err(format!("Pokédex ID too low: {}", pokedex_id));
        }

        let internal_id = self.pokedex_id_to_internal_id(pokedex_id).unwrap();

        let offset_base = ROM_PAGE * 0x0E;
        let offset = (offset_base + 0x021E) + (internal_id * 0x0A);

        let pokemon_name = PkmnapiDBPokemonName::from(&self.rom[offset..(offset + 0x0A)]);

        Ok(pokemon_name)
    }

    /// Set Pokémon name by Pokédex ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi::db::patch::*;
    /// use pkmnapi::db::string::*;
    /// use pkmnapi::db::types::*;
    /// use pkmnapi::db::*;
    /// use std::fs;
    /// # use std::fs::File;
    /// # use std::io::prelude::*;
    /// # let mut file = File::create("rom.db").unwrap();
    /// # let data: Vec<u8> = [
    /// #     vec![0x00; 0x1C21E],
    /// #     vec![0x91, 0x87, 0x98, 0x83, 0x8E, 0x8D, 0x50, 0x50, 0x50, 0x50],
    /// #     vec![0x00; 0x24DFC],
    /// #     vec![0x70],
    /// #     vec![0x00; 0xBD]
    /// # ].concat();
    /// # file.write_all(&data).unwrap();
    ///
    /// let rom = fs::read("rom.db").unwrap();
    /// let db = PkmnapiDB::new(&rom).unwrap();
    ///
    /// let patch = db
    ///     .set_pokemon_name(
    ///         112,
    ///         PkmnapiDBPokemonName {
    ///             name: PkmnapiDBString::from("ABC"),
    ///         },
    ///     )
    ///     .unwrap();
    ///
    /// assert_eq!(
    ///     patch,
    ///     PkmnapiDBPatch {
    ///         offset: 0x1C21E,
    ///         length: 0x0A,
    ///         data: vec![0x80, 0x81, 0x82, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50]
    ///     }
    /// );
    /// # fs::remove_file("rom.db");
    /// ```
    pub fn set_pokemon_name<S: Into<PkmnapiDBPokedexID>>(
        &self,
        pokedex_id: S,
        pokemon_name: PkmnapiDBPokemonName,
    ) -> Result<PkmnapiDBPatch, String> {
        let pokedex_id = pokedex_id.into();

        if pokedex_id < 1 {
            return Err(format!("Pokédex ID too low: {}", pokedex_id));
        }

        let internal_id = self.pokedex_id_to_internal_id(pokedex_id).unwrap();

        let offset_base = ROM_PAGE * 0x0E;
        let offset = (offset_base + 0x021E) + (internal_id * 0x0A);

        let pokemon_name_len = pokemon_name.name.value.len();
        let pokemon_name_raw = pokemon_name.to_raw();

        let data = [pokemon_name_raw, vec![0x50; 0x0A - pokemon_name_len]].concat();

        Ok(PkmnapiDBPatch::new(offset, data))
    }

    /// Get move stats by move ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi::db::types::*;
    /// use pkmnapi::db::*;
    /// use std::fs;
    /// # use std::fs::File;
    /// # use std::io::prelude::*;
    /// # let mut file = File::create("rom.db").unwrap();
    /// # let data: Vec<u8> = [
    /// #     vec![0x00; 0x38000],
    /// #     vec![0x01, 0x00, 0x28, 0x00, 0xFF, 0x23],
    /// # ].concat();
    /// # file.write_all(&data).unwrap();
    ///
    /// let rom = fs::read("rom.db").unwrap();
    /// let db = PkmnapiDB::new(&rom).unwrap();
    ///
    /// let move_stats = db.get_move_stats(1).unwrap();
    ///
    /// assert_eq!(
    ///     move_stats,
    ///     PkmnapiDBMoveStats {
    ///         move_id: PkmnapiDBMoveID::from(0x01),
    ///         effect: 0x00,
    ///         power: 0x28,
    ///         type_id: PkmnapiDBTypeID::from(0x00),
    ///         accuracy: 1.0,
    ///         pp: 0x23
    ///     }
    /// );
    /// # fs::remove_file("rom.db");
    /// ```
    pub fn get_move_stats<S: Into<PkmnapiDBMoveID>>(
        &self,
        move_id: S,
    ) -> Result<PkmnapiDBMoveStats, String> {
        let move_id = move_id.into();

        if move_id < 1 {
            return Err(format!("Move ID too low: {}", move_id));
        }

        let offset_base = ROM_PAGE * 0x1C;
        let offset = offset_base + ((move_id - 1) * 0x06);

        let move_stats = PkmnapiDBMoveStats::from(&self.rom[offset..(offset + 6)]);

        Ok(move_stats)
    }

    /// Set move stats by move ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi::db::patch::*;
    /// use pkmnapi::db::types::*;
    /// use pkmnapi::db::*;
    /// use std::fs;
    /// # use std::fs::File;
    /// # use std::io::prelude::*;
    /// # let mut file = File::create("rom.db").unwrap();
    /// # let data: Vec<u8> = [
    /// #     vec![0x00; 0x38000],
    /// #     vec![0x01, 0x00, 0x28, 0x00, 0xFF, 0x23],
    /// # ].concat();
    /// # file.write_all(&data).unwrap();
    ///
    /// let rom = fs::read("rom.db").unwrap();
    /// let db = PkmnapiDB::new(&rom).unwrap();
    ///
    /// let patch = db
    ///     .set_move_stats(
    ///         1,
    ///         PkmnapiDBMoveStats {
    ///             move_id: PkmnapiDBMoveID::from(0x01),
    ///             effect: 0x00,
    ///             power: 0xFF,
    ///             type_id: PkmnapiDBTypeID::from(0x01),
    ///             accuracy: 0.0,
    ///             pp: 0xFF,
    ///         },
    ///     )
    ///     .unwrap();
    ///
    /// assert_eq!(
    ///     patch,
    ///     PkmnapiDBPatch {
    ///         offset: 0x38000,
    ///         length: 0x06,
    ///         data: vec![0x01, 0x00, 0xFF, 0x01, 0x00, 0xFF]
    ///     }
    /// );
    /// # fs::remove_file("rom.db");
    /// ```
    pub fn set_move_stats<S: Into<PkmnapiDBMoveID>>(
        &self,
        move_id: S,
        move_stats: PkmnapiDBMoveStats,
    ) -> Result<PkmnapiDBPatch, String> {
        let move_id = move_id.into();

        if move_id < 1 {
            return Err(format!("Move ID too low: {}", move_id));
        }

        let offset_base = ROM_PAGE * 0x1C;
        let offset = offset_base + ((move_id - 1) * 0x06);

        let move_stats_raw = move_stats.to_raw();

        Ok(PkmnapiDBPatch::new(offset, move_stats_raw))
    }

    /// Get move name by move ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi::db::string::*;
    /// use pkmnapi::db::types::*;
    /// use pkmnapi::db::*;
    /// use std::fs;
    /// # use std::fs::File;
    /// # use std::io::prelude::*;
    /// # let mut file = File::create("rom.db").unwrap();
    /// # let data: Vec<u8> = [
    /// #     vec![0x00; 0xB0000],
    /// #     vec![0x8F, 0x8E, 0x94, 0x8D, 0x83, 0x50, 0x8A, 0x80, 0x91, 0x80, 0x93, 0x84, 0x7F]
    /// # ].concat();
    /// # file.write_all(&data).unwrap();
    ///
    /// let rom = fs::read("rom.db").unwrap();
    /// let db = PkmnapiDB::new(&rom).unwrap();
    ///
    /// let move_name = db.get_move_name(1).unwrap();
    ///
    /// assert_eq!(
    ///     move_name,
    ///     PkmnapiDBMoveName {
    ///         name: PkmnapiDBString::from("POUND")
    ///     }
    /// );
    /// # fs::remove_file("rom.db");
    /// ```
    pub fn get_move_name<S: Into<PkmnapiDBMoveID>>(
        &self,
        move_id: S,
    ) -> Result<PkmnapiDBMoveName, String> {
        let move_id = move_id.into();

        if move_id < 1 {
            return Err(format!("Invalid move ID: {}", move_id));
        }

        let offset_base = ROM_PAGE * 0x58;
        let offset = match {
            if move_id == 1 {
                Some(offset_base)
            } else {
                self.rom[offset_base..]
                    .iter()
                    .enumerate()
                    .filter_map(|(i, x)| {
                        if *x == 0x50 {
                            return Some(offset_base + i + 1);
                        }

                        None
                    })
                    .take(164)
                    .enumerate()
                    .filter_map(|(i, x)| {
                        if move_id.clone() - 2 == i {
                            return Some(x);
                        }

                        None
                    })
                    .next()
            }
        } {
            Some(offset) => offset,
            None => return Err(format!("Invalid move ID: {}", move_id)),
        };

        let move_name = PkmnapiDBMoveName::from(&self.rom[offset..(offset + 13)]);

        Ok(move_name)
    }

    /// Set move name by move ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi::db::patch::*;
    /// use pkmnapi::db::string::*;
    /// use pkmnapi::db::types::*;
    /// use pkmnapi::db::*;
    /// use std::fs;
    /// # use std::fs::File;
    /// # use std::io::prelude::*;
    /// # let mut file = File::create("rom.db").unwrap();
    /// # let data: Vec<u8> = [
    /// #     vec![0x00; 0xB0000],
    /// #     vec![0x8F, 0x8E, 0x94, 0x8D, 0x83, 0x50, 0x8A, 0x80, 0x91, 0x80, 0x93, 0x84, 0x7F]
    /// # ].concat();
    /// # file.write_all(&data).unwrap();
    ///
    /// let rom = fs::read("rom.db").unwrap();
    /// let db = PkmnapiDB::new(&rom).unwrap();
    ///
    /// let patch = db
    ///     .set_move_name(
    ///         1,
    ///         PkmnapiDBMoveName {
    ///             name: PkmnapiDBString::from("ABCDE"),
    ///         },
    ///     )
    ///     .unwrap();
    ///
    /// assert_eq!(
    ///     patch,
    ///     PkmnapiDBPatch {
    ///         offset: 0xB0000,
    ///         length: 0x05,
    ///         data: vec![0x80, 0x81, 0x82, 0x83, 0x084]
    ///     }
    /// );
    /// # fs::remove_file("rom.db");
    /// ```
    pub fn set_move_name<S: Into<PkmnapiDBMoveID>>(
        &self,
        move_id: S,
        move_name: PkmnapiDBMoveName,
    ) -> Result<PkmnapiDBPatch, String> {
        let move_id = move_id.into();
        let old_move_name = self.get_move_name(move_id.clone())?;
        let old_move_name = old_move_name.name.to_string();
        let old_move_name_len = old_move_name.len();
        let move_name_raw = move_name.to_raw();
        let move_name_len = move_name_raw.len();

        if old_move_name_len < move_name_len {
            return Err(format!(
                "Length mismatch: should be exactly {} characters, found {}",
                old_move_name_len, move_name_len
            ));
        }

        if move_id < 1 {
            return Err(format!("Invalid move ID: {}", move_id));
        }

        let offset_base = ROM_PAGE * 0x58;
        let offset = match {
            if move_id == 1 {
                Some(offset_base)
            } else {
                self.rom[offset_base..]
                    .iter()
                    .enumerate()
                    .filter_map(|(i, x)| {
                        if *x == 0x50 {
                            return Some(offset_base + i + 1);
                        }

                        None
                    })
                    .take(164)
                    .enumerate()
                    .filter_map(|(i, x)| {
                        if move_id.clone() - 2 == i {
                            return Some(x);
                        }

                        None
                    })
                    .next()
            }
        } {
            Some(offset) => offset,
            None => return Err(format!("Invalid move ID: {}", move_id)),
        };

        Ok(PkmnapiDBPatch::new(offset, move_name_raw))
    }

    /// Get HM by HM ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi::db::types::*;
    /// use pkmnapi::db::*;
    /// use std::fs;
    /// # use std::fs::File;
    /// # use std::io::prelude::*;
    /// # let mut file = File::create("rom.db").unwrap();
    /// # let data: Vec<u8> = [
    /// #     vec![0x00; 0x3052],
    /// #     vec![0x0F, 0x13, 0x39, 0x46, 0x94, 0xFF]
    /// # ].concat();
    /// # file.write_all(&data).unwrap();
    ///
    /// let rom = fs::read("rom.db").unwrap();
    /// let db = PkmnapiDB::new(&rom).unwrap();
    ///
    /// let hm = db.get_hm(1).unwrap();
    ///
    /// assert_eq!(
    ///     hm,
    ///     PkmnapiDBHM {
    ///         move_id: PkmnapiDBMoveID::from(0x0F)
    ///     }
    /// );
    /// # fs::remove_file("rom.db");
    /// ```
    pub fn get_hm<S: Into<PkmnapiDBHMID>>(&self, hm_id: S) -> Result<PkmnapiDBHM, String> {
        let hm_id = hm_id.into();

        let offset_base = ROM_PAGE * 0x01;
        let offset_base = offset_base + 0x1052;

        let max_id = (&self.rom[offset_base..])
            .iter()
            .position(|&r| r == 0xFF)
            .unwrap();

        if hm_id < 1 || hm_id > max_id as u8 {
            return Err(format!("Invalid ID: valid range is 1-{}", max_id));
        }

        let offset = (hm_id - 1) + offset_base;

        let hm = PkmnapiDBHM::from(self.rom[offset]);

        Ok(hm)
    }

    /// Set HM by HM ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi::db::patch::*;
    /// use pkmnapi::db::types::*;
    /// use pkmnapi::db::*;
    /// use std::fs;
    /// # use std::fs::File;
    /// # use std::io::prelude::*;
    /// # let mut file = File::create("rom.db").unwrap();
    /// # let data: Vec<u8> = [
    /// #     vec![0x00; 0x3052],
    /// #     vec![0x0F, 0x13, 0x39, 0x46, 0x94, 0xFF]
    /// # ].concat();
    /// # file.write_all(&data).unwrap();
    ///
    /// let rom = fs::read("rom.db").unwrap();
    /// let db = PkmnapiDB::new(&rom).unwrap();
    ///
    /// let patch = db
    ///     .set_hm(
    ///         1,
    ///         PkmnapiDBHM {
    ///             move_id: PkmnapiDBMoveID::from(0x42),
    ///         },
    ///     )
    ///     .unwrap();
    ///
    /// assert_eq!(
    ///     patch,
    ///     PkmnapiDBPatch {
    ///         offset: 0x3052,
    ///         length: 0x01,
    ///         data: vec![0x42]
    ///     }
    /// );
    /// # fs::remove_file("rom.db");
    /// ```
    pub fn set_hm<S: Into<PkmnapiDBHMID>>(
        &self,
        hm_id: S,
        hm: PkmnapiDBHM,
    ) -> Result<PkmnapiDBPatch, String> {
        let hm_id = hm_id.into();

        let offset_base = ROM_PAGE * 0x01;
        let offset_base = offset_base + 0x1052;

        let max_id = (&self.rom[offset_base..])
            .iter()
            .position(|&r| r == 0xFF)
            .unwrap();

        if hm_id < 1 || hm_id > max_id as u8 {
            return Err(format!("Invalid ID: valid range is 1-{}", max_id));
        }

        let offset = (hm_id - 1) + offset_base;

        Ok(PkmnapiDBPatch::new(offset, hm.to_raw()))
    }

    /// Get TM by TM ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi::db::types::*;
    /// use pkmnapi::db::*;
    /// use std::fs;
    /// # use std::fs::File;
    /// # use std::io::prelude::*;
    /// # let mut file = File::create("rom.db").unwrap();
    /// # let data: Vec<u8> = [
    /// #     vec![0x00; 0x13773],
    /// #     vec![0x05]
    /// # ].concat();
    /// # file.write_all(&data).unwrap();
    ///
    /// let rom = fs::read("rom.db").unwrap();
    /// let db = PkmnapiDB::new(&rom).unwrap();
    ///
    /// let tm = db.get_tm(1).unwrap();
    ///
    /// assert_eq!(
    ///     tm,
    ///     PkmnapiDBTM {
    ///         move_id: PkmnapiDBMoveID::from(0x05),
    ///     }
    /// );
    /// # fs::remove_file("rom.db");
    /// ```
    pub fn get_tm<S: Into<PkmnapiDBTMID>>(&self, tm_id: S) -> Result<PkmnapiDBTM, String> {
        let tm_id = tm_id.into();

        let offset_base = ROM_PAGE * 0x09;
        let offset_base = offset_base + 0x1773;

        let max_id = 50;

        if tm_id < 1 || tm_id > max_id {
            return Err(format!("Invalid ID: valid range is 1-{}", max_id));
        }

        let offset = (tm_id - 1) + offset_base;

        let tm = PkmnapiDBTM::from(self.rom[offset]);

        Ok(tm)
    }

    /// Set TM by TM ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi::db::patch::*;
    /// use pkmnapi::db::types::*;
    /// use pkmnapi::db::*;
    /// use std::fs;
    /// # use std::fs::File;
    /// # use std::io::prelude::*;
    /// # let mut file = File::create("rom.db").unwrap();
    /// # let data: Vec<u8> = [
    /// #     vec![0x00; 0x13773],
    /// #     vec![0x05]
    /// # ].concat();
    /// # file.write_all(&data).unwrap();
    ///
    /// let rom = fs::read("rom.db").unwrap();
    /// let db = PkmnapiDB::new(&rom).unwrap();
    ///
    /// let patch = db
    ///     .set_tm(
    ///         1,
    ///         PkmnapiDBTM {
    ///             move_id: PkmnapiDBMoveID::from(0x42),
    ///         },
    ///     )
    ///     .unwrap();
    ///
    /// assert_eq!(
    ///     patch,
    ///     PkmnapiDBPatch {
    ///         offset: 0x13773,
    ///         length: 0x01,
    ///         data: vec![0x42]
    ///     }
    /// );
    /// # fs::remove_file("rom.db");
    /// ```
    pub fn set_tm<S: Into<PkmnapiDBTMID>>(
        &self,
        tm_id: S,
        tm: PkmnapiDBTM,
    ) -> Result<PkmnapiDBPatch, String> {
        let tm_id = tm_id.into();

        let offset_base = ROM_PAGE * 0x09;
        let offset_base = offset_base + 0x1773;

        let max_id = 50;

        if tm_id < 1 || tm_id > max_id {
            return Err(format!("Invalid ID: valid range is 1-{}", max_id));
        }

        let offset = (tm_id - 1) + offset_base;

        Ok(PkmnapiDBPatch::new(offset, tm.to_raw()))
    }

    /// Get TM price by TM ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi::db::types::*;
    /// use pkmnapi::db::*;
    /// use std::fs;
    /// # use std::fs::File;
    /// # use std::io::prelude::*;
    /// # let mut file = File::create("rom.db").unwrap();
    /// # let data: Vec<u8> = [
    /// #     vec![0x00; 0x7BFA7],
    /// #     vec![0x32]
    /// # ].concat();
    /// # file.write_all(&data).unwrap();
    ///
    /// let rom = fs::read("rom.db").unwrap();
    /// let db = PkmnapiDB::new(&rom).unwrap();
    ///
    /// let tm_price = db.get_tm_price(1).unwrap();
    ///
    /// assert_eq!(tm_price, PkmnapiDBTMPrice { value: 3000 });
    /// # fs::remove_file("rom.db");
    /// ```
    pub fn get_tm_price<S: Into<PkmnapiDBTMID>>(
        &self,
        tm_id: S,
    ) -> Result<PkmnapiDBTMPrice, String> {
        let tm_id = tm_id.into();

        let offset_base = ROM_PAGE * 0x3D;
        let offset_base = offset_base + 0x1FA7;

        let max_id = 50;

        if tm_id < 1 || tm_id > max_id {
            return Err(format!("Invalid ID: valid range is 1-{}", max_id));
        }

        let offset = (((tm_id - 1) as f32 / 2.0) as usize) + offset_base;
        let value = {
            if ((tm_id - 1) % 2) == 0 {
                (self.rom[offset] & 0xF0) >> 4
            } else {
                self.rom[offset] & 0x0F
            }
        };

        let tm_price = PkmnapiDBTMPrice::from(value);

        Ok(tm_price)
    }

    /// Set TM price by TM ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi::db::patch::*;
    /// use pkmnapi::db::types::*;
    /// use pkmnapi::db::*;
    /// use std::fs;
    /// # use std::fs::File;
    /// # use std::io::prelude::*;
    /// # let mut file = File::create("rom.db").unwrap();
    /// # let data: Vec<u8> = [
    /// #     vec![0x00; 0x7BFA7],
    /// #     vec![0x32]
    /// # ].concat();
    /// # file.write_all(&data).unwrap();
    ///
    /// let rom = fs::read("rom.db").unwrap();
    /// let db = PkmnapiDB::new(&rom).unwrap();
    ///
    /// let patch = db
    ///     .set_tm_price(1, PkmnapiDBTMPrice { value: 9000 })
    ///     .unwrap();
    ///
    /// assert_eq!(
    ///     patch,
    ///     PkmnapiDBPatch {
    ///         offset: 0x7BFA7,
    ///         length: 0x01,
    ///         data: vec![0x92]
    ///     }
    /// );
    /// # fs::remove_file("rom.db");
    /// ```
    pub fn set_tm_price<S: Into<PkmnapiDBTMID>>(
        &self,
        tm_id: S,
        tm_price: PkmnapiDBTMPrice,
    ) -> Result<PkmnapiDBPatch, String> {
        let tm_id = tm_id.into();

        let offset_base = ROM_PAGE * 0x3D;
        let offset_base = offset_base + 0x1FA7;

        let max_id = 50;

        if tm_id < 1 || tm_id > max_id {
            return Err(format!("Invalid ID: valid range is 1-{}", max_id));
        }

        let offset = (((tm_id - 1) as f32 / 2.0) as usize) + offset_base;
        let value = {
            if ((tm_id - 1) % 2) == 0 {
                (self.rom[offset] & 0x0F) | (tm_price.to_raw()[0] << 0x04)
            } else {
                (self.rom[offset] & 0xF0) | (tm_price.to_raw()[0])
            }
        };

        Ok(PkmnapiDBPatch::new(offset, vec![value]))
    }

    /// Get Pokédex entry by Pokédex ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi::db::string::*;
    /// use pkmnapi::db::types::*;
    /// use pkmnapi::db::*;
    /// use std::fs;
    /// # use std::fs::File;
    /// # use std::io::prelude::*;
    /// # let mut file = File::create("rom.db").unwrap();
    /// # let data: Vec<u8> = [
    /// #     vec![0x00; 0x4047E],
    /// #     vec![0xFA, 0x45],
    /// #     vec![0x00; 0x17A],
    /// #     vec![0x83, 0x91, 0x88, 0x8B, 0x8B, 0x50, 0x06, 0x03, 0x5A, 0x0A, 0x00, 0x00, 0x00, 0x00, 0x00],
    /// #     vec![0x00; 0xA1B],
    /// #     vec![0x70],
    /// #     vec![0x00; 0xBE]
    /// # ].concat();
    /// # file.write_all(&data).unwrap();
    ///
    /// let rom = fs::read("rom.db").unwrap();
    /// let db = PkmnapiDB::new(&rom).unwrap();
    ///
    /// let pokedex_entry = db.get_pokedex_entry(112).unwrap();
    ///
    /// assert_eq!(pokedex_entry, PkmnapiDBPokedexEntry {
    ///     species: PkmnapiDBString::from("DRILL"),
    ///     height: 75,
    ///     weight: 2650
    /// });
    /// # fs::remove_file("rom.db");
    /// ```
    pub fn get_pokedex_entry<S: Into<PkmnapiDBPokedexID>>(
        &self,
        pokedex_id: S,
    ) -> Result<PkmnapiDBPokedexEntry, String> {
        let pokedex_id = pokedex_id.into();

        let internal_id = self.pokedex_id_to_internal_id(pokedex_id).unwrap();

        let offset_base = ROM_PAGE * 0x1E;
        let pointer_offset = (offset_base + 0x447E) + (internal_id * 2);

        let pointer = offset_base + {
            let mut cursor = Cursor::new(&self.rom[pointer_offset..(pointer_offset + 2)]);

            cursor.read_u16::<LittleEndian>().unwrap_or(0) as usize
        };

        let pokedex_entry = PkmnapiDBPokedexEntry::from(&self.rom[pointer..(pointer + 15)]);

        Ok(pokedex_entry)
    }

    /// Set Pokédex entry by Pokédex ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi::db::patch::*;
    /// use pkmnapi::db::string::*;
    /// use pkmnapi::db::types::*;
    /// use pkmnapi::db::*;
    /// use std::fs;
    /// # use std::fs::File;
    /// # use std::io::prelude::*;
    /// # let mut file = File::create("rom.db").unwrap();
    /// # let data: Vec<u8> = [
    /// #     vec![0x00; 0x4047E],
    /// #     vec![0xFA, 0x45],
    /// #     vec![0x00; 0x17A],
    /// #     vec![0x83, 0x91, 0x88, 0x8B, 0x8B, 0x50, 0x06, 0x03, 0x5A, 0x0A, 0x00, 0x00, 0x00, 0x00, 0x00],
    /// #     vec![0x00; 0xA1B],
    /// #     vec![0x70],
    /// #     vec![0x00; 0xBE]
    /// # ].concat();
    /// # file.write_all(&data).unwrap();
    ///
    /// let rom = fs::read("rom.db").unwrap();
    /// let db = PkmnapiDB::new(&rom).unwrap();
    ///
    /// let patch = db
    ///     .set_pokedex_entry(112, PkmnapiDBPokedexEntry {
    ///         species: PkmnapiDBString::from("BOBBY"),
    ///         height: 100,
    ///         weight: 300
    ///     })
    ///     .unwrap();
    ///
    /// assert_eq!(
    ///     patch,
    ///     PkmnapiDBPatch {
    ///         offset: 0x405FA,
    ///         length: 0x0A,
    ///         data: vec![0x81, 0x8E, 0x81, 0x81, 0x98, 0x50, 0x08, 0x04, 0x2C, 0x01]
    ///     }
    /// );
    /// # fs::remove_file("rom.db");
    /// ```
    pub fn set_pokedex_entry<S: Into<PkmnapiDBPokedexID>>(
        &self,
        pokedex_id: S,
        pokedex_entry: PkmnapiDBPokedexEntry,
    ) -> Result<PkmnapiDBPatch, String> {
        let pokedex_id = pokedex_id.into();

        let internal_id = self.pokedex_id_to_internal_id(pokedex_id).unwrap();

        let offset_base = ROM_PAGE * 0x1E;
        let pointer_offset = (offset_base + 0x447E) + (internal_id * 2);

        let pointer = offset_base + {
            let mut cursor = Cursor::new(&self.rom[pointer_offset..(pointer_offset + 2)]);

            cursor.read_u16::<LittleEndian>().unwrap_or(0) as usize
        };

        Ok(PkmnapiDBPatch::new(pointer, pokedex_entry.to_raw()))
    }

    /// Get Pokédex entry text by Pokédex ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi::db::string::*;
    /// use pkmnapi::db::types::*;
    /// use pkmnapi::db::*;
    /// use std::fs;
    /// # use std::fs::File;
    /// # use std::io::prelude::*;
    /// # let mut file = File::create("rom.db").unwrap();
    /// # let data: Vec<u8> = [
    /// #     vec![0x00; 0x4047E],
    /// #     vec![0xFA, 0x45],
    /// #     vec![0x00; 0x17A],
    /// #     vec![0x83, 0x91, 0x88, 0x8B, 0x8B, 0x50, 0x06, 0x03, 0x5A, 0x0A, 0x17, 0x00, 0x40, 0x2B, 0x50],
    /// #     vec![0x00; 0xA1B],
    /// #     vec![0x70],
    /// #     vec![0x00; 0xBE],
    /// #     vec![0x00; 0x6AF1D],
    /// #     vec![0x00, 0x8F, 0xB1, 0xAE, 0xB3, 0xA4, 0xA2, 0xB3, 0xA4, 0xA3, 0x5F]
    /// # ].concat();
    /// # file.write_all(&data).unwrap();
    ///
    /// let rom = fs::read("rom.db").unwrap();
    /// let db = PkmnapiDB::new(&rom).unwrap();
    ///
    /// let pokedex_entry_text = db.get_pokedex_entry_text(112).unwrap();
    ///
    /// assert_eq!(pokedex_entry_text, PkmnapiDBPokedexEntryText {
    ///     text: PkmnapiDBString::from("Protected"),
    /// });
    /// # fs::remove_file("rom.db");
    /// ```
    pub fn get_pokedex_entry_text<S: Into<PkmnapiDBPokedexID>>(
        &self,
        pokedex_id: S,
    ) -> Result<PkmnapiDBPokedexEntryText, String> {
        let pokedex_id = pokedex_id.into();

        let internal_id = self.pokedex_id_to_internal_id(pokedex_id).unwrap();

        let offset_base = ROM_PAGE * 0x1E;
        let pointer_offset = (offset_base + 0x447E) + (internal_id * 2);

        let pointer = offset_base + {
            let mut cursor = Cursor::new(&self.rom[pointer_offset..(pointer_offset + 2)]);

            cursor.read_u16::<LittleEndian>().unwrap_or(0) as usize
        };

        let pointer_offset =
            pointer + { self.rom[pointer..].iter().position(|&r| r == 0x50).unwrap() } + 0x06;

        let mut cursor = Cursor::new(&self.rom[pointer_offset..(pointer_offset + 3)]);

        let pointer = cursor.read_u16::<LittleEndian>().unwrap_or(0) as usize;
        let pointer_base = (ROM_PAGE * 2) * { cursor.read_u8().unwrap_or(0) as usize };
        let pointer = pointer + pointer_base - (ROM_PAGE * 2);

        let pokedex_entry_text = PkmnapiDBPokedexEntryText::from(&self.rom[pointer..]);

        Ok(pokedex_entry_text)
    }
}
