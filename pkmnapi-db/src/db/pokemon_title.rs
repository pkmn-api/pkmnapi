use crate::error::{self, Result};
use crate::patch::*;
use crate::PkmnapiDB;

impl PkmnapiDB {
    /// Get title Pokédex IDs
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
    /// let pokemon_title = db.get_pokemon_title().unwrap();
    ///
    /// // RED
    /// # #[cfg(feature = "PKMN_RED")]
    /// assert_eq!(
    ///     pokemon_title,
    ///     vec![
    ///         0xB0,
    ///         0xB1,
    ///         0x99,
    ///         0x70,
    ///         0x03,
    ///         0x1A,
    ///         0x54,
    ///         0x04,
    ///         0x01,
    ///         0x94,
    ///         0x19,
    ///         0x4C,
    ///         0x96,
    ///         0x22,
    ///         0xA3,
    ///         0x85,
    ///     ]
    /// );
    ///
    /// // BLUE
    /// # #[cfg(not(feature = "PKMN_RED"))]
    /// assert_eq!(
    ///     pokemon_title,
    ///     vec![
    ///         0xB1,
    ///         0xB0,
    ///         0x99,
    ///         0x39,
    ///         0x2B,
    ///         0x52,
    ///         0x28,
    ///         0xAB,
    ///         0x68,
    ///         0x84,
    ///         0xBA,
    ///         0x47,
    ///         0x46,
    ///         0xAA,
    ///         0x0E,
    ///         0x55
    ///     ]
    /// );
    /// ```
    pub fn get_pokemon_title(&self) -> Result<Vec<u8>> {
        let offset_base = PkmnapiDB::ROM_PAGE;
        let offset = offset_base + 0x0588;

        let pokemon_title = self.rom[offset..(offset + 0x10)].to_vec();

        Ok(pokemon_title)
    }

    /// Set title Pokédex IDs
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
    /// let patch = db.set_pokemon_title(&vec![
    ///     0x85,
    ///     0x85,
    ///     0x85,
    ///     0x85,
    ///     0x85,
    ///     0x85,
    ///     0x85,
    ///     0x85,
    ///     0x85,
    ///     0x85,
    ///     0x85,
    ///     0x85,
    ///     0x85,
    ///     0x85,
    ///     0x85,
    ///     0x85,
    /// ]).unwrap();
    ///
    /// assert_eq!(
    ///     patch,
    ///     Patch {
    ///         offset: 0x4588,
    ///         length: 0x10,
    ///         data: vec![0x85, 0x85, 0x85, 0x85, 0x85, 0x85, 0x85, 0x85, 0x85, 0x85, 0x85, 0x85, 0x85, 0x85, 0x85, 0x85]
    ///     }
    /// );
    /// ```
    pub fn set_pokemon_title(&self, pokemon_title: &Vec<u8>) -> Result<Patch> {
        let offset_base = PkmnapiDB::ROM_PAGE;
        let offset = offset_base + 0x0588;

        let data = pokemon_title.to_vec();
        let data_len = data.len();
        let max_len = 0x10usize;

        if data_len != max_len {
            return Err(error::Error::PokemonTitleWrongSize(max_len, data_len));
        }

        Ok(Patch::new(&offset, &data))
    }
}
