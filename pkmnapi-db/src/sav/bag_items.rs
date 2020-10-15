use crate::error::{self, Result};
use crate::patch::*;
use crate::sav::Sav;
use crate::PkmnapiDB;

impl Sav {
    /// Get save bag items
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::sav::*;
    /// use std::fs;
    /// # use std::env;
    /// # let sav_path = env::var("PKMN_SAV").expect("Set the PKMN_SAV environment variable to point to the SAV location");
    ///
    /// let sav_data = fs::read(sav_path).unwrap();
    /// let sav = Sav::new(&sav_data).unwrap();
    ///
    /// let bag_items = sav.get_bag_items().unwrap();
    ///
    /// assert_eq!(
    ///     bag_items,
    ///     vec![
    ///         SaveItem {
    ///             item_id: 0x04,
    ///             amount: 0x05
    ///         },
    ///         SaveItem {
    ///             item_id: 0x14,
    ///             amount: 0x02
    ///         }
    ///     ]
    /// );
    /// ```
    pub fn get_bag_items(&self) -> Result<Vec<SaveItem>> {
        let offset_base = PkmnapiDB::ROM_PAGE * 0x01;
        let offset = offset_base + 0x5C9;
        let max_len = 20;

        let item_count = self.sav[offset] as usize;

        let offset = offset + 1;

        let save_bag_items: Vec<SaveItem> = self.sav[offset..(offset + (max_len * 2))]
            .chunks(2)
            .take(item_count)
            .map(|chunk| SaveItem::from(chunk))
            .collect();

        Ok(save_bag_items)
    }

    /// Set save bag items
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::patch::*;
    /// use pkmnapi_db::sav::*;
    /// use std::fs;
    /// # use std::env;
    /// # let sav_path = env::var("PKMN_SAV").expect("Set the PKMN_SAV environment variable to point to the SAV location");
    ///
    /// let sav_data = fs::read(sav_path).unwrap();
    /// let sav = Sav::new(&sav_data).unwrap();
    ///
    /// let patch = sav
    ///     .set_bag_items(
    ///         &vec![
    ///             SaveItem {
    ///                 item_id: 0x01,
    ///                 amount: 0x20
    ///             }
    ///         ]
    ///     )
    ///     .unwrap();
    ///
    /// assert_eq!(
    ///     patch,
    ///     Patch {
    ///         offset: 0x25C9,
    ///         length: 0x04,
    ///         data: vec![0x01, 0x01, 0x20, 0xFF]
    ///     }
    /// );
    /// ```
    pub fn set_bag_items(&self, save_bag_items: &Vec<SaveItem>) -> Result<Patch> {
        let offset_base = PkmnapiDB::ROM_PAGE * 0x01;
        let offset = offset_base + 0x5C9;
        let max_len = 20;

        let item_count = save_bag_items.len();

        if item_count > max_len {
            return Err(error::Error::SavBagItemsWrongSize(max_len, item_count));
        }

        let save_bag_items_data: Vec<u8> = save_bag_items
            .iter()
            .take(max_len)
            .map(|save_bag_item| save_bag_item.to_raw())
            .flatten()
            .collect();

        let data = [vec![item_count as u8], save_bag_items_data, vec![0xFF]].concat();

        Ok(Patch::new(&offset, &data))
    }
}

/// Save item
///
/// # Example
///
/// ```
/// use pkmnapi_db::sav::*;
///
/// let sav = vec![0x01, 0x02];
/// let save_item = SaveItem::from(&sav[..]);
///
/// assert_eq!(
///     save_item,
///     SaveItem {
///         item_id: 0x01,
///         amount: 0x02
///     }
/// );
/// ```
#[derive(Debug, PartialEq)]
pub struct SaveItem {
    pub item_id: u8,
    pub amount: u8,
}

impl From<&[u8]> for SaveItem {
    /// Convert &[u8] to SaveItem
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::sav::*;
    ///
    /// let sav = vec![0x01, 0x02];
    /// let save_item = SaveItem::from(&sav[..]);
    ///
    /// assert_eq!(
    ///     save_item,
    ///     SaveItem {
    ///         item_id: 0x01,
    ///         amount: 0x02
    ///     }
    /// );
    /// ```
    fn from(sav: &[u8]) -> Self {
        let item_id = sav[0];
        let amount = sav[1];

        SaveItem { item_id, amount }
    }
}

impl SaveItem {
    /// Save item to raw bytes
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::sav::*;
    ///
    /// let save_item = SaveItem {
    ///     item_id: 0x01,
    ///     amount: 0x02,
    /// };
    ///
    /// let raw = save_item.to_raw();
    ///
    /// assert_eq!(raw, vec![0x01, 0x02]);
    /// ```
    pub fn to_raw(&self) -> Vec<u8> {
        vec![self.item_id, self.amount]
    }
}
