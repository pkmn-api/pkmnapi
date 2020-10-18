use crate::error::{self, Result};
use crate::patch::*;
use crate::string::*;
use crate::PkmnapiDB;
use std::collections::HashMap;

impl PkmnapiDB {
    pub fn get_move_name_all(&self, move_ids: &Vec<u8>) -> Result<HashMap<u8, MoveName>> {
        let move_name_all: HashMap<u8, MoveName> = move_ids
            .iter()
            .map(|move_id| {
                let move_name = self.get_move_name(move_id)?;

                Ok((*move_id, move_name))
            })
            .collect::<Result<HashMap<u8, MoveName>>>()?;

        Ok(move_name_all)
    }

    /// Get move name by move ID
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
    /// let move_name = db.get_move_name(&1).unwrap();
    ///
    /// assert_eq!(
    ///     move_name,
    ///     MoveName {
    ///         name: ROMString::from("POUND")
    ///     }
    /// );
    /// ```
    pub fn get_move_name(&self, move_id: &u8) -> Result<MoveName> {
        let (min_id, max_id) = self.move_id_validate(move_id)?;

        let offset_base = PkmnapiDB::ROM_PAGE * 0x58;
        let offset = self.rom[offset_base..]
            .iter()
            .enumerate()
            .filter_map(|(i, x)| {
                if i == 0 {
                    return Some(offset_base);
                }

                if *x == 0x50 {
                    return Some(offset_base + i + 1);
                }

                None
            })
            .take(max_id)
            .skip((*move_id - 1) as usize)
            .next();

        let offset = match offset {
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
    pub fn set_move_name(&self, move_id: &u8, move_name: &MoveName) -> Result<Patch> {
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

        let offset_base = PkmnapiDB::ROM_PAGE * 0x58;
        let offset = self.rom[offset_base..]
            .iter()
            .enumerate()
            .filter_map(|(i, x)| {
                if i == 0 {
                    return Some(offset_base);
                }

                if *x == 0x50 {
                    return Some(offset_base + i + 1);
                }

                None
            })
            .take(max_id)
            .skip((*move_id - 1) as usize)
            .next();

        let offset = match offset {
            Some(offset) => offset,
            None => return Err(error::Error::MoveIDInvalid(*move_id, min_id, max_id)),
        };

        Ok(Patch::new(&offset, &move_name_raw))
    }
}

/// Move name
///
/// # Example
///
/// ```
/// use pkmnapi_db::string::*;
/// use pkmnapi_db::*;
///
/// let rom = vec![0x80, 0x81, 0x82, 0x50];
/// let move_name = MoveName::from(&rom[..]);
///
/// assert_eq!(
///     move_name,
///     MoveName {
///         name: ROMString::from("ABC")
///     }
/// );
/// ```
#[derive(Debug, PartialEq)]
pub struct MoveName {
    pub name: ROMString,
}

impl From<&[u8]> for MoveName {
    /// Convert &[u8] to MoveName
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::string::*;
    /// use pkmnapi_db::*;
    ///
    /// let rom = vec![0x80, 0x81, 0x82, 0x50];
    /// let move_name = MoveName::from(&rom[..]);
    ///
    /// assert_eq!(
    ///     move_name,
    ///     MoveName {
    ///         name: ROMString::from("ABC")
    ///     }
    /// );
    /// ```
    fn from(rom: &[u8]) -> Self {
        let name_end_index = rom.iter().position(|&r| r == 0x50).unwrap_or(rom.len());

        let name = ROMString::new(&rom[..name_end_index]);

        MoveName { name }
    }
}

impl MoveName {
    /// Move name to raw bytes
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::string::*;
    /// use pkmnapi_db::*;
    ///
    /// let move_name = MoveName {
    ///     name: ROMString::from("ABC"),
    /// };
    ///
    /// let raw = move_name.to_raw();
    ///
    /// assert_eq!(raw, vec![0x80, 0x81, 0x82]);
    /// ```
    pub fn to_raw(&self) -> Vec<u8> {
        self.name.value[..].to_vec()
    }
}
