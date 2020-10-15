use crate::error::Result;
use crate::patch::*;
use crate::sav::Sav;
use crate::PkmnapiDB;

impl Sav {
    /// Get save current box
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
    /// let current_box = sav.get_current_box().unwrap();
    ///
    /// assert_eq!(
    ///     current_box,
    ///     0x00
    /// );
    /// ```
    pub fn get_current_box(&self) -> Result<u8> {
        let offset_base = PkmnapiDB::ROM_PAGE * 0x01;
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
    pub fn set_current_box(&self, current_box: &u8) -> Result<Patch> {
        let offset_base = PkmnapiDB::ROM_PAGE * 0x01;
        let offset = offset_base + 0x84C;

        let data = vec![(*current_box & 0x7F) | 0x80];

        Ok(Patch::new(&offset, &data))
    }
}
