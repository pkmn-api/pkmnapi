use crate::error::{self, Result};
use crate::patch::*;
use crate::PkmnapiDB;
use std::collections::HashMap;

impl PkmnapiDB {
    pub fn get_pokemon_learnset_all(
        &self,
        pokedex_ids: &Vec<u8>,
    ) -> Result<HashMap<u8, Vec<PokemonLearnset>>> {
        self.get_all(pokedex_ids, |id| self.get_pokemon_learnset(id))
    }

    /// Get Pokémon learnset by Pokédex ID
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
    /// let pokemon_learnset = db.get_pokemon_learnset(&1).unwrap();
    ///
    /// assert_eq!(
    ///     pokemon_learnset,
    ///     vec![
    ///         PokemonLearnset::new(7, 73),
    ///         PokemonLearnset::new(13, 22),
    ///         PokemonLearnset::new(20, 77),
    ///         PokemonLearnset::new(27, 75),
    ///         PokemonLearnset::new(34, 74),
    ///         PokemonLearnset::new(41, 79),
    ///         PokemonLearnset::new(48, 76),
    ///     ]
    /// );
    /// ```
    pub fn get_pokemon_learnset(&self, pokedex_id: &u8) -> Result<Vec<PokemonLearnset>> {
        let offset_base = PkmnapiDB::ROM_PAGE * 0x0E;
        let offset = offset_base + 0x305C;

        let internal_id = self.pokedex_id_to_internal_id(pokedex_id)?;

        let pointer_offset = offset + ((internal_id as usize) * 0x02);
        let pointer = offset_base - PkmnapiDB::ROM_PAGE + self.get_pointer(pointer_offset);

        let position = self.rom[pointer..].iter().position(|&r| r == 0x00).unwrap() + 0x01;

        let pokemon_learnset = self.rom[(pointer + position)..]
            .chunks(2)
            .take_while(|chunk| chunk[0] != 0x00)
            .map(|chunk| PokemonLearnset::new(chunk[0], chunk[1]))
            .collect();

        Ok(pokemon_learnset)
    }

    /// Set Pokémon learnset by Pokédex ID
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
    /// let patch = db.set_pokemon_learnset(&1, &vec![
    ///     PokemonLearnset::new(1, 2),
    ///     PokemonLearnset::new(1, 2),
    ///     PokemonLearnset::new(1, 2),
    ///     PokemonLearnset::new(1, 2),
    ///     PokemonLearnset::new(1, 2),
    ///     PokemonLearnset::new(1, 2),
    ///     PokemonLearnset::new(1, 2),
    /// ]).unwrap();
    ///
    /// assert_eq!(
    ///     patch,
    ///     Patch {
    ///         offset: 0x3B848,
    ///         length: 0x0E,
    ///         data: vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02]
    ///     }
    /// );
    /// ```
    pub fn set_pokemon_learnset(
        &self,
        pokedex_id: &u8,
        pokemon_learnset: &Vec<PokemonLearnset>,
    ) -> Result<Patch> {
        let old_pokemon_learnset = self.get_pokemon_learnset(pokedex_id)?;
        let old_pokemon_learnset_data: Vec<u8> = old_pokemon_learnset
            .iter()
            .map(|pokemon_learnset| pokemon_learnset.to_raw())
            .flatten()
            .collect();
        let old_pokemon_learnset_data_len = old_pokemon_learnset_data.len();
        let pokemon_learnset_data: Vec<u8> = pokemon_learnset
            .iter()
            .map(|pokemon_learnset| pokemon_learnset.to_raw())
            .flatten()
            .collect();
        let pokemon_learnset_data_len = pokemon_learnset_data.len();

        if old_pokemon_learnset_data_len != pokemon_learnset_data_len {
            return Err(error::Error::PokemonLearnsetWrongSize(
                old_pokemon_learnset_data_len,
                pokemon_learnset_data_len,
            ));
        }

        let offset_base = PkmnapiDB::ROM_PAGE * 0x0E;
        let offset = offset_base + 0x305C;

        let internal_id = self.pokedex_id_to_internal_id(pokedex_id)?;

        let pointer_offset = offset + ((internal_id as usize) * 0x02);
        let pointer = offset_base - PkmnapiDB::ROM_PAGE + self.get_pointer(pointer_offset);

        let position = self.rom[pointer..].iter().position(|&r| r == 0x00).unwrap() + 0x01;

        let pointer = pointer + position;

        Ok(Patch::new(&pointer, &pokemon_learnset_data))
    }
}

/// Pokémon learnset
///
/// ```
/// use pkmnapi_db::*;
///
/// let pokemon_learnset = PokemonLearnset::new(1, 2);
///
/// assert_eq!(
///     pokemon_learnset,
///     PokemonLearnset {
///         level: 1,
///         move_id: 2
///     }
/// );
/// ```
#[derive(Debug, PartialEq)]
pub struct PokemonLearnset {
    pub level: u8,
    pub move_id: u8,
}

impl PokemonLearnset {
    /// Create new Pokémon learnset
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::*;
    ///
    /// let pokemon_learnset = PokemonLearnset::new(1, 2);
    ///
    /// assert_eq!(
    ///     pokemon_learnset,
    ///     PokemonLearnset {
    ///         level: 1,
    ///         move_id: 2
    ///     }
    /// );
    /// ```
    pub fn new(level: u8, move_id: u8) -> Self {
        PokemonLearnset { level, move_id }
    }

    /// Pokémon learnset to raw bytes
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::*;
    ///
    /// let pokemon_learnset = PokemonLearnset::new(1, 2);
    /// let raw = pokemon_learnset.to_raw();
    ///
    /// assert_eq!(raw, vec![0x01, 0x02]);
    /// ```
    pub fn to_raw(&self) -> Vec<u8> {
        vec![self.level, self.move_id]
    }
}
