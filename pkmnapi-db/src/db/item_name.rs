use crate::error::{self, Result};
use crate::patch::*;
use crate::string::*;
use crate::PkmnapiDB;
use std::collections::HashMap;

impl PkmnapiDB {
    pub fn get_item_name_all(&self, item_ids: &Vec<u8>) -> Result<HashMap<u8, ItemName>> {
        self.get_all(item_ids, |id| self.get_item_name(id))
    }

    /// Get item name by item ID
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
    /// let item_name = db.get_item_name(&1).unwrap();
    ///
    /// assert_eq!(
    ///     item_name,
    ///     ItemName {
    ///         name: ROMString::from("MASTER BALL")
    ///     }
    /// );
    /// ```
    pub fn get_item_name(&self, item_id: &u8) -> Result<ItemName> {
        let (min_id, max_id) = self.item_id_validate(item_id)?;

        let offset_base = PkmnapiDB::ROM_PAGE;
        let offset_base = offset_base + 0x072B;
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
            .skip((*item_id - 1) as usize)
            .next();

        let offset = match offset {
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
    pub fn set_item_name(&self, item_id: &u8, item_name: &ItemName) -> Result<Patch> {
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

        let offset_base = PkmnapiDB::ROM_PAGE;
        let offset_base = offset_base + 0x072B;
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
            .skip((*item_id - 1) as usize)
            .next();

        let offset = match offset {
            Some(offset) => offset,
            None => return Err(error::Error::ItemIDInvalid(*item_id, min_id, max_id)),
        };

        Ok(Patch::new(&offset, &item_name_raw))
    }
}

/// Item name
///
/// # Example
///
/// ```
/// use pkmnapi_db::string::*;
/// use pkmnapi_db::*;
///
/// let rom = vec![0x80, 0x81, 0x82, 0x50];
/// let item_name = ItemName::from(&rom[..]);
///
/// assert_eq!(
///     item_name,
///     ItemName {
///         name: ROMString::from("ABC")
///     }
/// );
/// ```
#[derive(Debug, PartialEq)]
pub struct ItemName {
    pub name: ROMString,
}

impl From<&[u8]> for ItemName {
    /// Convert &[u8] to ItemName
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::string::*;
    /// use pkmnapi_db::*;
    ///
    /// let rom = vec![0x80, 0x81, 0x82, 0x50];
    /// let item_name = ItemName::from(&rom[..]);
    ///
    /// assert_eq!(
    ///     item_name,
    ///     ItemName {
    ///         name: ROMString::from("ABC")
    ///     }
    /// );
    /// ```
    fn from(rom: &[u8]) -> Self {
        let name_end_index = rom.iter().position(|&r| r == 0x50).unwrap_or(rom.len());

        let name = ROMString::new(&rom[..name_end_index]);

        ItemName { name }
    }
}

impl ItemName {
    /// Item name to raw bytes
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::string::*;
    /// use pkmnapi_db::*;
    ///
    /// let item_name = ItemName {
    ///     name: ROMString::from("ABC"),
    /// };
    ///
    /// let raw = item_name.to_raw();
    ///
    /// assert_eq!(raw, vec![0x80, 0x81, 0x82]);
    /// ```
    pub fn to_raw(&self) -> Vec<u8> {
        self.name.value[..].to_vec()
    }
}
