use crate::error::Result;
use crate::patch::*;
use crate::string::*;
use crate::PkmnapiDB;
use std::collections::HashMap;

impl PkmnapiDB {
    pub fn get_pokemon_name_all(&self, pokedex_ids: &Vec<u8>) -> Result<HashMap<u8, PokemonName>> {
        let pokemon_name_all: HashMap<u8, PokemonName> = pokedex_ids
            .iter()
            .map(|pokedex_id| {
                let pokemon_name = self.get_pokemon_name(pokedex_id)?;

                Ok((*pokedex_id, pokemon_name))
            })
            .collect::<Result<HashMap<u8, PokemonName>>>()?;

        Ok(pokemon_name_all)
    }

    /// Get Pokémon name by Pokédex ID
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
    pub fn get_pokemon_name(&self, pokedex_id: &u8) -> Result<PokemonName> {
        let internal_id = self.pokedex_id_to_internal_id(pokedex_id)?;

        let offset_base = PkmnapiDB::ROM_PAGE * 0x0E;
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
    pub fn set_pokemon_name(&self, pokedex_id: &u8, pokemon_name: &PokemonName) -> Result<Patch> {
        let internal_id = self.pokedex_id_to_internal_id(pokedex_id)?;

        let offset_base = PkmnapiDB::ROM_PAGE * 0x0E;
        let offset = (offset_base + 0x021E) + ((internal_id as usize) * 0x0A);

        let pokemon_name_len = pokemon_name.name.value.len();
        let pokemon_name_raw = pokemon_name.to_raw();

        let data = [pokemon_name_raw, vec![0x50; 0x0A - pokemon_name_len]].concat();

        Ok(Patch::new(&offset, &data))
    }
}

/// Pokémon name
///
/// # Example
///
/// ```
/// use pkmnapi_db::string::*;
/// use pkmnapi_db::*;
///
/// let rom = vec![0x80, 0x81, 0x82, 0x50];
/// let pokemon_name = PokemonName::from(&rom[..]);
///
/// assert_eq!(
///     pokemon_name,
///     PokemonName {
///         name: ROMString::from("ABC")
///     }
/// );
/// ```
#[derive(Debug, PartialEq)]
pub struct PokemonName {
    pub name: ROMString,
}

impl From<&[u8]> for PokemonName {
    /// Convert &[u8] to PokemonName
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::string::*;
    /// use pkmnapi_db::*;
    ///
    /// let rom = vec![0x80, 0x81, 0x82, 0x50];
    /// let pokemon_name = PokemonName::from(&rom[..]);
    ///
    /// assert_eq!(
    ///     pokemon_name,
    ///     PokemonName {
    ///         name: ROMString::from("ABC")
    ///     }
    /// );
    /// ```
    fn from(rom: &[u8]) -> Self {
        let name_end_index = rom.iter().position(|&r| r == 0x50).unwrap_or(rom.len());

        let name = ROMString::new(&rom[..name_end_index]);

        PokemonName { name }
    }
}

impl PokemonName {
    /// Pokémon name to raw bytes
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::string::*;
    /// use pkmnapi_db::*;
    ///
    /// let pokemon_name = PokemonName {
    ///     name: ROMString::from("ABC"),
    /// };
    ///
    /// let raw = pokemon_name.to_raw();
    ///
    /// assert_eq!(raw, vec![0x80, 0x81, 0x82]);
    /// ```
    pub fn to_raw(&self) -> Vec<u8> {
        self.name.value[..].to_vec()
    }
}
