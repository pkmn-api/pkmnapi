//! Pkmnapi database module
//!
//! # Example
//!
//! ```
//! use pkmnapi_db::*;
//! use std::fs;
//! # use std::env;
//! # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
//!
//! let rom = fs::read(rom_path).unwrap();
//! let db = PkmnapiDB::new(&rom).build().unwrap();
//! ```

pub mod cry;
pub mod error;
pub mod header;
pub mod img;
pub mod map;
pub mod patch;
pub mod pic;
pub mod sav;
pub mod string;

mod builder;
mod db;

pub use builder::*;
pub use db::*;

use byteorder::{LittleEndian, ReadBytesExt};
use error::Result;
use header::*;
use patch::*;
use sav::*;
use std::cmp;
use std::collections::HashMap;
use std::io::Cursor;
use std::num::Wrapping;

/// Pkmnapi database
///
/// # Example
///
/// ```
/// use pkmnapi_db::*;
/// use std::fs;
/// # use std::env;
/// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
///
/// let rom = fs::read(rom_path).unwrap();
/// let db = PkmnapiDB::new(&rom).build().unwrap();
/// ```
#[derive(Debug)]
pub struct PkmnapiDB {
    pub rom: Vec<u8>,
    pub sav: Option<Sav>,
    pub hash: String,
    pub header: Header,
}

impl PkmnapiDB {
    pub const ROM_PAGE: usize = 0x4000;

    /// Create new database
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::*;
    /// use std::fs;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom).build().unwrap();
    /// ```
    pub fn new(rom: &Vec<u8>) -> PkmnapiDBBuilder {
        PkmnapiDBBuilder {
            rom: rom.to_vec(),
            sav: None,
        }
    }

    /// Verify global checksum
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::*;
    /// use std::fs;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom).build().unwrap();
    ///
    /// assert_eq!(db.verify_checksum(), true);
    /// ```
    pub fn verify_checksum(&self) -> bool {
        let checksum = self.rom[..0x014E]
            .iter()
            .chain(self.rom[0x0150..].iter())
            .fold(Wrapping(0u16), |acc, x| acc + Wrapping(*x as u16));

        checksum.0 == self.header.global_checksum
    }

    /// Generate global checksum
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::patch::*;
    /// use pkmnapi_db::*;
    /// use std::fs;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom).build().unwrap();
    ///
    /// let patch = db.generate_checksum();
    ///
    /// // RED
    /// # #[cfg(feature = "PKMN_RED")]
    /// assert_eq!(
    ///     patch,
    ///     Patch {
    ///         offset: 0x014E,
    ///         length: 0x02,
    ///         data: vec![0x91, 0xE6]
    ///     }
    /// );
    ///
    /// // BLUE
    /// # #[cfg(not(feature = "PKMN_RED"))]
    /// assert_eq!(
    ///     patch,
    ///     Patch {
    ///         offset: 0x014E,
    ///         length: 0x02,
    ///         data: vec![0x9D, 0x0A]
    ///     }
    /// );
    /// ```
    pub fn generate_checksum(&self) -> Patch {
        let checksum = self.rom[..0x014E]
            .iter()
            .chain(self.rom[0x0150..].iter())
            .fold(Wrapping(0u16), |acc, x| acc + Wrapping(*x as u16));

        let checksum = checksum.0.to_be_bytes().to_vec();

        Patch::new(&0x014E, &checksum)
    }

    /// Verify ROM hash
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::*;
    /// use std::fs;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom).build().unwrap();
    ///
    /// // RED
    /// # #[cfg(feature = "PKMN_RED")]
    /// assert_eq!(db.verify_hash("3d45c1ee9abd5738df46d2bdda8b57dc"), true);
    ///
    /// // BLUE
    /// # #[cfg(not(feature = "PKMN_RED"))]
    /// assert_eq!(db.verify_hash("50927e843568814f7ed45ec4f944bd8b"), true);
    /// ```
    pub fn verify_hash<S: Into<String>>(&self, hash: S) -> bool {
        self.hash == hash.into()
    }

    /// Apply ROM patch
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::patch::*;
    /// use pkmnapi_db::*;
    /// use std::fs;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let mut db = PkmnapiDB::new(&rom).build().unwrap();
    ///
    /// assert_eq!(db.rom[..4], [0xFF, 0x00, 0x00, 0x00]);
    ///
    /// let patch = Patch::new(&0x00, &vec![0x13, 0x37]);
    ///
    /// db.apply_patch(patch);
    ///
    /// assert_eq!(db.rom[..4], [0x13, 0x37, 0x00, 0x00]);
    /// ```
    pub fn apply_patch<S: Into<Patch>>(&mut self, patch: S) {
        let patch = patch.into();

        self.rom = [
            &self.rom[..patch.offset],
            &patch.data[..],
            &self.rom[(patch.offset + patch.length)..],
        ]
        .concat();
    }

    fn get_all<T>(&self, ids: &Vec<u8>, func: impl Fn(&u8) -> Result<T>) -> Result<HashMap<u8, T>> {
        let all: HashMap<u8, T> = ids
            .iter()
            .map(|id| {
                let one = func(id)?;

                Ok((*id, one))
            })
            .collect::<Result<HashMap<u8, T>>>()?;

        Ok(all)
    }

    fn get_pointer(&self, offset: usize) -> usize {
        let mut cursor = Cursor::new(&self.rom[offset..(offset + 2)]);

        cursor.read_u16::<LittleEndian>().unwrap_or(0) as usize
    }

    fn get_tiles(&self, offset: usize, tile_count: usize, hi_bit: bool) -> Vec<Vec<u8>> {
        let hi_bit = if hi_bit { 0x01 } else { 0x00 };
        let lo_bit = hi_bit ^ 0x01;

        (0..tile_count)
            .map(|tile_id| {
                let tile_offset = offset + (tile_id * 0x10);

                self.rom[tile_offset..(tile_offset + 0x10)]
                    .to_vec()
                    .chunks(2)
                    .map(|chunk| {
                        let hi_byte =
                            (0..8).map(|bit| (chunk[hi_bit] & (0x01 << (7 - bit))) >> (7 - bit));
                        let lo_byte =
                            (0..8).map(|bit| (chunk[lo_bit] & (0x01 << (7 - bit))) >> (7 - bit));

                        hi_byte
                            .zip(lo_byte)
                            .map(|(hi_bit, lo_bit)| (hi_bit << 0x01) | lo_bit)
                            .collect::<Vec<u8>>()
                    })
                    .flatten()
                    .collect()
            })
            .collect()
    }

    /// Pokémon internal max
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::*;
    /// use std::fs;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom).build().unwrap();
    ///
    /// let pokemon_internal_max = db.pokemon_internal_max();
    ///
    /// assert_eq!(pokemon_internal_max, 190);
    /// ```
    pub fn pokemon_internal_max(&self) -> usize {
        let offset_base = PkmnapiDB::ROM_PAGE * 0x1C;
        let offset = offset_base + 0x1E5F;

        (self.rom[offset] as usize) - 1
    }

    /// Pokémon name to Pokédex ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::string::*;
    /// use pkmnapi_db::*;
    /// use std::fs;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom).build().unwrap();
    ///
    /// let pokemon_name = PokemonName {
    ///     name: ROMString::from("BULBASAUR"),
    /// };
    ///
    /// let pokedex_id = db.pokemon_name_to_pokedex_id(&pokemon_name).unwrap();
    ///
    /// assert_eq!(pokedex_id, 1);
    /// ```
    pub fn pokemon_name_to_pokedex_id(&self, pokemon_name: &PokemonName) -> Option<u8> {
        let offset_base = PkmnapiDB::ROM_PAGE * 0x07;
        let offset = offset_base + 0x021E;
        let pokemon_internal_max = self.pokemon_internal_max();

        return (0..pokemon_internal_max)
            .map(|i| offset + (i * 0x0A))
            .enumerate()
            .filter_map(|(internal_id, offset)| {
                let internal_id = internal_id as u8;
                let name = PokemonName::from(&self.rom[offset..(offset + 0x0A)]);

                if name == *pokemon_name {
                    let pokedex_id = self.internal_id_to_pokedex_id(&internal_id).unwrap();

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
    /// use pkmnapi_db::*;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom).build().unwrap();
    ///
    /// let pokedex_id = 151;
    /// let internal_id = db.pokedex_id_to_internal_id(&pokedex_id).unwrap();
    ///
    /// assert_eq!(internal_id, 0x14);
    /// ```
    pub fn pokedex_id_to_internal_id(&self, pokedex_id: &u8) -> Result<u8> {
        if pokedex_id < &1 {
            return Err(error::Error::PokedexIDInvalid(*pokedex_id));
        }

        let offset_base = PkmnapiDB::ROM_PAGE * 0x10;
        let offset = offset_base + 0x1024;
        let pokemon_internal_max = self.pokemon_internal_max();

        let internal_id = match (&self.rom[offset..(offset + pokemon_internal_max)])
            .iter()
            .position(|r| pokedex_id == r)
        {
            Some(internal_id) => internal_id,
            None => return Err(error::Error::PokedexIDInvalid(*pokedex_id)),
        };

        Ok(internal_id as u8)
    }

    /// Internal ID to Pokédex ID
    ///
    /// # Example
    ///
    /// ```
    /// use std::fs;
    /// use pkmnapi_db::*;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom).build().unwrap();
    ///
    /// let internal_id = 0x14;
    /// let pokedex_id = db.internal_id_to_pokedex_id(&internal_id).unwrap();
    ///
    /// assert_eq!(pokedex_id, 151);
    /// ```
    pub fn internal_id_to_pokedex_id(&self, internal_id: &u8) -> Result<u8> {
        let pokemon_internal_max = self.pokemon_internal_max();

        if internal_id >= &(pokemon_internal_max as u8) {
            return Err(error::Error::InternalIDInvalid(*internal_id));
        }

        let offset_base = PkmnapiDB::ROM_PAGE * 0x10;
        let offset = (offset_base + 0x1024) + (*internal_id as usize);

        Ok(self.rom[offset])
    }

    /// Pokédex ID bounds
    ///
    /// # Example
    ///
    /// ```
    /// use std::fs;
    /// use pkmnapi_db::*;
    /// use pkmnapi_db::error;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom).build().unwrap();
    ///
    /// let (min_pokedex_id, max_pokedex_id) = db.pokedex_id_bounds();
    ///
    /// assert_eq!((min_pokedex_id, max_pokedex_id), (1, 151));
    /// ```
    pub fn pokedex_id_bounds(&self) -> (usize, usize) {
        let mut min_id = 255;
        let mut max_id = 0;

        for internal_id in 0..self.pokemon_internal_max() {
            let pokedex_id = self
                .internal_id_to_pokedex_id(&(internal_id as u8))
                .unwrap();

            min_id = cmp::max(cmp::min(min_id, pokedex_id as usize), 1);
            max_id = cmp::max(max_id, pokedex_id as usize);
        }

        (min_id, max_id)
    }

    /// Validate Pokédex ID
    ///
    /// # Example
    ///
    /// ```
    /// use std::fs;
    /// use pkmnapi_db::*;
    /// use pkmnapi_db::error;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom).build().unwrap();
    ///
    /// let pokedex_id = 1;
    ///
    /// db.pokedex_id_validate(&pokedex_id).unwrap();
    ///
    /// let pokedex_id = 200;
    ///
    /// match db.pokedex_id_validate(&pokedex_id) {
    ///     Ok(_) => unreachable!(),
    ///     Err(e) => assert_eq!(e, error::Error::PokedexIDInvalid(pokedex_id))
    /// };
    /// ```
    pub fn pokedex_id_validate(&self, pokedex_id: &u8) -> Result<(usize, usize)> {
        let (min_id, max_id) = self.pokedex_id_bounds();

        if *pokedex_id > (max_id as u8) {
            return Err(error::Error::PokedexIDInvalid(*pokedex_id));
        }

        Ok((min_id, max_id))
    }

    /// Type ID bounds
    ///
    /// # Example
    ///
    /// ```
    /// use std::fs;
    /// use pkmnapi_db::*;
    /// use pkmnapi_db::error;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom).build().unwrap();
    ///
    /// let (min_type_id, max_type_id) = db.type_id_bounds();
    ///
    /// assert_eq!((min_type_id, max_type_id), (0, 26));
    /// ```
    pub fn type_id_bounds(&self) -> (usize, usize) {
        let min_id = 0usize;

        let offset_base = PkmnapiDB::ROM_PAGE * 0x08;
        let pointer_base = offset_base + 0x7DAE;

        let max_index = self.rom[pointer_base..]
            .iter()
            .position(|&r| r == 0x8D)
            .unwrap();
        let max_id = (((max_index as f32) / 2.0) as usize) - 1;

        (min_id, max_id)
    }

    /// Validate type ID
    ///
    /// # Example
    ///
    /// ```
    /// use std::fs;
    /// use pkmnapi_db::*;
    /// use pkmnapi_db::error;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom).build().unwrap();
    ///
    /// let type_id = 0;
    ///
    /// db.type_id_validate(&type_id).unwrap();
    ///
    /// let type_id = 100;
    ///
    /// match db.type_id_validate(&type_id) {
    ///     Ok(_) => unreachable!(),
    ///     Err(e) => assert_eq!(e, error::Error::TypeIDInvalid(type_id, 0, 26))
    /// };
    /// ```
    pub fn type_id_validate(&self, type_id: &u8) -> Result<(usize, usize)> {
        let (min_id, max_id) = self.type_id_bounds();

        if *type_id > (max_id as u8) {
            return Err(error::Error::TypeIDInvalid(*type_id, min_id, max_id));
        }

        Ok((min_id, max_id))
    }

    /// Type effect ID bounds
    ///
    /// # Example
    ///
    /// ```
    /// use std::fs;
    /// use pkmnapi_db::*;
    /// use pkmnapi_db::error;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom).build().unwrap();
    ///
    /// let (min_type_effect_id, max_type_effect_id) = db.type_effect_id_bounds();
    ///
    /// assert_eq!((min_type_effect_id, max_type_effect_id), (0, 81));
    /// ```
    pub fn type_effect_id_bounds(&self) -> (usize, usize) {
        let min_id = 0usize;

        let offset_base = PkmnapiDB::ROM_PAGE * 0x0F;
        let pointer = offset_base + 0x2474;

        let max_index = self.rom[pointer..].iter().position(|&r| r == 0xFF).unwrap();
        let max_id = (((max_index as f32) / 3.0) as usize) - 1;

        (min_id, max_id)
    }

    /// Validate type effect ID
    ///
    /// # Example
    ///
    /// ```
    /// use std::fs;
    /// use pkmnapi_db::*;
    /// use pkmnapi_db::error;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom).build().unwrap();
    ///
    /// let type_effect_id = 0;
    ///
    /// db.type_effect_id_validate(&type_effect_id).unwrap();
    ///
    /// let type_effect_id = 100;
    ///
    /// match db.type_effect_id_validate(&type_effect_id) {
    ///     Ok(_) => unreachable!(),
    ///     Err(e) => assert_eq!(e, error::Error::TypeEffectIDInvalid(type_effect_id, 0, 81))
    /// };
    /// ```
    pub fn type_effect_id_validate(&self, type_effect_id: &u8) -> Result<(usize, usize)> {
        let (min_id, max_id) = self.type_effect_id_bounds();

        if *type_effect_id > (max_id as u8) {
            return Err(error::Error::TypeEffectIDInvalid(
                *type_effect_id,
                min_id,
                max_id,
            ));
        }

        Ok((min_id, max_id))
    }

    /// Trainer ID bounds
    ///
    /// # Example
    ///
    /// ```
    /// use std::fs;
    /// use pkmnapi_db::*;
    /// use pkmnapi_db::error;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom).build().unwrap();
    ///
    /// let (min_trainer_id, max_trainer_id) = db.trainer_id_bounds();
    ///
    /// assert_eq!((min_trainer_id, max_trainer_id), (1, 47));
    /// ```
    pub fn trainer_id_bounds(&self) -> (usize, usize) {
        let min_id = 1usize;

        let offset_base = (PkmnapiDB::ROM_PAGE * 0x0E) + 0x19FF;

        let max_offset = self.rom[offset_base..]
            .iter()
            .position(|&r| r == 0x21)
            .unwrap();
        let max_id = self.rom[offset_base..(offset_base + max_offset)]
            .iter()
            .filter(|&x| *x == 0x50)
            .count();

        (min_id, max_id)
    }

    /// Validate trainer ID
    ///
    /// # Example
    ///
    /// ```
    /// use std::fs;
    /// use pkmnapi_db::*;
    /// use pkmnapi_db::error;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom).build().unwrap();
    ///
    /// let trainer_id = 1;
    ///
    /// db.trainer_id_validate(&trainer_id).unwrap();
    ///
    /// let trainer_id = 100;
    ///
    /// match db.trainer_id_validate(&trainer_id) {
    ///     Ok(_) => unreachable!(),
    ///     Err(e) => assert_eq!(e, error::Error::TrainerIDInvalid(trainer_id, 1, 47))
    /// };
    /// ```
    pub fn trainer_id_validate(&self, trainer_id: &u8) -> Result<(usize, usize)> {
        let (min_id, max_id) = self.trainer_id_bounds();

        if *trainer_id < (min_id as u8) || *trainer_id > (max_id as u8) {
            return Err(error::Error::TrainerIDInvalid(*trainer_id, min_id, max_id));
        }

        Ok((min_id, max_id))
    }

    /// HM ID bounds
    ///
    /// # Example
    ///
    /// ```
    /// use std::fs;
    /// use pkmnapi_db::*;
    /// use pkmnapi_db::error;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom).build().unwrap();
    ///
    /// let (min_hm_id, max_hm_id) = db.hm_id_bounds();
    ///
    /// assert_eq!((min_hm_id, max_hm_id), (1, 5));
    /// ```
    pub fn hm_id_bounds(&self) -> (usize, usize) {
        let min_id = 1usize;

        let offset_base = 0x3052;

        let max_id = self.rom[offset_base..]
            .iter()
            .position(|&r| r == 0xFF)
            .unwrap();

        (min_id, max_id)
    }

    /// Validate HM ID
    ///
    /// # Example
    ///
    /// ```
    /// use std::fs;
    /// use pkmnapi_db::*;
    /// use pkmnapi_db::error;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom).build().unwrap();
    ///
    /// let hm_id = 1;
    ///
    /// db.hm_id_validate(&hm_id).unwrap();
    ///
    /// let hm_id = 100;
    ///
    /// match db.hm_id_validate(&hm_id) {
    ///     Ok(_) => unreachable!(),
    ///     Err(e) => assert_eq!(e, error::Error::HMIDInvalid(hm_id, 1, 5))
    /// };
    /// ```
    pub fn hm_id_validate(&self, hm_id: &u8) -> Result<(usize, usize)> {
        let (min_id, max_id) = self.hm_id_bounds();

        if *hm_id < (min_id as u8) || *hm_id > (max_id as u8) {
            return Err(error::Error::HMIDInvalid(*hm_id, min_id, max_id));
        }

        Ok((min_id, max_id))
    }

    /// TM ID bounds
    ///
    /// # Example
    ///
    /// ```
    /// use std::fs;
    /// use pkmnapi_db::*;
    /// use pkmnapi_db::error;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom).build().unwrap();
    ///
    /// let (min_tm_id, max_tm_id) = db.tm_id_bounds();
    ///
    /// assert_eq!((min_tm_id, max_tm_id), (1, 50));
    /// ```
    pub fn tm_id_bounds(&self) -> (usize, usize) {
        let min_id = 1usize;
        let max_id = 50usize;

        (min_id, max_id)
    }

    /// Validate TM ID
    ///
    /// # Example
    ///
    /// ```
    /// use std::fs;
    /// use pkmnapi_db::*;
    /// use pkmnapi_db::error;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom).build().unwrap();
    ///
    /// let tm_id = 1;
    ///
    /// db.tm_id_validate(&tm_id).unwrap();
    ///
    /// let tm_id = 100;
    ///
    /// match db.tm_id_validate(&tm_id) {
    ///     Ok(_) => unreachable!(),
    ///     Err(e) => assert_eq!(e, error::Error::TMIDInvalid(tm_id, 1, 50))
    /// };
    /// ```
    pub fn tm_id_validate(&self, tm_id: &u8) -> Result<(usize, usize)> {
        let (min_id, max_id) = self.tm_id_bounds();

        if *tm_id < (min_id as u8) || *tm_id > (max_id as u8) {
            return Err(error::Error::TMIDInvalid(*tm_id, min_id, max_id));
        }

        Ok((min_id, max_id))
    }

    /// Item ID bounds
    ///
    /// # Example
    ///
    /// ```
    /// use std::fs;
    /// use pkmnapi_db::*;
    /// use pkmnapi_db::error;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom).build().unwrap();
    ///
    /// let (min_item_id, max_item_id) = db.item_id_bounds();
    ///
    /// assert_eq!((min_item_id, max_item_id), (1, 97));
    /// ```
    pub fn item_id_bounds(&self) -> (usize, usize) {
        let min_id = 1usize;

        let offset_base = PkmnapiDB::ROM_PAGE * 0x01;
        let offset_base = offset_base + 0x072B;

        let max_offset = self.rom[offset_base..]
            .iter()
            .position(|&r| r == 0xD0)
            .unwrap();
        let max_id = self.rom[offset_base..(offset_base + max_offset)]
            .iter()
            .filter(|&x| *x == 0x50)
            .count();

        (min_id, max_id)
    }

    /// Validate item ID
    ///
    /// # Example
    ///
    /// ```
    /// use std::fs;
    /// use pkmnapi_db::*;
    /// use pkmnapi_db::error;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom).build().unwrap();
    ///
    /// let item_id = 1;
    ///
    /// db.item_id_validate(&item_id).unwrap();
    ///
    /// let item_id = 100;
    ///
    /// match db.item_id_validate(&item_id) {
    ///     Ok(_) => unreachable!(),
    ///     Err(e) => assert_eq!(e, error::Error::ItemIDInvalid(item_id, 1, 97))
    /// };
    /// ```
    pub fn item_id_validate(&self, item_id: &u8) -> Result<(usize, usize)> {
        let (min_id, max_id) = self.item_id_bounds();

        if *item_id < (min_id as u8) || (item_id - 1) >= (max_id as u8) {
            return Err(error::Error::ItemIDInvalid(*item_id, min_id, max_id));
        }

        Ok((min_id, max_id))
    }

    /// Move ID bounds
    ///
    /// # Example
    ///
    /// ```
    /// use std::fs;
    /// use pkmnapi_db::*;
    /// use pkmnapi_db::error;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom).build().unwrap();
    ///
    /// let (min_move_id, max_move_id) = db.move_id_bounds();
    ///
    /// assert_eq!((min_move_id, max_move_id), (1, 165));
    /// ```
    pub fn move_id_bounds(&self) -> (usize, usize) {
        let min_id = 1usize;

        let offset_base = PkmnapiDB::ROM_PAGE * 0x0E;

        let max_index = self.rom[offset_base..]
            .chunks(2)
            .position(|r| r == [0x01, 0x2D])
            .unwrap();
        let max_id = ((max_index as f32) / 3.0) as usize;

        (min_id, max_id)
    }

    /// Validate move ID
    ///
    /// # Example
    ///
    /// ```
    /// use std::fs;
    /// use pkmnapi_db::*;
    /// use pkmnapi_db::error;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom).build().unwrap();
    ///
    /// let move_id = 1;
    ///
    /// db.move_id_validate(&move_id).unwrap();
    ///
    /// let move_id = 200;
    ///
    /// match db.move_id_validate(&move_id) {
    ///     Ok(_) => unreachable!(),
    ///     Err(e) => assert_eq!(e, error::Error::MoveIDInvalid(move_id, 1, 165))
    /// };
    /// ```
    pub fn move_id_validate(&self, move_id: &u8) -> Result<(usize, usize)> {
        let (min_id, max_id) = self.move_id_bounds();

        if *move_id < (min_id as u8) || *move_id > (max_id as u8) {
            return Err(error::Error::MoveIDInvalid(*move_id, min_id, max_id));
        }

        Ok((min_id, max_id))
    }

    /// Map ID bounds
    ///
    /// # Example
    ///
    /// ```
    /// use std::fs;
    /// use pkmnapi_db::*;
    /// use pkmnapi_db::error;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom).build().unwrap();
    ///
    /// let (min_map_id, max_map_id) = db.map_id_bounds();
    ///
    /// assert_eq!((min_map_id, max_map_id), (0, 247));
    /// ```
    pub fn map_id_bounds(&self) -> (usize, usize) {
        let min_id = 0usize;

        let offset_base = PkmnapiDB::ROM_PAGE * 0x03;
        let offset = offset_base + 0x0EEB;

        let max_id = self.rom[offset..]
            .chunks(2)
            .position(|r| r == [0xFF, 0xFF])
            .unwrap()
            - 1 as usize;

        (min_id, max_id)
    }

    /// Validate map ID
    ///
    /// # Example
    ///
    /// ```
    /// use std::fs;
    /// use pkmnapi_db::*;
    /// use pkmnapi_db::error;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom).build().unwrap();
    ///
    /// let map_id = 0;
    ///
    /// db.map_id_validate(&map_id).unwrap();
    ///
    /// let map_id = 255;
    ///
    /// match db.map_id_validate(&map_id) {
    ///     Ok(_) => unreachable!(),
    ///     Err(e) => assert_eq!(e, error::Error::MapIDInvalid(map_id, 0, 247))
    /// };
    /// ```
    pub fn map_id_validate(&self, map_id: &u8) -> Result<(usize, usize)> {
        let (min_id, max_id) = self.map_id_bounds();

        if *map_id > (max_id as u8) {
            return Err(error::Error::MapIDInvalid(*map_id, min_id, max_id));
        }

        Ok((min_id, max_id))
    }

    /// Icon ID bounds
    ///
    /// # Example
    ///
    /// ```
    /// use std::fs;
    /// use pkmnapi_db::*;
    /// use pkmnapi_db::error;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom).build().unwrap();
    ///
    /// let (min_icon_id, max_icon_id) = db.icon_id_bounds();
    ///
    /// assert_eq!((min_icon_id, max_icon_id), (0, 9));
    /// ```
    pub fn icon_id_bounds(&self) -> (usize, usize) {
        let min_id = 0usize;
        let max_id = 9usize;

        (min_id, max_id)
    }

    /// Validate icon ID
    ///
    /// # Example
    ///
    /// ```
    /// use std::fs;
    /// use pkmnapi_db::*;
    /// use pkmnapi_db::error;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom).build().unwrap();
    ///
    /// let icon_id = 0;
    ///
    /// db.icon_id_validate(&icon_id).unwrap();
    ///
    /// let icon_id = 100;
    ///
    /// match db.icon_id_validate(&icon_id) {
    ///     Ok(_) => unreachable!(),
    ///     Err(e) => assert_eq!(e, error::Error::IconIDInvalid(icon_id, 0, 9))
    /// };
    /// ```
    pub fn icon_id_validate(&self, icon_id: &u8) -> Result<(usize, usize)> {
        let (min_id, max_id) = self.icon_id_bounds();

        if icon_id > &(max_id as u8) {
            return Err(error::Error::IconIDInvalid(*icon_id, min_id, max_id));
        }

        Ok((min_id, max_id))
    }

    /// Mart ID bounds
    ///
    /// # Example
    ///
    /// ```
    /// use std::fs;
    /// use pkmnapi_db::*;
    /// use pkmnapi_db::error;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom).build().unwrap();
    ///
    /// let (min_mart_id, max_mart_id) = db.mart_id_bounds();
    ///
    /// assert_eq!((min_mart_id, max_mart_id), (0, 15));
    /// ```
    pub fn mart_id_bounds(&self) -> (usize, usize) {
        let offset_base = 0x2442;
        let min_id = 0usize;
        let mut max_id = 0usize;
        let mut max_index = 0;

        loop {
            if self.rom[offset_base + max_index] == 0xFF
                && self.rom[offset_base + max_index + 1] == 0x50
            {
                break;
            }

            if self.rom[offset_base + max_index] == 0xFE {
                max_id += 1;
            }

            max_index += 1;
        }

        (min_id, max_id - 1)
    }

    /// Validate mart ID
    ///
    /// # Example
    ///
    /// ```
    /// use std::fs;
    /// use pkmnapi_db::*;
    /// use pkmnapi_db::error;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom).build().unwrap();
    ///
    /// let mart_id = 0;
    ///
    /// db.mart_id_validate(&mart_id).unwrap();
    ///
    /// let mart_id = 100;
    ///
    /// match db.mart_id_validate(&mart_id) {
    ///     Ok(_) => unreachable!(),
    ///     Err(e) => assert_eq!(e, error::Error::MartIDInvalid(mart_id, 0, 15))
    /// };
    /// ```
    pub fn mart_id_validate(&self, mart_id: &u8) -> Result<(usize, usize)> {
        let (min_id, max_id) = self.mart_id_bounds();

        if mart_id > &(max_id as u8) {
            return Err(error::Error::MartIDInvalid(*mart_id, min_id, max_id));
        }

        Ok((min_id, max_id))
    }

    /// Trade ID bounds
    ///
    /// # Example
    ///
    /// ```
    /// use std::fs;
    /// use pkmnapi_db::*;
    /// use pkmnapi_db::error;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom).build().unwrap();
    ///
    /// let (min_trade_id, max_trade_id) = db.trade_id_bounds();
    ///
    /// assert_eq!((min_trade_id, max_trade_id), (0, 9));
    /// ```
    pub fn trade_id_bounds(&self) -> (usize, usize) {
        let min_id = 0usize;
        let max_id = 9usize;

        (min_id, max_id)
    }

    /// Validate trade ID
    ///
    /// # Example
    ///
    /// ```
    /// use std::fs;
    /// use pkmnapi_db::*;
    /// use pkmnapi_db::error;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom).build().unwrap();
    ///
    /// let trade_id = 0;
    ///
    /// db.trade_id_validate(&trade_id).unwrap();
    ///
    /// let trade_id = 100;
    ///
    /// match db.trade_id_validate(&trade_id) {
    ///     Ok(_) => unreachable!(),
    ///     Err(e) => assert_eq!(e, error::Error::TradeIDInvalid(trade_id, 0, 9))
    /// };
    /// ```
    pub fn trade_id_validate(&self, trade_id: &u8) -> Result<(usize, usize)> {
        let (min_id, max_id) = self.trade_id_bounds();

        if trade_id > &(max_id as u8) {
            return Err(error::Error::TradeIDInvalid(*trade_id, min_id, max_id));
        }

        Ok((min_id, max_id))
    }
}
