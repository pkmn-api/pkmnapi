use crate::error::Result;
use crate::patch::*;
use crate::sav::Sav;
use crate::PkmnapiDB;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::Cursor;

impl Sav {
    /// Get save coins
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
    /// let coins = sav.get_coins().unwrap();
    ///
    /// assert_eq!(
    ///     coins,
    ///     0x1234
    /// );
    /// ```
    pub fn get_coins(&self) -> Result<u16> {
        let offset_base = PkmnapiDB::ROM_PAGE * 0x01;
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
    pub fn set_coins(&self, save_coins: &u16) -> Result<Patch> {
        let offset_base = PkmnapiDB::ROM_PAGE * 0x01;
        let offset = offset_base + 0x850;

        let mut data = vec![];

        data.write_u16::<BigEndian>(*save_coins).unwrap();

        Ok(Patch::new(&offset, &data))
    }
}
