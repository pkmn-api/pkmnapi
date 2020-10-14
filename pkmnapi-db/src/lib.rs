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
//! let db = PkmnapiDB::new(&rom, None).unwrap();
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
pub mod types;

mod db;

pub use db::*;

use error::Result;
use header::*;
use patch::*;
use sav::*;
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
/// let db = PkmnapiDB::new(&rom, None).unwrap();
/// ```
#[derive(Debug)]
pub struct PkmnapiDB {
    pub rom: Vec<u8>,
    pub sav: Option<Sav>,
    pub hash: String,
    pub header: Header,
}

impl PkmnapiDB {
    pub const ROM_PAGE: usize = 0x2000;

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
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    /// ```
    pub fn new(rom: &Vec<u8>, sav: Option<Vec<u8>>) -> Result<PkmnapiDB> {
        let hash = format!("{:x}", md5::compute(&rom));
        let header = Header::from(&rom)?;
        let rom = rom[..].to_vec();
        let sav = match sav {
            Some(sav) => Some(Sav::new(&sav)?),
            None => None,
        };

        Ok(PkmnapiDB {
            rom,
            sav,
            hash,
            header,
        })
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
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    ///
    /// assert_eq!(db.verify_checksum(), true);
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
    /// use pkmnapi_db::patch::*;
    /// use pkmnapi_db::*;
    /// use std::fs;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
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
        let rom = [&self.rom[..0x014E], &self.rom[0x0150..]].concat();
        let checksum = rom
            .iter()
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
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
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
    /// let mut db = PkmnapiDB::new(&rom, None).unwrap();
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
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    ///
    /// let pokemon_internal_max = db.pokemon_internal_max();
    ///
    /// assert_eq!(pokemon_internal_max, 190);
    /// ```
    pub fn pokemon_internal_max(&self) -> usize {
        let offset_base = PkmnapiDB::ROM_PAGE * 0x38;
        let offset = offset_base + 0x1E5F;

        (self.rom[offset] as usize) - 1
    }

    /// Pokémon name to Pokédex ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::string::*;
    /// use pkmnapi_db::types::*;
    /// use pkmnapi_db::*;
    /// use std::fs;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
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
        let offset_base = PkmnapiDB::ROM_PAGE * 0x0E;
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
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
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

        let offset_base = PkmnapiDB::ROM_PAGE * 0x20;
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
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
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

        let offset_base = PkmnapiDB::ROM_PAGE * 0x20;
        let offset = (offset_base + 0x1024) + (*internal_id as usize);

        Ok(self.rom[offset])
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
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    ///
    /// let type_id = 0;
    ///
    /// match db.type_id_validate(&type_id) {
    ///     Ok(min_max) => assert_eq!(min_max, (0, 26)),
    ///     Err(_) => unreachable!()
    /// };
    ///
    /// let type_id = 100;
    ///
    /// match db.type_id_validate(&type_id) {
    ///     Ok(_) => unreachable!(),
    ///     Err(e) => assert_eq!(e, error::Error::TypeIDInvalid(type_id, 0, 26))
    /// };
    /// ```
    pub fn type_id_validate(&self, type_id: &u8) -> Result<(usize, usize)> {
        let min_id = 0usize;

        let offset_base = PkmnapiDB::ROM_PAGE * 0x10;
        let pointer_base = offset_base + 0x7DAE;

        let max_index = (&self.rom[pointer_base..])
            .iter()
            .position(|&r| r == 0x8D)
            .unwrap();
        let max_id = (((max_index as f32) / 2.0) as usize) - 1;

        if *type_id > (max_id as u8) {
            return Err(error::Error::TypeIDInvalid(*type_id, min_id, max_id));
        }

        Ok((min_id, max_id))
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
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    ///
    /// let type_effect_id = 0;
    ///
    /// match db.type_effect_id_validate(&type_effect_id) {
    ///     Ok(min_max) => assert_eq!(min_max, (0, 81)),
    ///     Err(_) => unreachable!()
    /// };
    ///
    /// let type_effect_id = 100;
    ///
    /// match db.type_effect_id_validate(&type_effect_id) {
    ///     Ok(_) => unreachable!(),
    ///     Err(e) => assert_eq!(e, error::Error::TypeEffectIDInvalid(type_effect_id, 0, 81))
    /// };
    /// ```
    pub fn type_effect_id_validate(&self, type_effect_id: &u8) -> Result<(usize, usize)> {
        let min_id = 0usize;

        let offset_base = PkmnapiDB::ROM_PAGE * 0x1F;
        let pointer = offset_base + 0x0474;

        let max_index = (&self.rom[pointer..])
            .iter()
            .position(|&r| r == 0xFF)
            .unwrap();
        let max_id = (((max_index as f32) / 3.0) as usize) - 1;

        if *type_effect_id > (max_id as u8) {
            return Err(error::Error::TypeEffectIDInvalid(
                *type_effect_id,
                min_id,
                max_id,
            ));
        }

        Ok((min_id, max_id))
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
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    ///
    /// let trainer_id = 1;
    ///
    /// match db.trainer_id_validate(&trainer_id) {
    ///     Ok(min_max) => assert_eq!(min_max, (1, 47)),
    ///     Err(_) => unreachable!()
    /// };
    ///
    /// let trainer_id = 100;
    ///
    /// match db.trainer_id_validate(&trainer_id) {
    ///     Ok(_) => unreachable!(),
    ///     Err(e) => assert_eq!(e, error::Error::TrainerIDInvalid(trainer_id, 1, 47))
    /// };
    /// ```
    pub fn trainer_id_validate(&self, trainer_id: &u8) -> Result<(usize, usize)> {
        let min_id = 1usize;

        let offset_base = (PkmnapiDB::ROM_PAGE * 0x1C) + 0x19FF;

        let max_offset = (&self.rom[offset_base..])
            .iter()
            .position(|&r| r == 0x21)
            .unwrap();
        let max_id = (&self.rom[offset_base..(offset_base + max_offset)])
            .iter()
            .filter(|&x| *x == 0x50)
            .count();

        if *trainer_id < (min_id as u8) || *trainer_id > (max_id as u8) {
            return Err(error::Error::TrainerIDInvalid(*trainer_id, min_id, max_id));
        }

        Ok((min_id, max_id))
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
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    ///
    /// let hm_id = 1;
    ///
    /// match db.hm_id_validate(&hm_id) {
    ///     Ok(min_max) => assert_eq!(min_max, (1, 5)),
    ///     Err(_) => unreachable!()
    /// };
    ///
    /// let hm_id = 100;
    ///
    /// match db.hm_id_validate(&hm_id) {
    ///     Ok(_) => unreachable!(),
    ///     Err(e) => assert_eq!(e, error::Error::HMIDInvalid(hm_id, 1, 5))
    /// };
    /// ```
    pub fn hm_id_validate(&self, hm_id: &u8) -> Result<(usize, usize)> {
        let min_id = 1usize;

        let offset_base = PkmnapiDB::ROM_PAGE * 0x01;
        let offset_base = offset_base + 0x1052;

        let max_id = (&self.rom[offset_base..])
            .iter()
            .position(|&r| r == 0xFF)
            .unwrap();

        if *hm_id < (min_id as u8) || *hm_id > (max_id as u8) {
            return Err(error::Error::HMIDInvalid(*hm_id, min_id, max_id));
        }

        Ok((min_id, max_id))
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
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    ///
    /// let tm_id = 1;
    ///
    /// match db.tm_id_validate(&tm_id) {
    ///     Ok(min_max) => assert_eq!(min_max, (1, 50)),
    ///     Err(_) => unreachable!()
    /// };
    ///
    /// let tm_id = 100;
    ///
    /// match db.tm_id_validate(&tm_id) {
    ///     Ok(_) => unreachable!(),
    ///     Err(e) => assert_eq!(e, error::Error::TMIDInvalid(tm_id, 1, 50))
    /// };
    /// ```
    pub fn tm_id_validate(&self, tm_id: &u8) -> Result<(usize, usize)> {
        let min_id = 1usize;
        let max_id = 50usize;

        if *tm_id < (min_id as u8) || *tm_id > (max_id as u8) {
            return Err(error::Error::TMIDInvalid(*tm_id, min_id, max_id));
        }

        Ok((min_id, max_id))
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
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    ///
    /// let item_id = 1;
    ///
    /// match db.item_id_validate(&item_id) {
    ///     Ok(min_max) => assert_eq!(min_max, (1, 97)),
    ///     Err(_) => unreachable!()
    /// };
    ///
    /// let item_id = 100;
    ///
    /// match db.item_id_validate(&item_id) {
    ///     Ok(_) => unreachable!(),
    ///     Err(e) => assert_eq!(e, error::Error::ItemIDInvalid(item_id, 1, 97))
    /// };
    /// ```
    pub fn item_id_validate(&self, item_id: &u8) -> Result<(usize, usize)> {
        let min_id = 1usize;

        let offset_base = PkmnapiDB::ROM_PAGE * 0x02;
        let offset_base = offset_base + 0x072B;

        let max_offset = (&self.rom[offset_base..])
            .iter()
            .position(|&r| r == 0xD0)
            .unwrap();
        let max_id = (&self.rom[offset_base..(offset_base + max_offset)])
            .iter()
            .filter(|&x| *x == 0x50)
            .count();

        if *item_id < (min_id as u8) || (item_id - 1) >= (max_id as u8) {
            return Err(error::Error::ItemIDInvalid(*item_id, min_id, max_id));
        }

        Ok((min_id, max_id))
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
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    ///
    /// let move_id = 1;
    ///
    /// match db.move_id_validate(&move_id) {
    ///     Ok(min_max) => assert_eq!(min_max, (1, 165)),
    ///     Err(_) => unreachable!()
    /// };
    ///
    /// let move_id = 200;
    ///
    /// match db.move_id_validate(&move_id) {
    ///     Ok(_) => unreachable!(),
    ///     Err(e) => assert_eq!(e, error::Error::MoveIDInvalid(move_id, 1, 165))
    /// };
    /// ```
    pub fn move_id_validate(&self, move_id: &u8) -> Result<(usize, usize)> {
        let min_id = 1usize;

        let offset_base = PkmnapiDB::ROM_PAGE * 0x1C;

        let max_index = self.rom[offset_base..]
            .chunks(2)
            .position(|r| r == [0x01, 0x2D])
            .unwrap();
        let max_id = ((max_index as f32) / 3.0) as usize;

        if *move_id < (min_id as u8) || *move_id > (max_id as u8) {
            return Err(error::Error::MoveIDInvalid(*move_id, min_id, max_id));
        }

        Ok((min_id, max_id))
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
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    ///
    /// let map_id = 0;
    ///
    /// match db.map_id_validate(&map_id) {
    ///     Ok(min_max) => assert_eq!(min_max, (0, 247)),
    ///     Err(_) => unreachable!()
    /// };
    ///
    /// let map_id = 255;
    ///
    /// match db.map_id_validate(&map_id) {
    ///     Ok(_) => unreachable!(),
    ///     Err(e) => assert_eq!(e, error::Error::MapIDInvalid(map_id, 0, 247))
    /// };
    /// ```
    pub fn map_id_validate(&self, map_id: &u8) -> Result<(usize, usize)> {
        let min_id = 0usize;

        let offset_base = PkmnapiDB::ROM_PAGE * 0x06;
        let offset = offset_base + 0x0EEB;

        let max_id = self.rom[offset..]
            .chunks(2)
            .position(|r| r == [0xFF, 0xFF])
            .unwrap()
            - 1 as usize;

        if *map_id > (max_id as u8) {
            return Err(error::Error::MapIDInvalid(*map_id, min_id, max_id));
        }

        Ok((min_id, max_id))
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
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    ///
    /// let icon_id = 0;
    ///
    /// match db.icon_id_validate(&icon_id) {
    ///     Ok(min_max) => assert_eq!(min_max, (0, 9)),
    ///     Err(_) => unreachable!()
    /// };
    ///
    /// let icon_id = 100;
    ///
    /// match db.icon_id_validate(&icon_id) {
    ///     Ok(_) => unreachable!(),
    ///     Err(e) => assert_eq!(e, error::Error::IconIDInvalid(icon_id, 0, 9))
    /// };
    /// ```
    pub fn icon_id_validate(&self, icon_id: &u8) -> Result<(usize, usize)> {
        let min_id = 0usize;
        let max_id = 9usize;

        if icon_id > &(max_id as u8) {
            return Err(error::Error::IconIDInvalid(*icon_id, min_id, max_id));
        }

        Ok((min_id, max_id))
    }
}
