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

pub mod error;
pub mod header;
pub mod map;
pub mod patch;
pub mod pic;
pub mod sav;
pub mod string;
pub mod types;

use byteorder::{LittleEndian, ReadBytesExt};
use header::*;
use map::*;
use patch::*;
use pic::*;
use sav::*;
use std::io::Cursor;
use std::num::Wrapping;
use types::*;

pub const ROM_PAGE: usize = 0x2000;
const POKEMON_INTERNAL_MAX: usize = 190;

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
    pub fn new(rom: &Vec<u8>, sav: Option<Vec<u8>>) -> Result<PkmnapiDB, error::Error> {
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
    /// assert_eq!(
    ///     patch,
    ///     Patch {
    ///         offset: 0x014E,
    ///         length: 0x02,
    ///         data: vec![0x91, 0xE6]
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
    /// assert_eq!(db.verify_hash("3d45c1ee9abd5738df46d2bdda8b57dc"), true);
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
        let offset_base = ROM_PAGE * 0x0E;
        let offset = offset_base + 0x021E;

        return (0..POKEMON_INTERNAL_MAX)
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
    pub fn pokedex_id_to_internal_id(&self, pokedex_id: &u8) -> Result<u8, error::Error> {
        if pokedex_id < &1 {
            return Err(error::Error::PokedexIDInvalid(*pokedex_id));
        }

        let offset_base = ROM_PAGE * 0x20;
        let offset = offset_base + 0x1024;

        let internal_id = match (&self.rom[offset..(offset + POKEMON_INTERNAL_MAX)])
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
    pub fn internal_id_to_pokedex_id(&self, internal_id: &u8) -> Result<u8, error::Error> {
        if internal_id >= &(POKEMON_INTERNAL_MAX as u8) {
            return Err(error::Error::InternalIDInvalid(*internal_id));
        }

        let offset_base = ROM_PAGE * 0x20;
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
    pub fn type_id_validate(&self, type_id: &u8) -> Result<(usize, usize), error::Error> {
        let min_id = 0usize;

        let offset_base = ROM_PAGE * 0x10;
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
    pub fn type_effect_id_validate(
        &self,
        type_effect_id: &u8,
    ) -> Result<(usize, usize), error::Error> {
        let min_id = 0usize;

        let offset_base = ROM_PAGE * 0x1F;
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
    pub fn trainer_id_validate(&self, trainer_id: &u8) -> Result<(usize, usize), error::Error> {
        let min_id = 1usize;

        let offset_base = (ROM_PAGE * 0x1C) + 0x19FF;

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
    pub fn hm_id_validate(&self, hm_id: &u8) -> Result<(usize, usize), error::Error> {
        let min_id = 1usize;

        let offset_base = ROM_PAGE * 0x01;
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
    pub fn tm_id_validate(&self, tm_id: &u8) -> Result<(usize, usize), error::Error> {
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
    pub fn item_id_validate(&self, item_id: &u8) -> Result<(usize, usize), error::Error> {
        let min_id = 1usize;

        let offset_base = ROM_PAGE * 0x02;
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
    pub fn move_id_validate(&self, move_id: &u8) -> Result<(usize, usize), error::Error> {
        let min_id = 1usize;

        let offset_base = ROM_PAGE * 0x1C;

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

    /// Get type name by type ID
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
    /// let type_name = db.get_type_name(&0).unwrap();
    ///
    /// assert_eq!(
    ///     type_name,
    ///     TypeName {
    ///         name: ROMString::from("NORMAL")
    ///     }
    /// );
    /// ```
    pub fn get_type_name(&self, type_id: &u8) -> Result<TypeName, error::Error> {
        let _max_id = self.type_id_validate(type_id)?;

        let offset_base = ROM_PAGE * 0x10;
        let pointer_base = offset_base + 0x7DAE;
        let pointer_offset = pointer_base + ((*type_id as usize) * 2);
        let pointer = offset_base + {
            let mut cursor = Cursor::new(&self.rom[pointer_offset..(pointer_offset + 2)]);

            cursor.read_u16::<LittleEndian>().unwrap_or(0) as usize
        };

        let type_name = TypeName::from(&self.rom[pointer..=(pointer + 9)]);

        Ok(type_name)
    }

    /// Set type name by type ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::patch::*;
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
    /// let patch = db
    ///     .set_type_name(
    ///         &0,
    ///         &TypeName {
    ///             name: ROMString::from("BORING"),
    ///         },
    ///     )
    ///     .unwrap();
    ///
    /// assert_eq!(
    ///     patch,
    ///     Patch {
    ///         offset: 0x27DE4,
    ///         length: 0x06,
    ///         data: vec![0x81, 0x8E, 0x91, 0x88, 0x8D, 0x86]
    ///     }
    /// );
    /// ```
    pub fn set_type_name(&self, type_id: &u8, type_name: &TypeName) -> Result<Patch, error::Error> {
        let old_type_name = self.get_type_name(type_id)?;
        let old_type_name_len = old_type_name.name.value.len();
        let type_name_raw = type_name.to_raw();
        let type_name_len = type_name_raw.len();

        if old_type_name_len < type_name_len {
            return Err(error::Error::TypeNameWrongSize(
                old_type_name_len,
                type_name_len,
            ));
        }

        let offset_base = ROM_PAGE * 0x10;
        let pointer_offset = (offset_base + 0x7DAE) + ((*type_id as usize) * 2);
        let pointer = offset_base + {
            let mut cursor = Cursor::new(&self.rom[pointer_offset..(pointer_offset + 2)]);

            cursor.read_u16::<LittleEndian>().unwrap_or(0) as usize
        };

        let data = [type_name_raw, vec![0x50; old_type_name_len - type_name_len]].concat();

        Ok(Patch::new(&pointer, &data))
    }

    /// Get type effect by type effect ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::types::*;
    /// use pkmnapi_db::*;
    /// use std::fs;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    ///
    /// let type_effect = db.get_type_effect(&0).unwrap();
    ///
    /// assert_eq!(
    ///     type_effect,
    ///     TypeEffect {
    ///         attacking_type_id: 0x15,
    ///         defending_type_id: 0x14,
    ///         multiplier: 2.0
    ///     }
    /// );
    /// ```
    pub fn get_type_effect(&self, type_effect_id: &u8) -> Result<TypeEffect, error::Error> {
        let _max_id = self.type_effect_id_validate(type_effect_id)?;

        let offset_base = ROM_PAGE * 0x1F;
        let pointer = (offset_base + 0x0474) + ((*type_effect_id as usize) * 0x03);

        let type_effect = TypeEffect::from(&self.rom[pointer..(pointer + 3)]);

        Ok(type_effect)
    }

    /// Set type effect by type effect ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::patch::*;
    /// use pkmnapi_db::types::*;
    /// use pkmnapi_db::*;
    /// use std::fs;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    ///
    /// let patch = db
    ///     .set_type_effect(
    ///         &0,
    ///         &TypeEffect {
    ///             attacking_type_id: 0x13,
    ///             defending_type_id: 0x37,
    ///             multiplier: 0.5,
    ///         },
    ///     )
    ///     .unwrap();
    ///
    /// assert_eq!(
    ///     patch,
    ///     Patch {
    ///         offset: 0x3E474,
    ///         length: 0x03,
    ///         data: vec![0x13, 0x37, 0x05]
    ///     }
    /// );
    /// ```
    pub fn set_type_effect(
        &self,
        type_effect_id: &u8,
        type_effect: &TypeEffect,
    ) -> Result<Patch, error::Error> {
        let _max_id = self.type_effect_id_validate(type_effect_id)?;

        let offset_base = ROM_PAGE * 0x1F;
        let pointer = offset_base + 0x0474 + ((*type_effect_id as usize) * 3);

        let type_effect_raw = type_effect.to_raw();

        Ok(Patch::new(&pointer, &type_effect_raw))
    }

    /// Get Pokémon stats by Pokédex ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::types::*;
    /// use pkmnapi_db::*;
    /// use std::fs;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    ///
    /// let pokemon_stats = db.get_pokemon_stats(&1).unwrap();
    ///
    /// assert_eq!(
    ///     pokemon_stats,
    ///     PokemonStats {
    ///         pokedex_id: 1,
    ///         base_hp: 45,
    ///         base_attack: 49,
    ///         base_defence: 49,
    ///         base_speed: 45,
    ///         base_special: 65,
    ///         type_ids: vec![22, 3],
    ///         catch_rate: 45,
    ///         base_exp_yield: 64
    ///     }
    /// );
    /// ```
    pub fn get_pokemon_stats(&self, pokedex_id: &u8) -> Result<PokemonStats, error::Error> {
        let _internal_id = self.pokedex_id_to_internal_id(pokedex_id)?;

        let offset = {
            if pokedex_id == &151 {
                0x425B
            } else {
                let offset_base = ROM_PAGE * 0x1C;

                (offset_base + 0x03DE) + (((*pokedex_id as usize) - 1) * 0x1C)
            }
        };

        let pokemon_stats = PokemonStats::from(&self.rom[offset..(offset + 0x1C)]);

        Ok(pokemon_stats)
    }

    /// Set Pokémon stats by Pokédex ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::patch::*;
    /// use pkmnapi_db::types::*;
    /// use pkmnapi_db::*;
    /// use std::fs;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    ///
    /// let patch = db
    ///     .set_pokemon_stats(
    ///         &1,
    ///         &PokemonStats {
    ///             pokedex_id: 0x01,
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
    ///     patch,
    ///     Patch {
    ///         offset: 0x383DE,
    ///         length: 0x0A,
    ///         data: vec![0x01, 0x42, 0x13, 0x37, 0x13, 0x37, 0x13, 0x37, 0x13, 0x37]
    ///     }
    /// );
    /// ```
    pub fn set_pokemon_stats(
        &self,
        pokedex_id: &u8,
        pokemon_stats: &PokemonStats,
    ) -> Result<Patch, error::Error> {
        let _internal_id = self.pokedex_id_to_internal_id(pokedex_id)?;

        let offset_base = ROM_PAGE * 0x1C;
        let offset = (offset_base + 0x03DE) + (((*pokedex_id as usize) - 1) * 0x1C);

        let pokemon_stats_raw = pokemon_stats.to_raw();

        Ok(Patch::new(&offset, &pokemon_stats_raw))
    }

    /// Get Pokémon name by Pokédex ID
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
    /// let pokemon_name = db.get_pokemon_name(&1).unwrap();
    ///
    /// assert_eq!(
    ///     pokemon_name,
    ///     PokemonName {
    ///         name: ROMString::from("BULBASAUR")
    ///     }
    /// );
    /// ```
    pub fn get_pokemon_name(&self, pokedex_id: &u8) -> Result<PokemonName, error::Error> {
        let internal_id = self.pokedex_id_to_internal_id(pokedex_id)?;

        let offset_base = ROM_PAGE * 0x0E;
        let offset = (offset_base + 0x021E) + ((internal_id as usize) * 0x0A);

        let pokemon_name = PokemonName::from(&self.rom[offset..(offset + 0x0A)]);

        Ok(pokemon_name)
    }

    /// Set Pokémon name by Pokédex ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::patch::*;
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
    /// let patch = db
    ///     .set_pokemon_name(
    ///         &1,
    ///         &PokemonName {
    ///             name: ROMString::from("ABC"),
    ///         },
    ///     )
    ///     .unwrap();
    ///
    /// assert_eq!(
    ///     patch,
    ///     Patch {
    ///         offset: 0x1C80E,
    ///         length: 0x0A,
    ///         data: vec![0x80, 0x81, 0x82, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50]
    ///     }
    /// );
    /// ```
    pub fn set_pokemon_name(
        &self,
        pokedex_id: &u8,
        pokemon_name: &PokemonName,
    ) -> Result<Patch, error::Error> {
        let internal_id = self.pokedex_id_to_internal_id(pokedex_id)?;

        let offset_base = ROM_PAGE * 0x0E;
        let offset = (offset_base + 0x021E) + ((internal_id as usize) * 0x0A);

        let pokemon_name_len = pokemon_name.name.value.len();
        let pokemon_name_raw = pokemon_name.to_raw();

        let data = [pokemon_name_raw, vec![0x50; 0x0A - pokemon_name_len]].concat();

        Ok(Patch::new(&offset, &data))
    }

    /// Get move stats by move ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::types::*;
    /// use pkmnapi_db::*;
    /// use std::fs;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    ///
    /// let move_stats = db.get_move_stats(&1).unwrap();
    ///
    /// assert_eq!(
    ///     move_stats,
    ///     MoveStats {
    ///         move_id: 0x01,
    ///         effect: 0x00,
    ///         power: 0x28,
    ///         type_id: 0x00,
    ///         accuracy: 1.0,
    ///         pp: 0x23
    ///     }
    /// );
    /// ```
    pub fn get_move_stats(&self, move_id: &u8) -> Result<MoveStats, error::Error> {
        let (_min_id, _max_id) = self.move_id_validate(move_id)?;

        let offset_base = ROM_PAGE * 0x1C;
        let offset = offset_base + (((*move_id as usize) - 1) * 0x06);

        let move_stats = MoveStats::from(&self.rom[offset..(offset + 6)]);

        Ok(move_stats)
    }

    /// Set move stats by move ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::patch::*;
    /// use pkmnapi_db::types::*;
    /// use pkmnapi_db::*;
    /// use std::fs;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    ///
    /// let patch = db
    ///     .set_move_stats(
    ///         &1,
    ///         &MoveStats {
    ///             move_id: 0x01,
    ///             effect: 0x00,
    ///             power: 0xFF,
    ///             type_id: 0x01,
    ///             accuracy: 0.0,
    ///             pp: 0xFF,
    ///         },
    ///     )
    ///     .unwrap();
    ///
    /// assert_eq!(
    ///     patch,
    ///     Patch {
    ///         offset: 0x38000,
    ///         length: 0x06,
    ///         data: vec![0x01, 0x00, 0xFF, 0x01, 0x00, 0xFF]
    ///     }
    /// );
    /// ```
    pub fn set_move_stats(
        &self,
        move_id: &u8,
        move_stats: &MoveStats,
    ) -> Result<Patch, error::Error> {
        let (_min_id, _max_id) = self.move_id_validate(move_id)?;

        let offset_base = ROM_PAGE * 0x1C;
        let offset = offset_base + (((*move_id as usize) - 1) * 0x06);

        let move_stats_raw = move_stats.to_raw();

        Ok(Patch::new(&offset, &move_stats_raw))
    }

    /// Get move name by move ID
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
    /// let move_name = db.get_move_name(&1).unwrap();
    ///
    /// assert_eq!(
    ///     move_name,
    ///     MoveName {
    ///         name: ROMString::from("POUND")
    ///     }
    /// );
    /// ```
    pub fn get_move_name(&self, move_id: &u8) -> Result<MoveName, error::Error> {
        let (min_id, max_id) = self.move_id_validate(move_id)?;

        let offset_base = ROM_PAGE * 0x58;
        let offset = match {
            if move_id == &1 {
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
                    .take(max_id)
                    .enumerate()
                    .filter_map(|(i, x)| {
                        if (*move_id as usize) - 2 == i {
                            return Some(x);
                        }

                        None
                    })
                    .next()
            }
        } {
            Some(offset) => offset,
            None => return Err(error::Error::MoveIDInvalid(*move_id, min_id, max_id)),
        };

        let move_name = MoveName::from(&self.rom[offset..(offset + 13)]);

        Ok(move_name)
    }

    /// Set move name by move ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::patch::*;
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
    /// let patch = db
    ///     .set_move_name(
    ///         &1,
    ///         &MoveName {
    ///             name: ROMString::from("ABCDE"),
    ///         },
    ///     )
    ///     .unwrap();
    ///
    /// assert_eq!(
    ///     patch,
    ///     Patch {
    ///         offset: 0xB0000,
    ///         length: 0x05,
    ///         data: vec![0x80, 0x81, 0x82, 0x83, 0x084]
    ///     }
    /// );
    /// ```
    pub fn set_move_name(&self, move_id: &u8, move_name: &MoveName) -> Result<Patch, error::Error> {
        let (min_id, max_id) = self.move_id_validate(move_id)?;

        let old_move_name = self.get_move_name(move_id)?;
        let old_move_name_len = old_move_name.name.value.len();
        let move_name_raw = move_name.to_raw();
        let move_name_len = move_name_raw.len();

        if old_move_name_len != move_name_len {
            return Err(error::Error::MoveNameWrongSize(
                old_move_name_len,
                move_name_len,
            ));
        }

        let offset_base = ROM_PAGE * 0x58;
        let offset = match {
            if move_id == &1 {
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
                    .take(max_id)
                    .enumerate()
                    .filter_map(|(i, x)| {
                        if (*move_id as usize) - 2 == i {
                            return Some(x);
                        }

                        None
                    })
                    .next()
            }
        } {
            Some(offset) => offset,
            None => return Err(error::Error::MoveIDInvalid(*move_id, min_id, max_id)),
        };

        Ok(Patch::new(&offset, &move_name_raw))
    }

    /// Get HM by HM ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::types::*;
    /// use pkmnapi_db::*;
    /// use std::fs;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    ///
    /// let hm = db.get_hm(&1).unwrap();
    ///
    /// assert_eq!(hm, HM { move_id: 0x0F });
    /// ```
    pub fn get_hm(&self, hm_id: &u8) -> Result<HM, error::Error> {
        let _max_id = self.hm_id_validate(hm_id)?;

        let offset_base = ROM_PAGE * 0x01;
        let offset = (offset_base + 0x1052) + ((*hm_id as usize) - 1);

        let hm = HM::from(self.rom[offset]);

        Ok(hm)
    }

    /// Set HM by HM ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::patch::*;
    /// use pkmnapi_db::types::*;
    /// use pkmnapi_db::*;
    /// use std::fs;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    ///
    /// let patch = db.set_hm(&1, &HM { move_id: 0x42 }).unwrap();
    ///
    /// assert_eq!(
    ///     patch,
    ///     Patch {
    ///         offset: 0x3052,
    ///         length: 0x01,
    ///         data: vec![0x42]
    ///     }
    /// );
    /// ```
    pub fn set_hm(&self, hm_id: &u8, hm: &HM) -> Result<Patch, error::Error> {
        let _max_id = self.hm_id_validate(hm_id)?;

        let offset_base = ROM_PAGE * 0x01;
        let offset = (offset_base + 0x1052) + ((*hm_id as usize) - 1);

        Ok(Patch::new(&offset, &hm.to_raw()))
    }

    /// Get TM by TM ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::types::*;
    /// use pkmnapi_db::*;
    /// use std::fs;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    ///
    /// let tm = db.get_tm(&1).unwrap();
    ///
    /// assert_eq!(tm, TM { move_id: 0x05 });
    /// ```
    pub fn get_tm(&self, tm_id: &u8) -> Result<TM, error::Error> {
        let _max_id = self.tm_id_validate(tm_id)?;

        let offset_base = ROM_PAGE * 0x09;
        let offset = (offset_base + 0x1773) + ((*tm_id as usize) - 1);

        let tm = TM::from(self.rom[offset]);

        Ok(tm)
    }

    /// Set TM by TM ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::patch::*;
    /// use pkmnapi_db::types::*;
    /// use pkmnapi_db::*;
    /// use std::fs;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    ///
    /// let patch = db.set_tm(&1, &TM { move_id: 0x42 }).unwrap();
    ///
    /// assert_eq!(
    ///     patch,
    ///     Patch {
    ///         offset: 0x13773,
    ///         length: 0x01,
    ///         data: vec![0x42]
    ///     }
    /// );
    /// ```
    pub fn set_tm(&self, tm_id: &u8, tm: &TM) -> Result<Patch, error::Error> {
        let _max_id = self.tm_id_validate(tm_id)?;

        let offset_base = ROM_PAGE * 0x09;
        let offset = (offset_base + 0x1773) + ((*tm_id as usize) - 1);

        Ok(Patch::new(&offset, &tm.to_raw()))
    }

    /// Get TM price by TM ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::types::*;
    /// use pkmnapi_db::*;
    /// use std::fs;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    ///
    /// let tm_price = db.get_tm_price(&1).unwrap();
    ///
    /// assert_eq!(tm_price, TMPrice { value: 3000 });
    /// ```
    pub fn get_tm_price(&self, tm_id: &u8) -> Result<TMPrice, error::Error> {
        let _max_id = self.tm_id_validate(tm_id)?;

        let offset_base = ROM_PAGE * 0x3D;
        let offset = (offset_base + 0x1FA7) + (((*tm_id as usize - 1) as f32 / 2.0) as usize);
        let value = {
            if ((tm_id - 1) % 2) == 0 {
                (self.rom[offset] & 0xF0) >> 4
            } else {
                self.rom[offset] & 0x0F
            }
        };

        let tm_price = TMPrice::from(value);

        Ok(tm_price)
    }

    /// Set TM price by TM ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::patch::*;
    /// use pkmnapi_db::types::*;
    /// use pkmnapi_db::*;
    /// use std::fs;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    ///
    /// let patch = db.set_tm_price(&1, &TMPrice { value: 9000 }).unwrap();
    ///
    /// assert_eq!(
    ///     patch,
    ///     Patch {
    ///         offset: 0x7BFA7,
    ///         length: 0x01,
    ///         data: vec![0x92]
    ///     }
    /// );
    /// ```
    pub fn set_tm_price(&self, tm_id: &u8, tm_price: &TMPrice) -> Result<Patch, error::Error> {
        let _max_id = self.tm_id_validate(tm_id)?;

        let offset_base = ROM_PAGE * 0x3D;
        let offset = (offset_base + 0x1FA7) + ((((*tm_id as usize) - 1) as f32 / 2.0) as usize);
        let value = {
            if ((tm_id - 1) % 2) == 0 {
                (self.rom[offset] & 0x0F) | (tm_price.to_raw()[0] << 0x04)
            } else {
                (self.rom[offset] & 0xF0) | (tm_price.to_raw()[0])
            }
        };

        Ok(Patch::new(&offset, &vec![value]))
    }

    /// Get Pokédex entry by Pokédex ID
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
    /// let pokedex_entry = db.get_pokedex_entry(&1).unwrap();
    ///
    /// assert_eq!(pokedex_entry, PokedexEntry {
    ///     species: ROMString::from("SEED"),
    ///     height: 28,
    ///     weight: 150
    /// });
    /// ```
    pub fn get_pokedex_entry(&self, pokedex_id: &u8) -> Result<PokedexEntry, error::Error> {
        let internal_id = self.pokedex_id_to_internal_id(pokedex_id)?;

        let offset_base = ROM_PAGE * 0x1E;
        let pointer_offset = (offset_base + 0x447E) + ((internal_id as usize) * 2);

        let pointer = offset_base + {
            let mut cursor = Cursor::new(&self.rom[pointer_offset..(pointer_offset + 2)]);

            cursor.read_u16::<LittleEndian>().unwrap_or(0) as usize
        };

        let pokedex_entry = PokedexEntry::from(&self.rom[pointer..(pointer + 15)]);

        Ok(pokedex_entry)
    }

    /// Set Pokédex entry by Pokédex ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::patch::*;
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
    /// let patch = db
    ///     .set_pokedex_entry(&1, &PokedexEntry {
    ///         species: ROMString::from("BLAH"),
    ///         height: 100,
    ///         weight: 300
    ///     })
    ///     .unwrap();
    ///
    /// assert_eq!(
    ///     patch,
    ///     Patch {
    ///         offset: 0x40E33,
    ///         length: 0x09,
    ///         data: vec![0x81, 0x8B, 0x80, 0x87, 0x50, 0x08, 0x04, 0x2C, 0x01]
    ///     }
    /// );
    /// ```
    pub fn set_pokedex_entry(
        &self,
        pokedex_id: &u8,
        pokedex_entry: &PokedexEntry,
    ) -> Result<Patch, error::Error> {
        let old_pokedex_entry_species = self.get_pokedex_entry(pokedex_id)?;
        let old_pokedex_entry_species_len = old_pokedex_entry_species.species.value.len();
        let pokedex_entry_species_len = pokedex_entry.species.value.len();

        if old_pokedex_entry_species_len != pokedex_entry_species_len {
            return Err(error::Error::PokedexEntrySpeciesWrongSize(
                old_pokedex_entry_species_len,
                pokedex_entry_species_len,
            ));
        }

        let internal_id = self.pokedex_id_to_internal_id(pokedex_id)?;

        let offset_base = ROM_PAGE * 0x1E;
        let pointer_offset = (offset_base + 0x447E) + ((internal_id as usize) * 2);

        let pointer = offset_base + {
            let mut cursor = Cursor::new(&self.rom[pointer_offset..(pointer_offset + 2)]);

            cursor.read_u16::<LittleEndian>().unwrap_or(0) as usize
        };

        Ok(Patch::new(&pointer, &pokedex_entry.to_raw()))
    }

    /// Get Pokédex entry text by Pokédex ID
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
    /// let pokedex_entry_text = db.get_pokedex_entry_text(&1).unwrap();
    ///
    /// assert_eq!(pokedex_entry_text, PokedexEntryText {
    ///     text: ROMString::from("A strange seed was\nplanted on its\nback at birth.¶The plant sprouts\nand grows with\nthis #MON"),
    /// });
    /// ```
    pub fn get_pokedex_entry_text(
        &self,
        pokedex_id: &u8,
    ) -> Result<PokedexEntryText, error::Error> {
        let internal_id = self.pokedex_id_to_internal_id(pokedex_id)?;

        let offset_base = ROM_PAGE * 0x1E;
        let pointer_offset = (offset_base + 0x447E) + ((internal_id as usize) * 2);

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

        let pokedex_entry_text = PokedexEntryText::from(&self.rom[pointer..]);

        Ok(pokedex_entry_text)
    }

    /// Set Pokédex entry text by Pokédex ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::patch::*;
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
    /// let patch = db
    ///     .set_pokedex_entry_text(&1, &PokedexEntryText {
    ///         text: ROMString::from("ABCDE"),
    ///     })
    ///     .unwrap();
    ///
    /// assert_eq!(
    ///     patch,
    ///     Patch {
    ///         offset: 0xAEE81,
    ///         length: 0x07,
    ///         data: vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
    ///     }
    /// );
    /// ```
    pub fn set_pokedex_entry_text(
        &self,
        pokedex_id: &u8,
        pokedex_entry_text: &PokedexEntryText,
    ) -> Result<Patch, error::Error> {
        let old_pokedex_entry_text = self.get_pokedex_entry_text(pokedex_id)?;
        let old_pokedex_entry_text_len = old_pokedex_entry_text.text.value.len();
        let pokedex_entry_text_raw = pokedex_entry_text.text.to_string();
        let pokedex_entry_text_len = pokedex_entry_text_raw.len();

        if pokedex_entry_text_len >= old_pokedex_entry_text_len {
            return Err(error::Error::PokedexEntryTextWrongSize(
                old_pokedex_entry_text_len,
                pokedex_entry_text_len,
            ));
        }

        let internal_id = self.pokedex_id_to_internal_id(pokedex_id)?;

        let offset_base = ROM_PAGE * 0x1E;
        let pointer_offset = (offset_base + 0x447E) + ((internal_id as usize) * 2);

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

        Ok(Patch::new(&pointer, &pokedex_entry_text.to_raw()))
    }

    /// Get Pokémon pic by Pokédex ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::types::*;
    /// use pkmnapi_db::*;
    /// use std::fs;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    ///
    /// let pokemon_pic = db.get_pokemon_pic(&1, &PokemonPicFace::FRONT).unwrap();
    ///
    /// assert_eq!(pokemon_pic.width, 5);
    /// assert_eq!(pokemon_pic.height, 5);
    /// assert_eq!(pokemon_pic.pixels.len(), 1600);
    /// ```
    pub fn get_pokemon_pic(
        &self,
        pokedex_id: &u8,
        pokemon_pic_face: &PokemonPicFace,
    ) -> Result<Pic, error::Error> {
        let internal_id = self.pokedex_id_to_internal_id(pokedex_id)?;

        let (offset, bank_offset) = {
            if pokedex_id == &151 {
                let offset_base = ROM_PAGE * 0x02;
                let offset = offset_base + 0x025B;
                let bank_offset = (self.rom[0x163A] - 1) * 0x02;

                (offset, bank_offset as usize)
            } else {
                let offset_base = ROM_PAGE * 0x1C;
                let offset = (offset_base + 0x03DE) + (((*pokedex_id as usize) - 1) * 0x1C);

                let bank_offset = match internal_id {
                    _ if internal_id < self.rom[0x1646] - 1 => self.rom[0x1648],
                    _ if internal_id < self.rom[0x164D] - 1 => self.rom[0x164F],
                    _ if internal_id < self.rom[0x1654] - 1 => self.rom[0x1656],
                    _ if internal_id < self.rom[0x165B] - 1 => self.rom[0x165D],
                    _ => self.rom[0x1661],
                };
                let bank_offset = (bank_offset - 1) * 0x02;

                (offset, bank_offset as usize)
            }
        };

        let mut cursor = Cursor::new(&self.rom[(offset + 11)..(offset + 15)]);

        let pointer_front = cursor.read_u16::<LittleEndian>().unwrap_or(0) as usize;
        let pointer_back = cursor.read_u16::<LittleEndian>().unwrap_or(0) as usize;

        let offset_base = ROM_PAGE * bank_offset;
        let offset_front = offset_base + pointer_front;
        let offset_back = offset_base + pointer_back;

        let pointer = match pokemon_pic_face {
            PokemonPicFace::FRONT => offset_front,
            PokemonPicFace::BACK => offset_back,
        };

        let pic = Pic::new(&self.rom[pointer..])?;

        Ok(pic)
    }

    /// Set Pokémon pic by Pokédex ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::patch::*;
    /// use pkmnapi_db::pic::*;
    /// use pkmnapi_db::types::*;
    /// use pkmnapi_db::*;
    /// use std::fs;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    ///
    /// let pokemon_pic = Pic::new(&vec![0x55]).unwrap();
    /// let patch = db.set_pokemon_pic(&1, &PokemonPicFace::FRONT, &pokemon_pic, PicEncodingMethod::THREE(0x01)).unwrap();
    ///
    /// assert_eq!(
    ///     patch,
    ///     Patch {
    ///         offset: 0x34000,
    ///         length: 0x07,
    ///         data: vec![0x55, 0xBF, 0xD2, 0x1D, 0xFE, 0x90, 0x80]
    ///     }
    /// );
    /// ```
    pub fn set_pokemon_pic(
        &self,
        pokedex_id: &u8,
        pokemon_pic_face: &PokemonPicFace,
        pic: &Pic,
        encoding_method: PicEncodingMethod,
    ) -> Result<Patch, error::Error> {
        let old_pixels = self.get_pokemon_pic(pokedex_id, pokemon_pic_face)?;
        let pixels = pic.encode(encoding_method);

        if pixels.len() > old_pixels.bytes + 1 {
            return Err(error::Error::PicTooLarge);
        }

        let internal_id = self.pokedex_id_to_internal_id(pokedex_id)?;

        let (offset, bank_offset) = {
            if pokedex_id == &151 {
                let offset_base = ROM_PAGE * 0x02;
                let offset = offset_base + 0x025B;
                let bank_offset = (self.rom[0x163A] - 1) * 0x02;

                (offset, bank_offset as usize)
            } else {
                let offset_base = ROM_PAGE * 0x1C;
                let offset = (offset_base + 0x03DE) + (((*pokedex_id as usize) - 1) * 0x1C);

                let bank_offset = match internal_id {
                    _ if internal_id < self.rom[0x1646] - 1 => self.rom[0x1648],
                    _ if internal_id < self.rom[0x164D] - 1 => self.rom[0x164F],
                    _ if internal_id < self.rom[0x1654] - 1 => self.rom[0x1656],
                    _ if internal_id < self.rom[0x165B] - 1 => self.rom[0x165D],
                    _ => self.rom[0x1661],
                };
                let bank_offset = (bank_offset - 1) * 0x02;

                (offset, bank_offset as usize)
            }
        };

        let mut cursor = Cursor::new(&self.rom[(offset + 11)..(offset + 15)]);

        let pointer_front = cursor.read_u16::<LittleEndian>().unwrap_or(0) as usize;
        let pointer_back = cursor.read_u16::<LittleEndian>().unwrap_or(0) as usize;

        let offset_base = ROM_PAGE * bank_offset;
        let offset_front = offset_base + pointer_front;
        let offset_back = offset_base + pointer_back;

        let pointer = match pokemon_pic_face {
            PokemonPicFace::FRONT => offset_front,
            PokemonPicFace::BACK => offset_back,
        };

        Ok(Patch::new(&pointer, &pixels))
    }

    /// Get trainer name by trainer ID
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
    /// let trainer_name = db.get_trainer_name(&1).unwrap();
    ///
    /// assert_eq!(
    ///     trainer_name,
    ///     TrainerName {
    ///         name: ROMString::from("YOUNGSTER")
    ///     }
    /// );
    /// ```
    pub fn get_trainer_name(&self, trainer_id: &u8) -> Result<TrainerName, error::Error> {
        let offset_base = (ROM_PAGE * 0x1C) + 0x19FF;

        let (min_id, max_id) = self.trainer_id_validate(trainer_id)?;

        let offset = match {
            if trainer_id == &1 {
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
                    .take(max_id - 1)
                    .enumerate()
                    .filter_map(|(i, x)| {
                        if (*trainer_id as usize) - 2 == i {
                            return Some(x);
                        }

                        None
                    })
                    .next()
            }
        } {
            Some(offset) => offset,
            None => return Err(error::Error::TrainerIDInvalid(*trainer_id, min_id, max_id)),
        };

        let trainer_name = TrainerName::from(&self.rom[offset..(offset + 13)]);

        Ok(trainer_name)
    }

    /// Set trainer name by trainer ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::patch::*;
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
    /// let patch = db
    ///     .set_trainer_name(
    ///         &1,
    ///         &TrainerName {
    ///             name: ROMString::from("ABCDEFGHI"),
    ///         },
    ///     )
    ///     .unwrap();
    ///
    /// assert_eq!(
    ///     patch,
    ///     Patch {
    ///         offset: 0x399FF,
    ///         length: 0x09,
    ///         data: vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88]
    ///     }
    /// );
    /// ```
    pub fn set_trainer_name(
        &self,
        trainer_id: &u8,
        trainer_name: &TrainerName,
    ) -> Result<Patch, error::Error> {
        let (min_id, max_id) = self.trainer_id_validate(trainer_id)?;

        let old_trainer_name = self.get_trainer_name(trainer_id)?;
        let old_trainer_name_len = old_trainer_name.name.value.len();
        let trainer_name_raw = trainer_name.to_raw();
        let trainer_name_len = trainer_name_raw.len();

        if old_trainer_name_len != trainer_name_len {
            return Err(error::Error::TrainerNameWrongSize(
                old_trainer_name_len,
                trainer_name_len,
            ));
        }

        let offset_base = (ROM_PAGE * 0x1C) + 0x19FF;

        let offset = match {
            if trainer_id == &1 {
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
                    .take(max_id - 1)
                    .enumerate()
                    .filter_map(|(i, x)| {
                        if (*trainer_id as usize) - 2 == i {
                            return Some(x);
                        }

                        None
                    })
                    .next()
            }
        } {
            Some(offset) => offset,
            None => return Err(error::Error::TrainerIDInvalid(*trainer_id, min_id, max_id)),
        };

        Ok(Patch::new(&offset, &trainer_name_raw))
    }

    pub fn get_trainer_pic(&self, trainer_id: &u8) -> Result<Pic, error::Error> {
        let offset_base = ROM_PAGE * 0x1C;
        let offset_base = offset_base + 0x1914;

        let (_min_id, _max_id) = self.trainer_id_validate(trainer_id)?;

        let offset = offset_base + (((*trainer_id - 1) as usize) * 0x05);

        let pointer_base = ROM_PAGE * 0x24;
        let pointer = pointer_base + {
            let mut cursor = Cursor::new(&self.rom[offset..(offset + 2)]);

            cursor.read_u16::<LittleEndian>().unwrap_or(0) as usize
        };

        let pic = Pic::new(&self.rom[pointer..])?;

        Ok(pic)
    }

    pub fn set_trainer_pic(
        &self,
        trainer_id: &u8,
        pic: &Pic,
        encoding_method: PicEncodingMethod,
    ) -> Result<Patch, error::Error> {
        let old_pixels = self.get_trainer_pic(trainer_id)?;
        let pixels = pic.encode(encoding_method);

        if pixels.len() > old_pixels.bytes + 1 {
            return Err(error::Error::PicTooLarge);
        }

        let offset_base = ROM_PAGE * 0x1C;
        let offset_base = offset_base + 0x1914;
        let offset = offset_base + (((*trainer_id - 1) as usize) * 0x05);

        let pointer_base = ROM_PAGE * 0x24;
        let pointer = pointer_base + {
            let mut cursor = Cursor::new(&self.rom[offset..(offset + 2)]);

            cursor.read_u16::<LittleEndian>().unwrap_or(0) as usize
        };

        Ok(Patch::new(&pointer, &pixels))
    }

    /// Get item name by item ID
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
    /// let item_name = db.get_item_name(&1).unwrap();
    ///
    /// assert_eq!(
    ///     item_name,
    ///     ItemName {
    ///         name: ROMString::from("MASTER BALL")
    ///     }
    /// );
    /// ```
    pub fn get_item_name(&self, item_id: &u8) -> Result<ItemName, error::Error> {
        let (min_id, max_id) = self.item_id_validate(item_id)?;

        let offset_base = ROM_PAGE * 0x02;
        let offset_base = offset_base + 0x072B;

        let offset = match {
            if item_id == &1 {
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
                    .take(max_id - 1)
                    .enumerate()
                    .filter_map(|(i, x)| {
                        if (*item_id as usize) - 2 == i {
                            return Some(x);
                        }

                        None
                    })
                    .next()
            }
        } {
            Some(offset) => offset,
            None => return Err(error::Error::ItemIDInvalid(*item_id, min_id, max_id)),
        };

        let item_name = ItemName::from(&self.rom[offset..(offset + 13)]);

        Ok(item_name)
    }

    /// Set item name by item ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::patch::*;
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
    /// let patch = db
    ///     .set_item_name(
    ///         &1,
    ///         &ItemName {
    ///             name: ROMString::from("CHEATERBALL"),
    ///         },
    ///     )
    ///     .unwrap();
    ///
    /// assert_eq!(
    ///     patch,
    ///     Patch {
    ///         offset: 0x472B,
    ///         length: 0x0B,
    ///         data: vec![0x82, 0x87, 0x84, 0x80, 0x93, 0x84, 0x91, 0x81, 0x80, 0x8B, 0x8B]
    ///     }
    /// );
    /// ```
    pub fn set_item_name(&self, item_id: &u8, item_name: &ItemName) -> Result<Patch, error::Error> {
        let (min_id, max_id) = self.item_id_validate(item_id)?;

        let old_item_name = self.get_item_name(item_id)?;
        let old_item_name_len = old_item_name.name.value.len();
        let item_name_raw = item_name.to_raw();
        let item_name_len = item_name_raw.len();

        if old_item_name_len != item_name_len {
            return Err(error::Error::ItemNameWrongSize(
                old_item_name_len,
                item_name_len,
            ));
        }

        let offset_base = ROM_PAGE * 0x02;
        let offset_base = offset_base + 0x072B;

        let offset = match {
            if item_id == &1 {
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
                    .take(max_id - 1)
                    .enumerate()
                    .filter_map(|(i, x)| {
                        if (*item_id as usize) - 2 == i {
                            return Some(x);
                        }

                        None
                    })
                    .next()
            }
        } {
            Some(offset) => offset,
            None => return Err(error::Error::ItemIDInvalid(*item_id, min_id, max_id)),
        };

        Ok(Patch::new(&offset, &item_name_raw))
    }

    pub fn get_map_pic(&self, map_id: &u8) -> Result<Map, error::Error> {
        let map_id_max = 0xF7;

        if map_id > &map_id_max {
            return Err(error::Error::MapInvalid(*map_id));
        }

        let bank_offset_base = ROM_PAGE * 0x06;
        let bank_offset = (bank_offset_base + 0x23D) + (*map_id as usize);
        let bank_id = self.rom[bank_offset];

        let bank = ((bank_id as usize) - 0x01) * (ROM_PAGE * 0x02);

        if bank == 0x00 {
            return Err(error::Error::MapInvalid(*map_id));
        }

        let header_offset = 0x01AE + ((*map_id as usize) * 0x02);
        let header_pointer = bank + {
            let mut cursor = Cursor::new(&self.rom[header_offset..(header_offset + 2)]);

            cursor.read_u16::<LittleEndian>().unwrap_or(0) as usize
        };

        let tileset = self.rom[header_pointer];

        let tileset_bank_pointer = 0xC7BE + ((tileset as usize) * 0x0C);
        let tileset_bank = ((self.rom[tileset_bank_pointer] as usize) - 0x01) * (ROM_PAGE * 0x02);
        let tileset_block_pointer = tileset_bank + {
            let mut cursor =
                Cursor::new(&self.rom[(tileset_bank_pointer + 1)..(tileset_bank_pointer + 3)]);

            cursor.read_u16::<LittleEndian>().unwrap_or(0) as usize
        };
        let tileset_graphics_pointer = tileset_bank + {
            let mut cursor =
                Cursor::new(&self.rom[(tileset_bank_pointer + 3)..(tileset_bank_pointer + 5)]);

            cursor.read_u16::<LittleEndian>().unwrap_or(0) as usize
        };

        let height = self.rom[header_pointer + 1] as u32;
        let width = self.rom[header_pointer + 2] as u32;

        if width >= 0xF6 || height >= 0xF0 {
            return Err(error::Error::MapInvalid(*map_id));
        }

        let blocks_pointer = bank + {
            let mut cursor = Cursor::new(&self.rom[(header_pointer + 3)..(header_pointer + 5)]);

            cursor.read_u16::<LittleEndian>().unwrap_or(0) as usize
        };

        let blocks_data =
            self.rom[blocks_pointer..(blocks_pointer + ((width * height) as usize))].to_vec();

        let _text_pointer = bank + {
            let mut cursor = Cursor::new(&self.rom[(header_pointer + 5)..(header_pointer + 7)]);

            cursor.read_u16::<LittleEndian>().unwrap_or(0) as usize
        };

        let _script_pointer = bank + {
            let mut cursor = Cursor::new(&self.rom[(header_pointer + 7)..(header_pointer + 9)]);

            cursor.read_u16::<LittleEndian>().unwrap_or(0) as usize
        };

        let _connections = self.rom[header_pointer + 9];

        let object_pointer = bank + {
            let mut cursor = Cursor::new(&self.rom[(header_pointer + 10)..(header_pointer + 12)]);

            cursor.read_u16::<LittleEndian>().unwrap_or(0) as usize
        };

        let _border_block = self.rom[object_pointer];

        let tiles: Vec<Vec<u8>> = (0..(16 * 6 * 8 * 8))
            .map(|tile_id| {
                let tile_offset = tileset_graphics_pointer + (tile_id * 0x10);

                self.rom[tile_offset..(tile_offset + 0x10)]
                    .to_vec()
                    .chunks(2)
                    .map(|chunk| {
                        let hi_byte =
                            (0..8).map(|bit| (chunk[0] & (0x01 << (7 - bit))) >> (7 - bit));
                        let lo_byte =
                            (0..8).map(|bit| (chunk[1] & (0x01 << (7 - bit))) >> (7 - bit));

                        hi_byte
                            .zip(lo_byte)
                            .map(|(hi_bit, lo_bit)| (hi_bit << 0x01) | lo_bit)
                            .collect::<Vec<u8>>()
                    })
                    .flatten()
                    .collect()
            })
            .collect();

        let map_tiles: Vec<Vec<u8>> = (0..(width * height * 4 * 4))
            .map(|i| {
                let x = i % (width * 4);
                let y = ((i as f32) / (width * 4) as f32) as u32;
                let block_index =
                    (((((y as f32) / 4.0) as u32) * width) + (((x as f32) / 4.0) as u32)) as usize;
                let block = blocks_data[block_index] as usize;
                let block_tile_index = ((i % 4) + ((y % 4) * 4)) as usize;

                let tile_id =
                    (self.rom[tileset_block_pointer + (block * 0x10) + block_tile_index]) as usize;

                tiles[tile_id].to_vec()
            })
            .collect();

        let map = Map::new(&width, &height, &map_tiles)?;

        Ok(map)
    }

    pub fn get_trainer_parties(&self, trainer_id: &u8) -> Result<Vec<Party>, error::Error> {
        let (min_id, max_id) = self.trainer_id_validate(trainer_id)?;

        let offset_base = ROM_PAGE * 0x1C;
        let offset = offset_base + 0x1D3B;

        let pointer_min_offset = offset + ((*trainer_id as usize) - 1) * 0x02;
        let pointer_min = (offset_base - (ROM_PAGE * 2)) + {
            let mut cursor = Cursor::new(&self.rom[pointer_min_offset..(pointer_min_offset + 2)]);

            cursor.read_u16::<LittleEndian>().unwrap_or(0) as usize
        };

        let pointer_max_offset = offset + (*trainer_id as usize) * 0x02;
        let pointer_max = (offset_base - (ROM_PAGE * 2)) + {
            let mut cursor = Cursor::new(&self.rom[pointer_max_offset..(pointer_max_offset + 2)]);

            cursor.read_u16::<LittleEndian>().unwrap_or(0) as usize
        };

        let data_size = if trainer_id == &(max_id as u8) {
            self.rom[pointer_min..]
                .iter()
                .position(|r| r == &0x00)
                .unwrap()
                + 0x01
        } else {
            pointer_max - pointer_min
        };

        let trainer_party_offsets: Vec<usize> = [
            vec![0x00],
            self.rom[pointer_min..(pointer_min + data_size)]
                .iter()
                .enumerate()
                .filter_map(|(i, x)| {
                    let offset = i + 1;

                    if offset == data_size {
                        return None;
                    }

                    if x == &0x00 {
                        Some(i + 1)
                    } else {
                        None
                    }
                })
                .collect::<Vec<usize>>(),
        ]
        .concat();

        if data_size == 0x00 {
            return Err(error::Error::TrainerIDInvalid(*trainer_id, min_id, max_id));
        }

        let trainer_parties: Vec<Party> = trainer_party_offsets
            .iter()
            .map(|trainer_party_offset| {
                let mut party = Party::from(
                    &self.rom[(pointer_min + trainer_party_offset)..(pointer_min + data_size)],
                );

                party.pokemon = party
                    .pokemon
                    .iter()
                    .map(|party_pokemon| {
                        PartyPokemon::new(
                            party_pokemon.level,
                            self.internal_id_to_pokedex_id(&party_pokemon.internal_id)
                                .unwrap(),
                        )
                    })
                    .collect();

                party
            })
            .collect();

        Ok(trainer_parties)
    }

    pub fn set_trainer_parties(
        &self,
        trainer_id: &u8,
        trainer_parties: &Vec<Party>,
    ) -> Result<Patch, error::Error> {
        let old_trainer_parties = self.get_trainer_parties(trainer_id)?;
        let old_trainer_parties_len = old_trainer_parties.len();
        let old_trainer_parties_data: Vec<u8> = old_trainer_parties
            .iter()
            .map(|old_trainer_party| old_trainer_party.to_raw())
            .flatten()
            .collect();
        let old_trainer_parties_data_len = old_trainer_parties_data.len();
        let trainer_parties_len = trainer_parties.len();
        let trainer_parties_data: Vec<u8> = trainer_parties
            .iter()
            .map(|trainer_party| {
                let new_trainer_party = Party {
                    level_type: trainer_party.level_type,
                    pokemon: trainer_party
                        .pokemon
                        .iter()
                        .map(|pokemon| PartyPokemon {
                            level: pokemon.level,
                            pokedex_id: pokemon.pokedex_id,
                            internal_id: self
                                .pokedex_id_to_internal_id(&pokemon.pokedex_id)
                                .unwrap()
                                + 1,
                        })
                        .collect(),
                };

                new_trainer_party.to_raw()
            })
            .flatten()
            .collect();
        let trainer_parties_data_len = trainer_parties_data.len();

        if old_trainer_parties_len != trainer_parties_len {
            return Err(error::Error::TrainerPartiesWrongSize(
                old_trainer_parties_len,
                trainer_parties_len,
            ));
        }

        if old_trainer_parties_data_len != trainer_parties_data_len {
            return Err(error::Error::TrainerPartiesWrongDataSize(
                old_trainer_parties_data_len,
                trainer_parties_data_len,
            ));
        }

        let offset_base = ROM_PAGE * 0x1C;
        let offset = offset_base + 0x1D3B;

        let pointer_offset = offset + ((*trainer_id as usize) - 1) * 0x02;
        let pointer = (offset_base - (ROM_PAGE * 2)) + {
            let mut cursor = Cursor::new(&self.rom[pointer_offset..(pointer_offset + 2)]);

            cursor.read_u16::<LittleEndian>().unwrap_or(0) as usize
        };

        Ok(Patch::new(&pointer, &trainer_parties_data))
    }
}
