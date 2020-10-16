use crate::error::Result;
use crate::patch::*;
use crate::PkmnapiDB;
use byteorder::ReadBytesExt;
use std::collections::HashMap;
use std::io::{Cursor, Read};

impl PkmnapiDB {
    pub fn get_pokemon_stats_all(
        &self,
        pokedex_ids: &Vec<u8>,
    ) -> Result<HashMap<u8, PokemonStats>> {
        let pokemon_stats_all: HashMap<u8, PokemonStats> = pokedex_ids
            .iter()
            .map(|pokedex_id| {
                let pokemon_stats = self.get_pokemon_stats(pokedex_id)?;

                Ok((*pokedex_id, pokemon_stats))
            })
            .collect::<Result<HashMap<u8, PokemonStats>>>()?;

        Ok(pokemon_stats_all)
    }

    /// Get Pokémon stats by Pokédex ID
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
    pub fn get_pokemon_stats(&self, pokedex_id: &u8) -> Result<PokemonStats> {
        let _internal_id = self.pokedex_id_to_internal_id(pokedex_id)?;

        let offset = {
            if pokedex_id == &151 {
                0x425B
            } else {
                let offset_base = PkmnapiDB::ROM_PAGE * 0x1C;

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
    ) -> Result<Patch> {
        let _internal_id = self.pokedex_id_to_internal_id(pokedex_id)?;

        let offset_base = PkmnapiDB::ROM_PAGE * 0x1C;
        let offset = (offset_base + 0x03DE) + (((*pokedex_id as usize) - 1) * 0x1C);

        let pokemon_stats_raw = pokemon_stats.to_raw();

        Ok(Patch::new(&offset, &pokemon_stats_raw))
    }
}

/// Pokemon stats (HP, attack, defence, etc)
///
/// # Example
///
/// ```
/// use pkmnapi_db::*;
///
/// let rom = vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A];
/// let stats = PokemonStats::from(&rom[..]);
///
/// assert_eq!(
///     stats,
///     PokemonStats {
///         pokedex_id: 0x01,
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
pub struct PokemonStats {
    pub pokedex_id: u8,
    pub base_hp: u8,
    pub base_attack: u8,
    pub base_defence: u8,
    pub base_speed: u8,
    pub base_special: u8,
    pub type_ids: Vec<u8>,
    pub catch_rate: u8,
    pub base_exp_yield: u8,
}

impl From<&[u8]> for PokemonStats {
    /// Convert &[u8] to PokemonStats
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::*;
    ///
    /// let rom = vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A];
    /// let stats = PokemonStats::from(&rom[..]);
    ///
    /// assert_eq!(
    ///     stats,
    ///     PokemonStats {
    ///         pokedex_id: 0x01,
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

        let pokedex_id = cursor.read_u8().unwrap_or(0);
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

        PokemonStats {
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

impl PokemonStats {
    /// Convert stats to raw bytes
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::*;
    ///
    /// let stats = PokemonStats {
    ///     pokedex_id: 0x01,
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
            vec![
                self.pokedex_id,
                self.base_hp,
                self.base_attack,
                self.base_defence,
                self.base_speed,
                self.base_special,
            ],
            self.type_ids.to_vec(),
            vec![self.catch_rate, self.base_exp_yield],
        ]
        .concat()
    }
}
