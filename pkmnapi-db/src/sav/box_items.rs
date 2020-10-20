use crate::error::{self, Result};
use crate::patch::*;
use crate::sav::{Sav, SaveItem};

impl Sav {
    /// Get save box items
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
    /// let box_items = sav.get_box_items().unwrap();
    ///
    /// assert_eq!(
    ///     box_items,
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
    pub fn get_box_items(&self) -> Result<Vec<SaveItem>> {
        let offset = 0x27E6;
        let max_len = 50;

        let item_count = self.sav[offset] as usize;

        let offset = offset + 1;

        let save_box_items: Vec<SaveItem> = self.sav[offset..(offset + (max_len * 2))]
            .chunks(2)
            .take(item_count)
            .map(|chunk| SaveItem::from(chunk))
            .collect();

        Ok(save_box_items)
    }

    /// Set save box items
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
    ///     .set_box_items(
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
    ///         offset: 0x27E6,
    ///         length: 0x04,
    ///         data: vec![0x01, 0x01, 0x20, 0xFF]
    ///     }
    /// );
    /// ```
    pub fn set_box_items(&self, save_box_items: &Vec<SaveItem>) -> Result<Patch> {
        let offset = 0x27E6;
        let max_len = 50;

        let item_count = save_box_items.len();

        if item_count > max_len {
            return Err(error::Error::SavBoxItemsWrongSize(max_len, item_count));
        }

        let save_box_items_data: Vec<u8> = save_box_items
            .iter()
            .take(max_len)
            .map(|save_bag_item| save_bag_item.to_raw())
            .flatten()
            .collect();

        let data = [vec![item_count as u8], save_box_items_data, vec![0xFF]].concat();

        Ok(Patch::new(&offset, &data))
    }
}
