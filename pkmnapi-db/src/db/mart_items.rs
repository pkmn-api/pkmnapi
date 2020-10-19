use crate::error::{self, Result};
use crate::patch::*;
use crate::PkmnapiDB;
use std::collections::HashMap;

impl PkmnapiDB {
    pub fn get_mart_items_all(&self, mart_ids: &Vec<u8>) -> Result<HashMap<u8, Vec<MartItem>>> {
        let mart_items_all: HashMap<u8, Vec<MartItem>> = mart_ids
            .iter()
            .map(|mart_id| {
                let mart_items = self.get_mart_items(mart_id)?;

                Ok((*mart_id, mart_items))
            })
            .collect::<Result<HashMap<u8, Vec<MartItem>>>>()?;

        Ok(mart_items_all)
    }

    /// Get mart items by mart ID
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
    /// let mart_items = db.get_mart_items(&0).unwrap();
    ///
    /// assert_eq!(
    ///     mart_items,
    ///     vec![
    ///         MartItem::ITEM(0x04),
    ///         MartItem::ITEM(0x0B),
    ///         MartItem::ITEM(0x0F),
    ///         MartItem::ITEM(0x0C)
    ///     ]
    /// );
    /// ```
    pub fn get_mart_items(&self, mart_id: &u8) -> Result<Vec<MartItem>> {
        let (min_id, max_id) = self.mart_id_validate(mart_id)?;

        let offset_base = 0x2442;
        let offset = self.rom[offset_base..]
            .iter()
            .enumerate()
            .filter_map(|(i, x)| {
                if *x == 0xFE {
                    return Some(offset_base + i);
                }

                None
            })
            .take(max_id + 1)
            .skip(*mart_id as usize)
            .next();

        let offset = match offset {
            Some(offset) => offset,
            None => return Err(error::Error::MartIDInvalid(*mart_id, min_id, max_id)),
        };

        let mart_items = self.rom[(offset + 2)..]
            .iter()
            .take_while(|&x| *x != 0xFF)
            .map(|x| MartItem::from(x))
            .collect();

        Ok(mart_items)
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
    ///     .set_mart_items(
    ///         &0,
    ///         &vec![
    ///             MartItem::ITEM(0x01),
    ///             MartItem::ITEM(0x02),
    ///             MartItem::TM(0x01),
    ///             MartItem::TM(0x02)
    ///         ],
    ///     )
    ///     .unwrap();
    ///
    /// assert_eq!(
    ///     patch,
    ///     Patch {
    ///         offset: 0x2444,
    ///         length: 0x04,
    ///         data: vec![0x01, 0x02, 0xC9, 0xCA]
    ///     }
    /// );
    /// ```
    pub fn set_mart_items(&self, mart_id: &u8, mart_items: &Vec<MartItem>) -> Result<Patch> {
        let (min_id, max_id) = self.mart_id_validate(mart_id)?;

        let old_mart_items = self.get_mart_items(mart_id)?;
        let old_mart_items_len = old_mart_items.len();
        let mart_items_len = mart_items.len();

        if old_mart_items_len != mart_items_len {
            return Err(error::Error::MartItemsWrongSize(
                old_mart_items_len,
                mart_items_len,
            ));
        }

        let offset_base = 0x2442;
        let offset = self.rom[offset_base..]
            .iter()
            .enumerate()
            .filter_map(|(i, x)| {
                if *x == 0xFE {
                    return Some(offset_base + i);
                }

                None
            })
            .take(max_id + 1)
            .skip(*mart_id as usize)
            .next();

        let offset = match offset {
            Some(offset) => offset,
            None => return Err(error::Error::MartIDInvalid(*mart_id, min_id, max_id)),
        } + 2;

        let data = mart_items
            .iter()
            .map(|mart_item| (*mart_item).to_raw())
            .flatten()
            .collect();

        Ok(Patch::new(&offset, &data))
    }
}

/// Mart item
///
/// # Example
///
/// ```
/// use pkmnapi_db::*;
///
/// let mart_item = MartItem::from(&0x01);
///
/// assert_eq!(mart_item, MartItem::ITEM(0x01));
///
/// let mart_item = MartItem::from(&0xC9);
///
/// assert_eq!(mart_item, MartItem::TM(0x01));
/// ```
#[derive(Debug, PartialEq)]
pub enum MartItem {
    ITEM(u8),
    TM(u8),
}

impl From<&u8> for MartItem {
    /// Convert &u8 to MartItem
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::*;
    ///
    /// let mart_item = MartItem::from(&0x01);
    ///
    /// assert_eq!(mart_item, MartItem::ITEM(0x01));
    ///
    /// let mart_item = MartItem::from(&0xC9);
    ///
    /// assert_eq!(mart_item, MartItem::TM(0x01));
    /// ```
    fn from(item_id: &u8) -> Self {
        if *item_id >= 0xC9 {
            MartItem::TM(*item_id - 0xC8)
        } else {
            MartItem::ITEM(*item_id)
        }
    }
}

impl MartItem {
    /// Mart item to raw bytes
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::*;
    ///
    /// let mart_item = MartItem::ITEM(0x01);
    /// let raw = mart_item.to_raw();
    ///
    /// assert_eq!(raw, vec![0x01]);
    ///
    /// let mart_item = MartItem::TM(0x01);
    /// let raw = mart_item.to_raw();
    ///
    /// assert_eq!(raw, vec![0xC9]);
    /// ```
    pub fn to_raw(&self) -> Vec<u8> {
        vec![match self {
            MartItem::ITEM(item_id) => *item_id,
            MartItem::TM(tm_id) => *tm_id + 0xC8,
        }]
    }
}
