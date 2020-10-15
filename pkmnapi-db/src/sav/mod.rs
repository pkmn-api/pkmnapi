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

mod badges;
mod bag_items;
mod box_items;
mod coins;
mod current_box;
mod money;
mod options;
mod player_id;
mod player_name;
mod pokemon_owned;
mod pokemon_seen;
mod rival_name;

pub use badges::*;
pub use bag_items::*;
pub use box_items::*;
pub use coins::*;
pub use current_box::*;
pub use money::*;
pub use options::*;
pub use player_id::*;
pub use player_name::*;
pub use pokemon_owned::*;
pub use pokemon_seen::*;
pub use rival_name::*;

use crate::error::{self, Result};
use crate::patch::*;
use crate::PkmnapiDB;
use std::num::Wrapping;

#[derive(Debug, PartialEq)]
pub struct Sav {
    pub sav: Vec<u8>,
}

impl Sav {
    /// Create sav from an array of bytes
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
    /// assert_eq!(sav.sav.len(), 0x8000);
    pub fn new(sav: &Vec<u8>) -> Result<Self> {
        let sav_len = sav.len();
        let expected_sav_len = 0x8000;

        if sav_len != expected_sav_len {
            return Err(error::Error::SavWrongSize(expected_sav_len, sav_len));
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
        let offset_base = PkmnapiDB::ROM_PAGE * 0x01;
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
    pub fn generate_checksum(&self) -> Result<Patch> {
        let offset_base = PkmnapiDB::ROM_PAGE * 0x01;
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
}
