use crate::error::Result;
use crate::patch::*;
use crate::sav::Sav;
use crate::PkmnapiDB;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::Cursor;

impl Sav {
    /// Get save player ID
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
    /// let player_id = sav.get_player_id().unwrap();
    ///
    /// assert_eq!(
    ///     player_id,
    ///     666
    /// );
    /// ```
    pub fn get_player_id(&self) -> Result<u16> {
        let offset_base = PkmnapiDB::ROM_PAGE * 0x01;
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
    pub fn set_player_id(&self, save_player_id: &u16) -> Result<Patch> {
        let offset_base = PkmnapiDB::ROM_PAGE * 0x01;
        let offset = offset_base + 0x605;

        let mut data = vec![];

        data.write_u16::<BigEndian>(*save_player_id).unwrap();

        Ok(Patch::new(&offset, &data))
    }
}
