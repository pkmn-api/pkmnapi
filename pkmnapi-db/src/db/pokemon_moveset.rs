use crate::error::{self, Result};
use crate::patch::*;
use crate::PkmnapiDB;
use std::collections::HashMap;

impl PkmnapiDB {
    pub fn get_pokemon_moveset_all(&self, pokedex_ids: &Vec<u8>) -> Result<HashMap<u8, Vec<u8>>> {
        self.get_all(pokedex_ids, |id| self.get_pokemon_moveset(id))
    }

    /// Get Pokémon moveset by Pokédex ID
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
    /// let pokemon_moveset = db.get_pokemon_moveset(&1).unwrap();
    ///
    /// assert_eq!(
    ///     pokemon_moveset,
    ///     vec![
    ///         0x21,
    ///         0x2D
    ///     ]
    /// );
    /// ```
    pub fn get_pokemon_moveset(&self, pokedex_id: &u8) -> Result<Vec<u8>> {
        let _internal_id = self.pokedex_id_to_internal_id(pokedex_id)?;
        let (_, max_pokedex_id) = self.pokedex_id_bounds();

        let offset = {
            if pokedex_id == &(max_pokedex_id as u8) {
                0x425B
            } else {
                let offset_base = PkmnapiDB::ROM_PAGE * 0x0E;

                (offset_base + 0x03DE) + (((*pokedex_id as usize) - 1) * 0x1C)
            }
        } + 0x0F;

        let pokemon_moveset = self.rom[offset..]
            .iter()
            .take(4)
            .filter_map(|move_id| {
                if move_id == &0x00 {
                    None
                } else {
                    Some(*move_id)
                }
            })
            .collect();

        Ok(pokemon_moveset)
    }

    /// Set Pokémon moveset by Pokédex ID
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
    /// let patch = db.set_pokemon_moveset(&1, &vec![
    ///     0x01, 0x02, 0x03, 0x04
    /// ]).unwrap();
    ///
    /// assert_eq!(
    ///     patch,
    ///     Patch {
    ///         offset: 0x383ED,
    ///         length: 0x04,
    ///         data: vec![0x01, 0x02, 0x03, 0x04]
    ///     }
    /// );
    /// ```
    pub fn set_pokemon_moveset(&self, pokedex_id: &u8, pokemon_moveset: &Vec<u8>) -> Result<Patch> {
        let old_pokemon_moveset = self.get_pokemon_moveset(pokedex_id)?;
        let old_pokemon_moveset_len = old_pokemon_moveset.len();
        let pokemon_moveset_len = 0x04;

        if old_pokemon_moveset_len > pokemon_moveset_len {
            return Err(error::Error::PokemonMovesetWrongSize(
                old_pokemon_moveset_len,
                pokemon_moveset_len,
            ));
        }

        let (_, max_pokedex_id) = self.pokedex_id_bounds();

        let offset = {
            if pokedex_id == &(max_pokedex_id as u8) {
                0x425B
            } else {
                let offset_base = PkmnapiDB::ROM_PAGE * 0x0E;

                (offset_base + 0x03DE) + (((*pokedex_id as usize) - 1) * 0x1C)
            }
        } + 0x0F;

        Ok(Patch::new(&offset, &pokemon_moveset))
    }
}
