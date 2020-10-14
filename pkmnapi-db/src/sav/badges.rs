use crate::error;
use crate::patch::*;
use crate::sav::Sav;
use crate::PkmnapiDB;

impl Sav {
    /// Get save badges
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
    /// let badges = sav.get_badges().unwrap();
    ///
    /// assert_eq!(badges, vec![0x00]);
    /// ```
    pub fn get_badges(&self) -> Result<Vec<u8>, error::Error> {
        let offset_base = PkmnapiDB::ROM_PAGE * 0x01;
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
    pub fn set_badges(&self, save_badges: &Vec<u8>) -> Result<Patch, error::Error> {
        let offset_base = PkmnapiDB::ROM_PAGE * 0x01;
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
}
