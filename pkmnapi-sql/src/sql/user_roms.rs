use diesel::prelude::*;

use crate::error::{self, Result};
use crate::models::*;
use crate::utils;
use crate::{PgPooledConnection, PkmnapiSQL};

impl PkmnapiSQL {
    /// Select row in `roms`
    ///
    /// # Panics
    ///
    /// Panics if the `SECRET_KEY` environment variable is not set
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_sql::*;
    /// # use std::process::Command;
    ///
    /// let sql = PkmnapiSQL::new();
    ///
    /// let connection = sql.get_connection().unwrap();
    /// # let (new_user, access_token) = sql.insert_user(&connection, &String::from("foo@bar.com")).unwrap();
    /// # sql.update_user_rom_by_access_token(&connection, &access_token, &String::from("foo"), &vec![0x01, 0x02, 0x03, 0x04]).unwrap();
    /// let rom = sql.select_user_rom_by_access_token(&connection, &access_token).unwrap().unwrap();
    ///
    /// assert_eq!(rom.id.len(), 32);
    /// assert_eq!(rom.date_create.len(), 20);
    /// assert_eq!(rom.name, String::from("foo"));
    /// assert_eq!(rom.etag.len(), 36);
    /// assert_eq!(rom.rom_data_id.len(), 32);
    /// # sql.revert_migration();
    /// ```
    pub fn select_user_rom_by_access_token(
        &self,
        connection: &PgPooledConnection,
        access_token: &String,
    ) -> Result<Option<Rom>> {
        use crate::schema::roms;
        use crate::schema::users;

        let access_token_hash = utils::hmac(&access_token);

        match users::table
            .filter(users::access_token_hash.eq(access_token_hash))
            .inner_join(roms::table)
            .select((
                roms::id,
                roms::date_create,
                roms::name,
                roms::etag,
                roms::rom_data_id,
            ))
            .first::<Rom>(connection)
        {
            Ok(rom) => Ok(Some(rom)),
            Err(diesel::result::Error::NotFound) => Ok(None),
            Err(e) => return Err(e.into()),
        }
    }

    /// Update row in `roms`
    ///
    /// # Panics
    ///
    /// Panics if the `SECRET_KEY` environment variable is not set
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_sql::*;
    /// # use std::process::Command;
    ///
    /// let sql = PkmnapiSQL::new();
    ///
    /// let connection = sql.get_connection().unwrap();
    /// # let (new_user, access_token) = sql.insert_user(&connection, &String::from("foo@bar.com")).unwrap();
    /// let rom = sql
    ///     .update_user_rom_by_access_token(
    ///         &connection,
    ///         &access_token,
    ///         &String::from("foo"),
    ///         &vec![0x01, 0x02, 0x03, 0x04],
    ///     )
    ///     .unwrap();
    ///
    /// assert_eq!(rom.id.len(), 32);
    /// assert_eq!(rom.name, String::from("foo"));
    /// assert_eq!(rom.etag.len(), 36);
    /// assert_eq!(rom.rom_data_id.len(), 32);
    /// # sql.revert_migration();
    /// ```
    pub fn update_user_rom_by_access_token(
        &self,
        connection: &PgPooledConnection,
        access_token: &String,
        name: &String,
        data: &Vec<u8>,
    ) -> Result<Rom> {
        use crate::schema::users;

        match connection.transaction::<_, diesel::result::Error, _>(|| {
            let new_rom = match self.insert_rom(connection, &name, &data) {
                Ok(new_rom) => new_rom,
                Err(e) => return Err(e.into_inner()),
            };
            let user = match self.select_user_by_access_token(connection, &access_token) {
                Ok(Some(user)) => user,
                Ok(None) => return Err(diesel::result::Error::NotFound),
                Err(e) => return Err(e.into_inner()),
            };

            let rom_id = match user.rom_id {
                Some(_) => return Err(diesel::result::Error::RollbackTransaction),
                None => new_rom.id,
            };

            let access_token_hash = utils::hmac(&access_token);

            diesel::update(users::table.filter(users::access_token_hash.eq(access_token_hash)))
                .set(users::rom_id.eq(&rom_id))
                .execute(connection)?;

            match self.select_rom_by_id(connection, &rom_id) {
                Ok(Some(rom)) => Ok(rom),
                Ok(None) => return Err(diesel::result::Error::NotFound),
                Err(e) => return Err(e.into_inner()),
            }
        }) {
            Ok(rom) => Ok(rom),
            Err(e) => return Err(e.into()),
        }
    }

    /// Update row in `roms`
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
    /// # let rom = sql.update_user_rom_by_access_token(&connection, &access_token, &String::from("foo"), &vec![0x01, 0x02, 0x03, 0x04]).unwrap();
    /// # let etag = rom.etag;
    /// sql.delete_user_rom_by_access_token(&connection, &access_token, &etag);
    /// # sql.revert_migration();
    /// ```
    pub fn delete_user_rom_by_access_token(
        &self,
        connection: &PgPooledConnection,
        access_token: &String,
        etag: &String,
    ) -> Result<()> {
        use crate::schema::users;

        connection.transaction::<_, error::Error, _>(|| {
            let rom = match self.select_user_rom_by_access_token(connection, &access_token) {
                Ok(Some(rom)) => rom,
                Ok(None) => return Err(diesel::result::Error::NotFound.into()),
                Err(e) => return Err(e.into()),
            };

            if rom.etag != etag.to_owned() {
                return Err(error::Error::ETagError);
            }

            let access_token_hash = utils::hmac(&access_token);
            let rom_id: Option<String> = None;

            diesel::update(users::table.filter(users::access_token_hash.eq(access_token_hash)))
                .set(users::rom_id.eq(&rom_id))
                .execute(connection)?;

            self.delete_rom_by_id(connection, &rom.id)
        })
    }
}
