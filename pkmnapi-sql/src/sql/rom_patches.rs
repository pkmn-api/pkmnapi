use diesel::prelude::*;

use crate::error::{self, Result};
use crate::models::*;
use crate::utils;
use crate::{PgPooledConnection, PkmnapiSQL};

impl PkmnapiSQL {
    /// Select row in `rom_patches` by ID
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
    /// # let new_rom_patch = sql.insert_rom_patch(&connection, &access_token, &vec![0x01, 0x02, 0x03, 0x04], None).unwrap();
    /// # let id = new_rom_patch.id;
    /// let patch = sql.select_rom_patch_by_id(&connection, &access_token, &id).unwrap().unwrap();
    ///
    /// assert_eq!(patch.id.len(), 32);
    /// assert_eq!(patch.date_create.len(), 20);
    /// assert_eq!(patch.data, vec![0x01, 0x02, 0x03, 0x04]);
    /// assert_eq!(patch.description, None);
    /// assert_eq!(patch.etag.len(), 36);
    /// # sql.revert_migration();
    /// ```
    pub fn select_rom_patch_by_id(
        &self,
        connection: &PgPooledConnection,
        access_token: &String,
        id: &String,
    ) -> Result<Option<RomPatch>> {
        use crate::schema::rom_patches;
        use crate::schema::users;

        let access_token_hash = utils::hmac(&access_token);

        match users::table
            .filter(users::access_token_hash.eq(access_token_hash))
            .inner_join(rom_patches::table)
            .filter(rom_patches::id.eq(id))
            .select((
                rom_patches::id,
                rom_patches::date_create,
                rom_patches::data,
                rom_patches::description,
                rom_patches::etag,
            ))
            .first::<RomPatch>(connection)
        {
            Ok(patch) => Ok(Some(patch)),
            Err(diesel::result::Error::NotFound) => Ok(None),
            Err(e) => return Err(e.into()),
        }
    }

    /// Select rows in `rom_patches` by access token
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
    /// # sql.insert_rom_patch(&connection, &access_token, &vec![0x01, 0x02, 0x03, 0x04], None).unwrap();
    /// let rom_patches = sql.select_rom_patches_by_access_token(&connection, &access_token).unwrap();
    /// let patch = &rom_patches[0];
    ///
    /// assert_eq!(patch.id.len(), 32);
    /// assert_eq!(patch.date_create.len(), 20);
    /// assert_eq!(patch.data, vec![0x01, 0x02, 0x03, 0x04]);
    /// assert_eq!(patch.description, None);
    /// assert_eq!(patch.etag.len(), 36);
    /// # sql.revert_migration();
    /// ```
    pub fn select_rom_patches_by_access_token(
        &self,
        connection: &PgPooledConnection,
        access_token: &String,
    ) -> Result<Vec<RomPatch>> {
        use crate::schema::rom_patches;
        use crate::schema::users;

        let access_token_hash = utils::hmac(&access_token);

        match users::table
            .filter(users::access_token_hash.eq(access_token_hash))
            .inner_join(rom_patches::table)
            .select((
                rom_patches::id,
                rom_patches::date_create,
                rom_patches::data,
                rom_patches::description,
                rom_patches::etag,
            ))
            .get_results::<RomPatch>(connection)
        {
            Ok(rom_patches) => Ok(rom_patches),
            Err(e) => return Err(e.into()),
        }
    }

    /// Insert new row into `rom_patches`
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
    /// let new_rom_patch = sql
    ///     .insert_rom_patch(
    ///         &connection,
    ///         &access_token,
    ///         &vec![0x01, 0x02, 0x03, 0x04],
    ///         None
    ///     )
    ///     .unwrap();
    ///
    /// assert_eq!(new_rom_patch.id.len(), 32);
    /// assert_eq!(new_rom_patch.data, vec![0x01, 0x02, 0x03, 0x04]);
    /// # sql.revert_migration();
    /// ```
    pub fn insert_rom_patch(
        &self,
        connection: &PgPooledConnection,
        access_token: &String,
        data: &Vec<u8>,
        description: Option<String>,
    ) -> Result<RomPatch> {
        use crate::schema::rom_patches;

        let user = match self.select_user_by_access_token(&connection, &access_token) {
            Ok(Some(user)) => user,
            Ok(None) => return Err(diesel::result::Error::NotFound.into()),
            Err(e) => return Err(e.into()),
        };

        let new_rom_patch = NewRomPatch::new(&user.id, &data, description);

        match diesel::insert_into(rom_patches::table)
            .values(&new_rom_patch)
            .on_conflict_do_nothing()
            .execute(connection)
        {
            Ok(_) => {
                match self.select_rom_patch_by_id(connection, &access_token, &new_rom_patch.id) {
                    Ok(Some(patch)) => Ok(patch),
                    Ok(None) => return Err(diesel::result::Error::NotFound.into()),
                    Err(e) => return Err(e.into()),
                }
            }
            Err(e) => return Err(e.into()),
        }
    }

    /// Delete patch from `rom_patches` by ID
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
    /// # let new_rom_patch = sql.insert_rom_patch(&connection, &access_token, &vec![0x01, 0x02, 0x03, 0x04], None).unwrap();
    /// # let id = new_rom_patch.id;
    /// # let etag = new_rom_patch.etag;
    /// sql.delete_rom_patch_by_id(&connection, &access_token, &id, &etag).unwrap();
    /// # sql.revert_migration();
    /// ```
    pub fn delete_rom_patch_by_id(
        &self,
        connection: &PgPooledConnection,
        access_token: &String,
        id: &String,
        etag: &String,
    ) -> Result<()> {
        use crate::schema::rom_patches;

        connection.transaction::<_, error::Error, _>(|| {
            let user = match self.select_user_by_access_token(connection, &access_token) {
                Ok(Some(user)) => user,
                Ok(None) => return Err(diesel::result::Error::NotFound.into()),
                Err(e) => return Err(e.into()),
            };

            let patch = match self.select_rom_patch_by_id(connection, &access_token, id) {
                Ok(Some(patch)) => patch,
                Ok(None) => return Err(diesel::result::Error::NotFound.into()),
                Err(e) => return Err(e.into()),
            };

            if patch.etag != etag.to_owned() {
                return Err(error::Error::ETagError);
            }

            match diesel::delete(
                rom_patches::table
                    .filter(rom_patches::user_id.eq(user.id))
                    .filter(rom_patches::id.eq(id)),
            )
            .execute(connection)
            {
                Ok(_) => Ok(()),
                Err(e) => return Err(e.into()),
            }
        })
    }
}
