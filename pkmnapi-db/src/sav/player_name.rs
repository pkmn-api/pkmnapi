use crate::error::{self, Result};
use crate::patch::*;
use crate::sav::Sav;
use crate::string::*;
use crate::PkmnapiDB;

impl Sav {
    /// Get save player name
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::string::*;
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
    pub fn get_player_name(&self) -> Result<SavePlayerName> {
        let offset_base = PkmnapiDB::ROM_PAGE * 0x01;
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
    pub fn set_player_name(&self, save_player_name: &SavePlayerName) -> Result<Patch> {
        let offset_base = PkmnapiDB::ROM_PAGE * 0x01;
        let offset = offset_base + 0x598;

        let save_player_name_raw = save_player_name.to_raw();
        let save_player_name_raw_len = save_player_name_raw.len();
        let max_len = 0x0A;

        if save_player_name_raw_len > max_len {
            return Err(error::Error::SavPlayerNameWrongSize(
                max_len,
                save_player_name_raw_len,
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
}

/// Save player name
///
/// # Example
///
/// ```
/// use pkmnapi_db::sav::*;
/// use pkmnapi_db::string::*;
///
/// let sav = vec![0x80, 0x81, 0x82, 0x50];
/// let save_player_name = SavePlayerName::from(&sav[..]);
///
/// assert_eq!(
///     save_player_name,
///     SavePlayerName {
///         name: ROMString::from("ABC")
///     }
/// );
/// ```
#[derive(Debug, PartialEq)]
pub struct SavePlayerName {
    pub name: ROMString,
}

impl From<&[u8]> for SavePlayerName {
    /// Convert &[u8] to SavePlayerName
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::sav::*;
    /// use pkmnapi_db::string::*;
    ///
    /// let sav = vec![0x80, 0x81, 0x82, 0x50];
    /// let save_player_name = SavePlayerName::from(&sav[..]);
    ///
    /// assert_eq!(
    ///     save_player_name,
    ///     SavePlayerName {
    ///         name: ROMString::from("ABC")
    ///     }
    /// );
    /// ```
    fn from(sav: &[u8]) -> Self {
        let name_end_index = sav.iter().position(|&r| r == 0x50).unwrap_or(sav.len());

        let name = ROMString::new(&sav[..name_end_index]);

        SavePlayerName { name }
    }
}

impl SavePlayerName {
    /// Save player name to raw bytes
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::sav::*;
    /// use pkmnapi_db::string::*;
    ///
    /// let save_player_name = SavePlayerName {
    ///     name: ROMString::from("ABC"),
    /// };
    ///
    /// let raw = save_player_name.to_raw();
    ///
    /// assert_eq!(raw, vec![0x80, 0x81, 0x82]);
    /// ```
    pub fn to_raw(&self) -> Vec<u8> {
        self.name.value[..].to_vec()
    }
}
