use crate::error::Result;
use crate::patch::*;
use crate::PkmnapiDB;
use std::collections::HashMap;

impl PkmnapiDB {
    pub fn get_pokemon_icon_all(&self, pokedex_ids: &Vec<u8>) -> Result<HashMap<u8, PokemonIcon>> {
        let pokemon_icon_all: HashMap<u8, PokemonIcon> = pokedex_ids
            .iter()
            .map(|pokedex_id| {
                let pokemon_icon = self.get_pokemon_icon(pokedex_id)?;

                Ok((*pokedex_id, pokemon_icon))
            })
            .collect::<Result<HashMap<u8, PokemonIcon>>>()?;

        Ok(pokemon_icon_all)
    }

    /// Get Pokémon icon by Pokédex ID
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
    /// let pokemon_icon = db.get_pokemon_icon(&1).unwrap();
    ///
    /// assert_eq!(
    ///     pokemon_icon,
    ///     PokemonIcon {
    ///         icon_id: 0x07
    ///     }
    /// );
    /// ```
    pub fn get_pokemon_icon(&self, pokedex_id: &u8) -> Result<PokemonIcon> {
        let _internal_id = self.pokedex_id_to_internal_id(pokedex_id)?;

        let offset_base = PkmnapiDB::ROM_PAGE * 0x1C;
        let offset = (offset_base + 0x190D) + ((((*pokedex_id - 1) as f32) / 2.0).floor() as usize);

        let icon_id = if pokedex_id % 2 == 0 {
            self.rom[offset] & 0x0F
        } else {
            (self.rom[offset] & 0xF0) >> 0x04
        };

        let pokemon_icon = PokemonIcon::from(&icon_id);

        Ok(pokemon_icon)
    }

    /// Set Pokémon icon by Pokédex ID
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
    ///     .set_pokemon_icon(
    ///         &1,
    ///         &PokemonIcon::from(&0x02)
    ///     )
    ///     .unwrap();
    ///
    /// assert_eq!(
    ///     patch,
    ///     Patch {
    ///         offset: 0x7190D,
    ///         length: 0x01,
    ///         data: vec![0x27]
    ///     }
    /// );
    /// ```
    pub fn set_pokemon_icon(&self, pokedex_id: &u8, pokemon_icon: &PokemonIcon) -> Result<Patch> {
        let _internal_id = self.pokedex_id_to_internal_id(pokedex_id)?;

        let offset_base = PkmnapiDB::ROM_PAGE * 0x1C;
        let offset = (offset_base + 0x190D) + ((((*pokedex_id - 1) as f32) / 2.0).floor() as usize);

        let data = if pokedex_id % 2 == 0 {
            vec![(self.rom[offset] & 0xF0) | pokemon_icon.value()]
        } else {
            vec![(self.rom[offset] & 0x0F) | (pokemon_icon.value() << 0x04)]
        };

        Ok(Patch::new(&offset, &data))
    }
}

#[derive(Debug, PartialEq)]
pub struct PokemonIcon {
    pub icon_id: u8,
}

impl From<&u8> for PokemonIcon {
    fn from(icon_id: &u8) -> Self {
        PokemonIcon { icon_id: *icon_id }
    }
}

impl PokemonIcon {
    pub fn value(&self) -> u8 {
        self.icon_id
    }
}
