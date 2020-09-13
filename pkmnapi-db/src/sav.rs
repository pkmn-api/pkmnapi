//! Pkmnapi sav module
//!
//! # Example
//!
//! ```
//! use pkmnapi_db::sav::*;
//! use std::fs;
//! # use std::env;
//! # let sav_path = env::var("PKMN_SAV").expect("Set the PKMN_SAV environment variable to point to the SAV location");
//!
//! let sav_data = fs::read(sav_path).unwrap();
//! let sav = Sav::new(&sav_data).unwrap();
//!
//! assert_eq!(sav.sav.len(), 0x8000);
//! ```

// SRC: https://github.com/junebug12851/pokered-save-editor/blob/37f5992e866b416ed731276f387a15db80c70815/non-app-assets/savefile-structure.bt

use crate::patch::*;
use crate::types::*;
use crate::ROM_PAGE;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::Cursor;
use std::num::Wrapping;

#[derive(Debug, PartialEq)]
pub struct Sav {
    pub sav: Vec<u8>,
}

impl Sav {
    //! Create sav from an array of bytes
    //!
    //! # Example
    //!
    //! ```
    //! use pkmnapi_db::sav::*;
    //! use std::fs;
    //! # use std::env;
    //! # let sav_path = env::var("PKMN_SAV").expect("Set the PKMN_SAV environment variable to point to the SAV location");
    //!
    //! let sav_data = fs::read(sav_path).unwrap();
    //! let sav = Sav::new(&sav_data).unwrap();
    //!
    //! assert_eq!(sav.sav.len(), 0x8000);
    pub fn new(sav: &Vec<u8>) -> Result<Self, String> {
        let sav_len = sav.len();
        let expected_sav_len = 0x8000;

        if sav_len != expected_sav_len {
            return Err(format!(
                "Length mismatch: should be {} bytes, found {}",
                expected_sav_len, sav_len
            ));
        }

        Ok(Sav { sav: sav.to_vec() })
    }

    /// Verify save checksum
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
    /// assert_eq!(sav.verify_checksum(), true);
    /// ```
    pub fn verify_checksum(&self) -> bool {
        let offset_base = ROM_PAGE * 0x01;
        let offset_start = offset_base + 0x0598;
        let offset_end = offset_base + 0x1522;
        let offset_checksum = offset_base + 0x1523;

        let checksum = self.sav[offset_start..=offset_end]
            .iter()
            .fold(Wrapping(0u8), |acc, x| acc + Wrapping(*x));
        let checksum_value = checksum.0 ^ 0xFF;

        checksum_value == self.sav[offset_checksum]
    }

    /// Generate save checksum
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
    /// let patch = sav.generate_checksum().unwrap();
    ///
    /// assert_eq!(
    ///     patch,
    ///     Patch {
    ///         offset: 0x3523,
    ///         length: 0x01,
    ///         data: vec![0xC5]
    ///     }
    /// );
    /// ```
    pub fn generate_checksum(&self) -> Result<Patch, String> {
        let offset_base = ROM_PAGE * 0x01;
        let offset_start = offset_base + 0x0598;
        let offset_end = offset_base + 0x1522;
        let offset_checksum = offset_base + 0x1523;

        let checksum = self.sav[offset_start..=offset_end]
            .iter()
            .fold(Wrapping(0u8), |acc, x| acc + Wrapping(*x));
        let checksum_value = checksum.0 ^ 0xFF;

        Ok(Patch::new(&offset_checksum, &vec![checksum_value]))
    }

    /// Apply save patch
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
    /// let mut sav = Sav::new(&sav_data).unwrap();
    ///
    /// assert_eq!(sav.sav[..4], [0x00, 0x00, 0x00, 0x00]);
    ///
    /// let patch = Patch::new(&0x00, &vec![0x13, 0x37]);
    ///
    /// sav.apply_patch(patch);
    ///
    /// assert_eq!(sav.sav[..4], [0x13, 0x37, 0x00, 0x00]);
    /// ```
    pub fn apply_patch<S: Into<Patch>>(&mut self, patch: S) {
        let patch = patch.into();

        self.sav = [
            &self.sav[..patch.offset],
            &patch.data[..],
            &self.sav[(patch.offset + patch.length)..],
        ]
        .concat();
    }

    /// Get save player name
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::string::*;
    /// use pkmnapi_db::types::*;
    /// use pkmnapi_db::sav::*;
    /// use std::fs;
    /// # use std::env;
    /// # let sav_path = env::var("PKMN_SAV").expect("Set the PKMN_SAV environment variable to point to the SAV location");
    ///
    /// let sav_data = fs::read(sav_path).unwrap();
    /// let sav = Sav::new(&sav_data).unwrap();
    ///
    /// let player_name = sav.get_player_name().unwrap();
    ///
    /// assert_eq!(
    ///     player_name,
    ///     SavePlayerName {
    ///         name: ROMString::from("RED")
    ///     }
    /// );
    /// ```
    pub fn get_player_name(&self) -> Result<SavePlayerName, String> {
        let offset_base = ROM_PAGE * 0x01;
        let offset = offset_base + 0x598;

        let save_player_name = SavePlayerName::from(&self.sav[offset..(offset + 0x0B)]);

        Ok(save_player_name)
    }

    /// Set save player name
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::patch::*;
    /// use pkmnapi_db::string::*;
    /// use pkmnapi_db::types::*;
    /// use pkmnapi_db::sav::*;
    /// use std::fs;
    /// # use std::env;
    /// # let sav_path = env::var("PKMN_SAV").expect("Set the PKMN_SAV environment variable to point to the SAV location");
    ///
    /// let sav_data = fs::read(sav_path).unwrap();
    /// let sav = Sav::new(&sav_data).unwrap();
    ///
    /// let patch = sav
    ///     .set_player_name(
    ///         &SavePlayerName {
    ///             name: ROMString::from("ABCDE"),
    ///         },
    ///     )
    ///     .unwrap();
    ///
    /// assert_eq!(
    ///     patch,
    ///     Patch {
    ///         offset: 0x2598,
    ///         length: 0x06,
    ///         data: vec![0x80, 0x81, 0x82, 0x83, 0x084, 0x50]
    ///     }
    /// );
    /// ```
    pub fn set_player_name(&self, save_player_name: &SavePlayerName) -> Result<Patch, String> {
        let offset_base = ROM_PAGE * 0x01;
        let offset = offset_base + 0x598;

        let save_player_name_raw = save_player_name.to_raw();
        let save_player_name_raw_len = save_player_name_raw.len();
        let max_len = 0x0A;

        if save_player_name_raw_len > max_len {
            return Err(format!(
                "Length mismatch: should be {} characters or fewer, found {}",
                max_len, save_player_name_raw_len
            ));
        }

        let padding = vec![
            0x50;
            {
                if save_player_name_raw_len != max_len {
                    0x01
                } else {
                    0x00
                }
            }
        ];

        let save_player_name_raw = [save_player_name_raw, padding].concat();

        Ok(Patch::new(&offset, &save_player_name_raw))
    }

    /// Get save Pokémon owned
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::types::*;
    /// use pkmnapi_db::sav::*;
    /// use std::fs;
    /// # use std::env;
    /// # let sav_path = env::var("PKMN_SAV").expect("Set the PKMN_SAV environment variable to point to the SAV location");
    ///
    /// let sav_data = fs::read(sav_path).unwrap();
    /// let sav = Sav::new(&sav_data).unwrap();
    ///
    /// let pokemon_owned = sav.get_pokemon_owned().unwrap();
    ///
    /// assert_eq!(
    ///     pokemon_owned,
    ///     vec![0x01]
    /// );
    /// ```
    pub fn get_pokemon_owned(&self) -> Result<Vec<u8>, String> {
        let offset_base = ROM_PAGE * 0x01;
        let offset = offset_base + 0x5A3;

        let save_pokemon_owned: Vec<u8> = self.sav[offset..(offset + 19)]
            .iter()
            .map(|byte| (0..8).map(move |i| (byte & (0x01 << i)) >> i))
            .flatten()
            .enumerate()
            .filter_map(
                |(i, bit)| {
                    if bit == 0x01 {
                        Some(i as u8 + 1)
                    } else {
                        None
                    }
                },
            )
            .collect();

        Ok(save_pokemon_owned)
    }

    /// Set save Pokémon owned
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::patch::*;
    /// use pkmnapi_db::types::*;
    /// use pkmnapi_db::sav::*;
    /// use std::fs;
    /// # use std::env;
    /// # let sav_path = env::var("PKMN_SAV").expect("Set the PKMN_SAV environment variable to point to the SAV location");
    ///
    /// let sav_data = fs::read(sav_path).unwrap();
    /// let sav = Sav::new(&sav_data).unwrap();
    ///
    /// let patch = sav
    ///     .set_pokemon_owned(
    ///         &vec![0x01, 0x04, 0x07]
    ///     )
    ///     .unwrap();
    ///
    /// assert_eq!(
    ///     patch,
    ///     Patch {
    ///         offset: 0x25A3,
    ///         length: 0x13,
    ///         data: vec![0x49, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
    ///     }
    /// );
    /// ```
    pub fn set_pokemon_owned(&self, save_pokemon_owned: &Vec<u8>) -> Result<Patch, String> {
        let offset_base = ROM_PAGE * 0x01;
        let offset = offset_base + 0x5A3;

        let bits: Vec<u8> = (1..=152)
            .map(|id| {
                if save_pokemon_owned.contains(&id) {
                    0x01
                } else {
                    0x00
                }
            })
            .collect::<Vec<u8>>()
            .chunks(8)
            .map(|chunk| {
                (0..8)
                    .map(move |i| chunk[i] << i)
                    .fold(0, |acc, bit| acc | bit)
            })
            .collect();

        Ok(Patch::new(&offset, &bits))
    }

    /// Get save Pokémon seen
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::types::*;
    /// use pkmnapi_db::sav::*;
    /// use std::fs;
    /// # use std::env;
    /// # let sav_path = env::var("PKMN_SAV").expect("Set the PKMN_SAV environment variable to point to the SAV location");
    ///
    /// let sav_data = fs::read(sav_path).unwrap();
    /// let sav = Sav::new(&sav_data).unwrap();
    ///
    /// let pokemon_seen = sav.get_pokemon_seen().unwrap();
    ///
    /// assert_eq!(
    ///     pokemon_seen,
    ///     vec![0x01]
    /// );
    /// ```
    pub fn get_pokemon_seen(&self) -> Result<Vec<u8>, String> {
        let offset_base = ROM_PAGE * 0x01;
        let offset = offset_base + 0x5B6;

        let save_pokemon_seen: Vec<u8> = self.sav[offset..(offset + 19)]
            .iter()
            .map(|byte| (0..8).map(move |i| (byte & (0x01 << i)) >> i))
            .flatten()
            .enumerate()
            .filter_map(
                |(i, bit)| {
                    if bit == 0x01 {
                        Some(i as u8 + 1)
                    } else {
                        None
                    }
                },
            )
            .collect();

        Ok(save_pokemon_seen)
    }

    /// Set save Pokémon seen
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::patch::*;
    /// use pkmnapi_db::types::*;
    /// use pkmnapi_db::sav::*;
    /// use std::fs;
    /// # use std::env;
    /// # let sav_path = env::var("PKMN_SAV").expect("Set the PKMN_SAV environment variable to point to the SAV location");
    ///
    /// let sav_data = fs::read(sav_path).unwrap();
    /// let sav = Sav::new(&sav_data).unwrap();
    ///
    /// let patch = sav
    ///     .set_pokemon_seen(
    ///         &vec![0x01, 0x04, 0x07]
    ///     )
    ///     .unwrap();
    ///
    /// assert_eq!(
    ///     patch,
    ///     Patch {
    ///         offset: 0x25B6,
    ///         length: 0x13,
    ///         data: vec![0x49, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
    ///     }
    /// );
    /// ```
    pub fn set_pokemon_seen(&self, save_pokemon_seen: &Vec<u8>) -> Result<Patch, String> {
        let offset_base = ROM_PAGE * 0x01;
        let offset = offset_base + 0x5B6;

        let bits: Vec<u8> = (1..=152)
            .map(|id| {
                if save_pokemon_seen.contains(&id) {
                    0x01
                } else {
                    0x00
                }
            })
            .collect::<Vec<u8>>()
            .chunks(8)
            .map(|chunk| {
                (0..8)
                    .map(move |i| chunk[i] << i)
                    .fold(0, |acc, bit| acc | bit)
            })
            .collect();

        Ok(Patch::new(&offset, &bits))
    }

    /// Get save bag items
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::types::*;
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
    pub fn get_bag_items(&self) -> Result<Vec<SaveItem>, String> {
        let offset_base = ROM_PAGE * 0x01;
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
    /// use pkmnapi_db::types::*;
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
    pub fn set_bag_items(&self, save_bag_items: &Vec<SaveItem>) -> Result<Patch, String> {
        let offset_base = ROM_PAGE * 0x01;
        let offset = offset_base + 0x5C9;
        let max_len = 20;

        let item_count = save_bag_items.len();

        if item_count > max_len {
            return Err(format!(
                "Length mismatch: should be {} items or fewer, found {}",
                max_len, item_count
            ));
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

    /// Get save money
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::types::*;
    /// use pkmnapi_db::sav::*;
    /// use std::fs;
    /// # use std::env;
    /// # let sav_path = env::var("PKMN_SAV").expect("Set the PKMN_SAV environment variable to point to the SAV location");
    ///
    /// let sav_data = fs::read(sav_path).unwrap();
    /// let sav = Sav::new(&sav_data).unwrap();
    ///
    /// let money = sav.get_money().unwrap();
    ///
    /// assert_eq!(
    ///     money,
    ///     123456
    /// );
    /// ```
    pub fn get_money(&self) -> Result<u32, String> {
        let offset_base = ROM_PAGE * 0x01;
        let offset = offset_base + 0x5F3;

        let money = self.sav[offset..(offset + 3)]
            .iter()
            .map(|byte| vec![(byte & 0xF0) >> 4, byte & 0x0F])
            .flatten()
            .enumerate()
            .map(|(i, digit)| (digit as u32) * 10_u32.pow(5 - i as u32))
            .fold(0, |acc, val| acc + val);

        Ok(money)
    }

    /// Set save money
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::patch::*;
    /// use pkmnapi_db::types::*;
    /// use pkmnapi_db::sav::*;
    /// use std::fs;
    /// # use std::env;
    /// # let sav_path = env::var("PKMN_SAV").expect("Set the PKMN_SAV environment variable to point to the SAV location");
    ///
    /// let sav_data = fs::read(sav_path).unwrap();
    /// let sav = Sav::new(&sav_data).unwrap();
    ///
    /// let patch = sav
    ///     .set_money(
    ///         &113377
    ///     )
    ///     .unwrap();
    ///
    /// assert_eq!(
    ///     patch,
    ///     Patch {
    ///         offset: 0x25F3,
    ///         length: 0x03,
    ///         data: vec![0x11, 0x33, 0x77]
    ///     }
    /// );
    /// ```
    pub fn set_money(&self, money: &u32) -> Result<Patch, String> {
        let offset_base = ROM_PAGE * 0x01;
        let offset = offset_base + 0x5F3;

        let data: Vec<u8> = (0..3)
            .map(|exp| {
                vec![
                    (((*money as f32) / (10u32.pow(5 - (exp * 2)) as f32)) % 10.0).floor() as u8,
                    (((*money as f32) / (10u32.pow(5 - (exp * 2) - 1) as f32)) % 10.0).floor()
                        as u8,
                ]
            })
            .map(|chunk| (chunk[0] << 4) | chunk[1])
            .collect();

        Ok(Patch::new(&offset, &data))
    }

    /// Get save rival name
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::string::*;
    /// use pkmnapi_db::types::*;
    /// use pkmnapi_db::sav::*;
    /// use std::fs;
    /// # use std::env;
    /// # let sav_path = env::var("PKMN_SAV").expect("Set the PKMN_SAV environment variable to point to the SAV location");
    ///
    /// let sav_data = fs::read(sav_path).unwrap();
    /// let sav = Sav::new(&sav_data).unwrap();
    ///
    /// let rival_name = sav.get_rival_name().unwrap();
    ///
    /// assert_eq!(
    ///     rival_name,
    ///     SaveRivalName {
    ///         name: ROMString::from("BLUE")
    ///     }
    /// );
    /// ```
    pub fn get_rival_name(&self) -> Result<SaveRivalName, String> {
        let offset_base = ROM_PAGE * 0x01;
        let offset = offset_base + 0x5F6;

        let save_rival_name = SaveRivalName::from(&self.sav[offset..(offset + 0x0B)]);

        Ok(save_rival_name)
    }

    /// Set save rival name
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::patch::*;
    /// use pkmnapi_db::string::*;
    /// use pkmnapi_db::types::*;
    /// use pkmnapi_db::sav::*;
    /// use std::fs;
    /// # use std::env;
    /// # let sav_path = env::var("PKMN_SAV").expect("Set the PKMN_SAV environment variable to point to the SAV location");
    ///
    /// let sav_data = fs::read(sav_path).unwrap();
    /// let sav = Sav::new(&sav_data).unwrap();
    ///
    /// let patch = sav
    ///     .set_rival_name(
    ///         &SaveRivalName {
    ///             name: ROMString::from("ABCDE"),
    ///         },
    ///     )
    ///     .unwrap();
    ///
    /// assert_eq!(
    ///     patch,
    ///     Patch {
    ///         offset: 0x25F6,
    ///         length: 0x06,
    ///         data: vec![0x80, 0x81, 0x82, 0x83, 0x084, 0x50]
    ///     }
    /// );
    /// ```
    pub fn set_rival_name(&self, save_rival_name: &SaveRivalName) -> Result<Patch, String> {
        let offset_base = ROM_PAGE * 0x01;
        let offset = offset_base + 0x5F6;

        let save_rival_name_raw = save_rival_name.to_raw();
        let save_rival_name_raw_len = save_rival_name_raw.len();
        let max_len = 0x0A;

        if save_rival_name_raw_len > max_len {
            return Err(format!(
                "Length mismatch: should be {} characters or fewer, found {}",
                max_len, save_rival_name_raw_len
            ));
        }

        let padding = vec![
            0x50;
            {
                if save_rival_name_raw_len != max_len {
                    0x01
                } else {
                    0x00
                }
            }
        ];

        let save_rival_name_raw = [save_rival_name_raw, padding].concat();

        Ok(Patch::new(&offset, &save_rival_name_raw))
    }

    /// Get save options
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::types::*;
    /// use pkmnapi_db::sav::*;
    /// use std::fs;
    /// # use std::env;
    /// # let sav_path = env::var("PKMN_SAV").expect("Set the PKMN_SAV environment variable to point to the SAV location");
    ///
    /// let sav_data = fs::read(sav_path).unwrap();
    /// let sav = Sav::new(&sav_data).unwrap();
    ///
    /// let options = sav.get_options().unwrap();
    ///
    /// assert_eq!(
    ///     options,
    ///     SaveOptions {
    ///         text_speed: SaveTextSpeed::FAST,
    ///         battle_animation: SaveBattleAnimation::ON,
    ///         battle_style: SaveBattleStyle::SHIFT,
    ///     }
    /// );
    /// ```
    pub fn get_options(&self) -> Result<SaveOptions, String> {
        let offset_base = ROM_PAGE * 0x01;
        let offset = offset_base + 0x601;

        let save_options = SaveOptions::from(&self.sav[offset]);

        Ok(save_options)
    }

    /// Set save options
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::patch::*;
    /// use pkmnapi_db::types::*;
    /// use pkmnapi_db::sav::*;
    /// use std::fs;
    /// # use std::env;
    /// # let sav_path = env::var("PKMN_SAV").expect("Set the PKMN_SAV environment variable to point to the SAV location");
    ///
    /// let sav_data = fs::read(sav_path).unwrap();
    /// let sav = Sav::new(&sav_data).unwrap();
    ///
    /// let patch = sav
    ///     .set_options(
    ///         &SaveOptions {
    ///             text_speed: SaveTextSpeed::SLOW,
    ///             battle_animation: SaveBattleAnimation::OFF,
    ///             battle_style: SaveBattleStyle::SET
    ///         }
    ///     )
    ///     .unwrap();
    ///
    /// assert_eq!(
    ///     patch,
    ///     Patch {
    ///         offset: 0x2601,
    ///         length: 0x01,
    ///         data: vec![0xC5]
    ///     }
    /// );
    /// ```
    pub fn set_options(&self, save_options: &SaveOptions) -> Result<Patch, String> {
        let offset_base = ROM_PAGE * 0x01;
        let offset = offset_base + 0x601;

        Ok(Patch::new(&offset, &save_options.to_raw()))
    }

    /// Get save badges
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::types::*;
    /// use pkmnapi_db::sav::*;
    /// use std::fs;
    /// # use std::env;
    /// # let sav_path = env::var("PKMN_SAV").expect("Set the PKMN_SAV environment variable to point to the SAV location");
    ///
    /// let sav_data = fs::read(sav_path).unwrap();
    /// let sav = Sav::new(&sav_data).unwrap();
    ///
    /// let badges = sav.get_badges().unwrap();
    ///
    /// assert_eq!(badges, vec![0x00]);
    /// ```
    pub fn get_badges(&self) -> Result<Vec<u8>, String> {
        let offset_base = ROM_PAGE * 0x01;
        let offset = offset_base + 0x602;

        let save_badges = (0..8)
            .filter_map(|i| {
                let exists = ((self.sav[offset] & (0x01 << i)) >> i) == 0x01;

                if exists {
                    Some(i as u8)
                } else {
                    None
                }
            })
            .collect();

        Ok(save_badges)
    }

    /// Set save badges
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::patch::*;
    /// use pkmnapi_db::types::*;
    /// use pkmnapi_db::sav::*;
    /// use std::fs;
    /// # use std::env;
    /// # let sav_path = env::var("PKMN_SAV").expect("Set the PKMN_SAV environment variable to point to the SAV location");
    ///
    /// let sav_data = fs::read(sav_path).unwrap();
    /// let sav = Sav::new(&sav_data).unwrap();
    ///
    /// let patch = sav
    ///     .set_badges(
    ///         &vec![0x00, 0x01, 0x02]
    ///     )
    ///     .unwrap();
    ///
    /// assert_eq!(
    ///     patch,
    ///     Patch {
    ///         offset: 0x2602,
    ///         length: 0x01,
    ///         data: vec![0x07]
    ///     }
    /// );
    /// ```
    pub fn set_badges(&self, save_badges: &Vec<u8>) -> Result<Patch, String> {
        let offset_base = ROM_PAGE * 0x01;
        let offset = offset_base + 0x602;

        let data = (0..8)
            .filter_map(|i| {
                let exists = save_badges.contains(&(i as u8));

                if exists {
                    Some(0x01 << i)
                } else {
                    None
                }
            })
            .fold(0, |acc, val| acc | val);

        Ok(Patch::new(&offset, &vec![data]))
    }

    /// Get save player ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::types::*;
    /// use pkmnapi_db::sav::*;
    /// use std::fs;
    /// # use std::env;
    /// # let sav_path = env::var("PKMN_SAV").expect("Set the PKMN_SAV environment variable to point to the SAV location");
    ///
    /// let sav_data = fs::read(sav_path).unwrap();
    /// let sav = Sav::new(&sav_data).unwrap();
    ///
    /// let player_id = sav.get_player_id().unwrap();
    ///
    /// assert_eq!(
    ///     player_id,
    ///     666
    /// );
    /// ```
    pub fn get_player_id(&self) -> Result<u16, String> {
        let offset_base = ROM_PAGE * 0x01;
        let offset = offset_base + 0x605;

        let save_player_id = {
            let mut cursor = Cursor::new(&self.sav[offset..(offset + 2)]);

            cursor.read_u16::<BigEndian>().unwrap_or(0)
        };

        Ok(save_player_id)
    }

    /// Set save player ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::patch::*;
    /// use pkmnapi_db::types::*;
    /// use pkmnapi_db::sav::*;
    /// use std::fs;
    /// # use std::env;
    /// # let sav_path = env::var("PKMN_SAV").expect("Set the PKMN_SAV environment variable to point to the SAV location");
    ///
    /// let sav_data = fs::read(sav_path).unwrap();
    /// let sav = Sav::new(&sav_data).unwrap();
    ///
    /// let patch = sav
    ///     .set_player_id(
    ///         &1234
    ///     )
    ///     .unwrap();
    ///
    /// assert_eq!(
    ///     patch,
    ///     Patch {
    ///         offset: 0x2605,
    ///         length: 0x02,
    ///         data: vec![0x04, 0xD2]
    ///     }
    /// );
    /// ```
    pub fn set_player_id(&self, save_player_id: &u16) -> Result<Patch, String> {
        let offset_base = ROM_PAGE * 0x01;
        let offset = offset_base + 0x605;

        let mut data = vec![];

        data.write_u16::<BigEndian>(*save_player_id).unwrap();

        Ok(Patch::new(&offset, &data))
    }

    /// Get save box items
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::types::*;
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
    pub fn get_box_items(&self) -> Result<Vec<SaveItem>, String> {
        let offset_base = ROM_PAGE * 0x01;
        let offset = offset_base + 0x7E6;
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
    /// use pkmnapi_db::types::*;
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
    pub fn set_box_items(&self, save_box_items: &Vec<SaveItem>) -> Result<Patch, String> {
        let offset_base = ROM_PAGE * 0x01;
        let offset = offset_base + 0x7E6;
        let max_len = 50;

        let item_count = save_box_items.len();

        if item_count > max_len {
            return Err(format!(
                "Length mismatch: should be {} items or fewer, found {}",
                max_len, item_count
            ));
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

    /// Get save current box
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::types::*;
    /// use pkmnapi_db::sav::*;
    /// use std::fs;
    /// # use std::env;
    /// # let sav_path = env::var("PKMN_SAV").expect("Set the PKMN_SAV environment variable to point to the SAV location");
    ///
    /// let sav_data = fs::read(sav_path).unwrap();
    /// let sav = Sav::new(&sav_data).unwrap();
    ///
    /// let current_box = sav.get_current_box().unwrap();
    ///
    /// assert_eq!(
    ///     current_box,
    ///     0x00
    /// );
    /// ```
    pub fn get_current_box(&self) -> Result<u8, String> {
        let offset_base = ROM_PAGE * 0x01;
        let offset = offset_base + 0x84C;

        let current_box = self.sav[offset] & 0x7F;

        Ok(current_box)
    }

    /// Set save current box
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::patch::*;
    /// use pkmnapi_db::types::*;
    /// use pkmnapi_db::sav::*;
    /// use std::fs;
    /// # use std::env;
    /// # let sav_path = env::var("PKMN_SAV").expect("Set the PKMN_SAV environment variable to point to the SAV location");
    ///
    /// let sav_data = fs::read(sav_path).unwrap();
    /// let sav = Sav::new(&sav_data).unwrap();
    ///
    /// let patch = sav
    ///     .set_current_box(
    ///         &0x01
    ///     )
    ///     .unwrap();
    ///
    /// assert_eq!(
    ///     patch,
    ///     Patch {
    ///         offset: 0x284C,
    ///         length: 0x01,
    ///         data: vec![0x81]
    ///     }
    /// );
    /// ```
    pub fn set_current_box(&self, current_box: &u8) -> Result<Patch, String> {
        let offset_base = ROM_PAGE * 0x01;
        let offset = offset_base + 0x84C;

        let data = vec![(*current_box & 0x7F) | 0x80];

        Ok(Patch::new(&offset, &data))
    }

    /// Get save coins
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::types::*;
    /// use pkmnapi_db::sav::*;
    /// use std::fs;
    /// # use std::env;
    /// # let sav_path = env::var("PKMN_SAV").expect("Set the PKMN_SAV environment variable to point to the SAV location");
    ///
    /// let sav_data = fs::read(sav_path).unwrap();
    /// let sav = Sav::new(&sav_data).unwrap();
    ///
    /// let coins = sav.get_coins().unwrap();
    ///
    /// assert_eq!(
    ///     coins,
    ///     0x1234
    /// );
    /// ```
    pub fn get_coins(&self) -> Result<u16, String> {
        let offset_base = ROM_PAGE * 0x01;
        let offset = offset_base + 0x850;

        let save_coins = {
            let mut cursor = Cursor::new(&self.sav[offset..(offset + 2)]);

            cursor.read_u16::<BigEndian>().unwrap_or(0)
        };

        Ok(save_coins)
    }

    /// Set save coins
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::patch::*;
    /// use pkmnapi_db::types::*;
    /// use pkmnapi_db::sav::*;
    /// use std::fs;
    /// # use std::env;
    /// # let sav_path = env::var("PKMN_SAV").expect("Set the PKMN_SAV environment variable to point to the SAV location");
    ///
    /// let sav_data = fs::read(sav_path).unwrap();
    /// let sav = Sav::new(&sav_data).unwrap();
    ///
    /// let patch = sav
    ///     .set_coins(
    ///         &0x9999
    ///     )
    ///     .unwrap();
    ///
    /// assert_eq!(
    ///     patch,
    ///     Patch {
    ///         offset: 0x2850,
    ///         length: 0x02,
    ///         data: vec![0x99, 0x99]
    ///     }
    /// );
    /// ```
    pub fn set_coins(&self, save_coins: &u16) -> Result<Patch, String> {
        let offset_base = ROM_PAGE * 0x01;
        let offset = offset_base + 0x850;

        let mut data = vec![];

        data.write_u16::<BigEndian>(*save_coins).unwrap();

        Ok(Patch::new(&offset, &data))
    }
}
