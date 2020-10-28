use crate::error::{self, Result};
use crate::patch::*;
use crate::string::*;
use crate::PkmnapiDB;
use byteorder::{LittleEndian, ReadBytesExt};
use std::collections::HashMap;
use std::io::Cursor;

impl PkmnapiDB {
    pub fn get_pokedex_entry_all(
        &self,
        pokedex_ids: &Vec<u8>,
    ) -> Result<HashMap<u8, PokedexEntry>> {
        self.get_all(pokedex_ids, |id| self.get_pokedex_entry(id))
    }

    /// Get Pokédex entry by Pokédex ID
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
    /// let pokedex_entry = db.get_pokedex_entry(&1).unwrap();
    ///
    /// assert_eq!(pokedex_entry, PokedexEntry {
    ///     species: ROMString::from("SEED"),
    ///     height: 28,
    ///     weight: 150
    /// });
    /// ```
    pub fn get_pokedex_entry(&self, pokedex_id: &u8) -> Result<PokedexEntry> {
        let internal_id = self.pokedex_id_to_internal_id(pokedex_id)?;

        let offset_base = PkmnapiDB::ROM_PAGE * 0x0F;
        let pointer_offset = (offset_base + 0x447E) + ((internal_id as usize) * 2);

        let pointer = offset_base + self.get_pointer(pointer_offset);

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
    /// use pkmnapi_db::*;
    /// use std::fs;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom).build().unwrap();
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
    ) -> Result<Patch> {
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

        let offset_base = PkmnapiDB::ROM_PAGE * 0x0F;
        let pointer_offset = (offset_base + 0x447E) + ((internal_id as usize) * 2);

        let pointer = offset_base + self.get_pointer(pointer_offset);

        Ok(Patch::new(&pointer, &pokedex_entry.to_raw()))
    }
}

/// Pokédex entry
///
/// # Example
///
/// ```
/// use pkmnapi_db::string::*;
/// use pkmnapi_db::*;
///
/// let rom = vec![
///     0x83, 0x91, 0x88, 0x8B, 0x8B, 0x50, 0x06, 0x03, 0x5A, 0x0A, 0x00, 0x00, 0x00, 0x00, 0x00,
/// ];
/// let pokedex_entry = PokedexEntry::from(&rom[..]);
///
/// assert_eq!(
///     pokedex_entry,
///     PokedexEntry {
///         species: ROMString::from("DRILL"),
///         height: 75,
///         weight: 2650,
///     }
/// );
/// ```
#[derive(Debug, PartialEq)]
pub struct PokedexEntry {
    pub species: ROMString,
    pub height: u32,
    pub weight: u32,
}

impl From<&[u8]> for PokedexEntry {
    /// Convert &[u8] to PokedexEntry
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::string::*;
    /// use pkmnapi_db::*;
    ///
    /// let rom = vec![
    ///     0x83, 0x91, 0x88, 0x8B, 0x8B, 0x50, 0x06, 0x03, 0x5A, 0x0A, 0x00, 0x00, 0x00, 0x00, 0x00,
    /// ];
    /// let pokedex_entry = PokedexEntry::from(&rom[..]);
    ///
    /// assert_eq!(
    ///     pokedex_entry,
    ///     PokedexEntry {
    ///         species: ROMString::from("DRILL"),
    ///         height: 75,
    ///         weight: 2650,
    ///     }
    /// );
    /// ```
    fn from(rom: &[u8]) -> Self {
        let species_end_index = rom.iter().position(|&r| r == 0x50).unwrap_or(rom.len());

        let species = ROMString::new(&rom[..species_end_index]);

        let mut cursor = Cursor::new(&rom[(species_end_index + 1)..]);

        let height_ft = cursor.read_u8().unwrap_or(0) as u32;
        let height_in = cursor.read_u8().unwrap_or(0) as u32;
        let height = (height_ft * 12) + height_in;
        let weight = cursor.read_u16::<LittleEndian>().unwrap_or(0) as u32;

        PokedexEntry {
            species,
            height,
            weight,
        }
    }
}

impl PokedexEntry {
    /// Pokédex entry to raw bytes
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::string::*;
    /// use pkmnapi_db::*;
    ///
    /// let pokedex_entry = PokedexEntry {
    ///     species: ROMString::from("DRILL"),
    ///     height: 75,
    ///     weight: 2650,
    /// };
    ///
    /// let raw = pokedex_entry.to_raw();
    ///
    /// assert_eq!(
    ///     raw,
    ///     vec![0x83, 0x91, 0x88, 0x8B, 0x8B, 0x50, 0x06, 0x03, 0x5A, 0x0A]
    /// );
    /// ```
    pub fn to_raw(&self) -> Vec<u8> {
        let species_string = ROMString::from(self.species.to_string());
        let height_ft = ((self.height as f32) / 12.0) as u32;
        let height_in = (self.height - (height_ft * 12)) as u32;

        [
            species_string.value,
            vec![0x50],
            vec![height_ft as u8, height_in as u8],
            (self.weight as u16).to_le_bytes().to_vec(),
        ]
        .concat()
    }
}
