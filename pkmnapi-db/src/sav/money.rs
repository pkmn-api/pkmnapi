use crate::error;
use crate::patch::*;
use crate::sav::Sav;
use crate::PkmnapiDB;

impl Sav {
    /// Get save money
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
    /// let money = sav.get_money().unwrap();
    ///
    /// assert_eq!(
    ///     money,
    ///     123456
    /// );
    /// ```
    pub fn get_money(&self) -> Result<u32, error::Error> {
        let offset_base = PkmnapiDB::ROM_PAGE * 0x01;
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
    pub fn set_money(&self, money: &u32) -> Result<Patch, error::Error> {
        let offset_base = PkmnapiDB::ROM_PAGE * 0x01;
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
}
