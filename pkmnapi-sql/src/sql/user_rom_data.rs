use diesel::prelude::*;

use crate::error::Result;
use crate::models::*;
use crate::utils;
use crate::{PgPooledConnection, PkmnapiSQL};

impl PkmnapiSQL {
    /// Select row in `rom_data`
    ///
    /// # Panics
    ///
    /// Panics if the `SECRET_KEY` environment variable is not set
    ///
    /// # Example
    ///
    /// ```
    /// # use std::process::Command;
    /// use pkmnapi_sql::*;
    ///
    /// let sql = PkmnapiSQL::new();
    ///
    /// let connection = sql.get_connection().unwrap();
    /// # let (new_user, access_token) = sql.insert_user(&connection, &String::from("foo@bar.com")).unwrap();
    /// # sql.update_user_rom_by_access_token(&connection, &access_token, &String::from("foo"), &vec![0x01, 0x02, 0x03, 0x04]).unwrap();
    /// let rom_datum = sql.select_user_rom_data_by_access_token(&connection, &access_token).unwrap().unwrap();
    ///
    /// assert_eq!(rom_datum.id.len(), 32);
    /// assert_eq!(rom_datum.name, String::from("foo"));
    /// assert_eq!(rom_datum.data, vec![0x01, 0x02, 0x03, 0x04]);
    /// # sql.revert_migration();
    /// ```
    pub fn select_user_rom_data_by_access_token(
        &self,
        connection: &PgPooledConnection,
        access_token: &String,
    ) -> Result<Option<RomData>> {
        use crate::schema::rom_data;
        use crate::schema::roms;
        use crate::schema::users;

        let access_token_hash = utils::hmac(&access_token);

        match users::table
            .filter(users::access_token_hash.eq(access_token_hash))
            .inner_join(roms::table.inner_join(rom_data::table))
            .select((rom_data::id, rom_data::name, rom_data::data))
            .first::<RomData>(connection)
        {
            Ok(rom_data) => Ok(Some(rom_data)),
            Err(diesel::result::Error::NotFound) => Ok(None),
            Err(e) => return Err(e.into()),
        }
    }
}
