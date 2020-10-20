use crate::error::{self, Result};
use crate::patch::*;
use crate::string::*;
use crate::PkmnapiDB;
use std::collections::HashMap;

impl PkmnapiDB {
    pub fn get_pokedex_text_all(&self, pokedex_ids: &Vec<u8>) -> Result<HashMap<u8, PokedexText>> {
        let pokedex_text_all: HashMap<u8, PokedexText> = pokedex_ids
            .iter()
            .map(|pokedex_id| {
                let pokedex_text = self.get_pokedex_text(pokedex_id)?;

                Ok((*pokedex_id, pokedex_text))
            })
            .collect::<Result<HashMap<u8, PokedexText>>>()?;

        Ok(pokedex_text_all)
    }

    /// Get Pokédex entry text by Pokédex ID
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
    /// let pokedex_text = db.get_pokedex_text(&1).unwrap();
    ///
    /// assert_eq!(pokedex_text, PokedexText {
    ///     text: ROMString::from("A strange seed was\nplanted on its\nback at birth.¶The plant sprouts\nand grows with\nthis #MON"),
    /// });
    /// ```
    pub fn get_pokedex_text(&self, pokedex_id: &u8) -> Result<PokedexText> {
        let internal_id = self.pokedex_id_to_internal_id(pokedex_id)?;

        let offset_base = PkmnapiDB::ROM_PAGE * 0x0F;
        let pointer_offset = (offset_base + 0x447E) + ((internal_id as usize) * 2);

        let pointer = offset_base + self.get_pointer(pointer_offset);

        let pointer_offset =
            pointer + { self.rom[pointer..].iter().position(|&r| r == 0x50).unwrap() } + 0x06;

        let pointer = self.get_pointer(pointer_offset);
        let pointer_base = PkmnapiDB::ROM_PAGE * (self.rom[pointer_offset + 2] as usize);
        let pointer = pointer + pointer_base - PkmnapiDB::ROM_PAGE;

        let pokedex_text = PokedexText::from(&self.rom[pointer..]);

        Ok(pokedex_text)
    }

    /// Set Pokédex entry text by Pokédex ID
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
    ///     .set_pokedex_text(&1, &PokedexText {
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
    pub fn set_pokedex_text(&self, pokedex_id: &u8, pokedex_text: &PokedexText) -> Result<Patch> {
        let old_pokedex_text = self.get_pokedex_text(pokedex_id)?;
        let old_pokedex_text_len = old_pokedex_text.text.value.len();
        let pokedex_text_raw = pokedex_text.text.to_string();
        let pokedex_text_len = pokedex_text_raw.len();

        if pokedex_text_len >= old_pokedex_text_len {
            return Err(error::Error::PokedexTextWrongSize(
                old_pokedex_text_len,
                pokedex_text_len,
            ));
        }

        let internal_id = self.pokedex_id_to_internal_id(pokedex_id)?;

        let offset_base = PkmnapiDB::ROM_PAGE * 0x0F;
        let pointer_offset = (offset_base + 0x447E) + ((internal_id as usize) * 2);

        let pointer = offset_base + self.get_pointer(pointer_offset);

        let pointer_offset =
            pointer + { self.rom[pointer..].iter().position(|&r| r == 0x50).unwrap() } + 0x06;

        let pointer = self.get_pointer(pointer_offset);
        let pointer_base = PkmnapiDB::ROM_PAGE * (self.rom[pointer_offset + 2] as usize);
        let pointer = pointer + pointer_base - PkmnapiDB::ROM_PAGE;

        Ok(Patch::new(&pointer, &pokedex_text.to_raw()))
    }
}

/// Pokédex entry text
///
/// # Example
///
/// ```
/// use pkmnapi_db::string::*;
/// use pkmnapi_db::*;
///
/// let rom = vec![0x00, 0x83, 0x91, 0x88, 0x8B, 0x8B, 0x5F];
/// let pokedex_text = PokedexText::from(&rom[..]);
///
/// assert_eq!(
///     pokedex_text,
///     PokedexText {
///         text: ROMString::from("DRILL"),
///     }
/// );
/// ```
#[derive(Debug, PartialEq)]
pub struct PokedexText {
    pub text: ROMString,
}

impl From<&[u8]> for PokedexText {
    /// Convert &[u8] to PokedexText
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::string::*;
    /// use pkmnapi_db::*;
    ///
    /// let rom = vec![0x00, 0x83, 0x91, 0x88, 0x8B, 0x8B, 0x5F];
    /// let pokedex_entry = PokedexText::from(&rom[..]);
    ///
    /// assert_eq!(
    ///     pokedex_entry,
    ///     PokedexText {
    ///         text: ROMString::from("DRILL"),
    ///     }
    /// );
    /// ```
    fn from(rom: &[u8]) -> Self {
        let text_end_index = rom.iter().position(|&r| r == 0x5F).unwrap_or(rom.len());

        let text = ROMString::new(&rom[1..text_end_index]);

        PokedexText { text }
    }
}

impl PokedexText {
    /// Pokédex entry text to raw bytes
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::string::*;
    /// use pkmnapi_db::*;
    ///
    /// let pokedex_text = PokedexText {
    ///     text: ROMString::from("DRILL"),
    /// };
    ///
    /// let raw = pokedex_text.to_raw();
    ///
    /// assert_eq!(raw, vec![0x00, 0x83, 0x91, 0x88, 0x8B, 0x8B, 0x5F]);
    /// ```
    pub fn to_raw(&self) -> Vec<u8> {
        [vec![0x00], self.text.value.to_vec(), vec![0x5F]].concat()
    }
}
