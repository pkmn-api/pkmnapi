use diesel::prelude::*;

use crate::error::{self, Result};
use crate::models::*;
use crate::utils;
use crate::{PgPooledConnection, PkmnapiSQL};

impl PkmnapiSQL {
    /// Select row in `savs`
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
    /// # sql.update_user_sav_by_access_token(&connection, &access_token, &vec![0x01, 0x02, 0x03, 0x04]).unwrap();
    /// let sav = sql.select_user_sav_by_access_token(&connection, &access_token).unwrap().unwrap();
    ///
    /// assert_eq!(sav.id.len(), 32);
    /// assert_eq!(sav.date_create.len(), 20);
    /// assert_eq!(sav.data, vec![0x01, 0x02, 0x03, 0x04]);
    /// assert_eq!(sav.etag.len(), 36);
    /// # sql.revert_migration();
    /// ```
    pub fn select_user_sav_by_access_token(
        &self,
        connection: &PgPooledConnection,
        access_token: &String,
    ) -> Result<Option<Sav>> {
        use crate::schema::savs;
        use crate::schema::users;

        let access_token_hash = utils::hmac(&access_token);

        match users::table
            .filter(users::access_token_hash.eq(access_token_hash))
            .inner_join(savs::table)
            .select((savs::id, savs::date_create, savs::data, savs::etag))
            .first::<Sav>(connection)
        {
            Ok(sav) => Ok(Some(sav)),
            Err(diesel::result::Error::NotFound) => Ok(None),
            Err(e) => return Err(e.into()),
        }
    }

    /// Update row in `savs`
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
    /// let sav = sql
    ///     .update_user_sav_by_access_token(
    ///         &connection,
    ///         &access_token,
    ///         &vec![0x01, 0x02, 0x03, 0x04],
    ///     )
    ///     .unwrap();
    ///
    /// assert_eq!(sav.id.len(), 32);
    /// assert_eq!(sav.date_create.len(), 20);
    /// assert_eq!(sav.data, vec![0x01, 0x02, 0x03, 0x04]);
    /// assert_eq!(sav.etag.len(), 36);
    /// # sql.revert_migration();
    /// ```
    pub fn update_user_sav_by_access_token(
        &self,
        connection: &PgPooledConnection,
        access_token: &String,
        data: &Vec<u8>,
    ) -> Result<Sav> {
        use crate::schema::users;

        match connection.transaction::<_, diesel::result::Error, _>(|| {
            let new_sav = match self.insert_sav(connection, &data) {
                Ok(new_sav) => new_sav,
                Err(e) => return Err(e.into_inner()),
            };
            let user = match self.select_user_by_access_token(connection, &access_token) {
                Ok(Some(user)) => user,
                Ok(None) => return Err(diesel::result::Error::NotFound),
                Err(e) => return Err(e.into_inner()),
            };

            let sav_id = match user.sav_id {
                Some(_) => return Err(diesel::result::Error::RollbackTransaction),
                None => new_sav.id,
            };

            let access_token_hash = utils::hmac(&access_token);

            diesel::update(users::table.filter(users::access_token_hash.eq(access_token_hash)))
                .set(users::sav_id.eq(&sav_id))
                .execute(connection)?;

            match self.select_sav_by_id(connection, &sav_id) {
                Ok(Some(sav)) => Ok(sav),
                Ok(None) => return Err(diesel::result::Error::NotFound),
                Err(e) => return Err(e.into_inner()),
            }
        }) {
            Ok(sav) => Ok(sav),
            Err(e) => return Err(e.into()),
        }
    }

    /// Update row in `savs`
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
    /// # let sav = sql.update_user_sav_by_access_token(&connection, &access_token, &vec![0x01, 0x02, 0x03, 0x04]).unwrap();
    /// # let etag = sav.etag;
    /// sql.delete_user_sav_by_access_token(&connection, &access_token, &etag);
    /// # sql.revert_migration();
    /// ```
    pub fn delete_user_sav_by_access_token(
        &self,
        connection: &PgPooledConnection,
        access_token: &String,
        etag: &String,
    ) -> Result<()> {
        use crate::schema::users;

        connection.transaction::<_, error::Error, _>(|| {
            let sav = match self.select_user_sav_by_access_token(connection, &access_token) {
                Ok(Some(sav)) => sav,
                Ok(None) => return Err(diesel::result::Error::NotFound.into()),
                Err(e) => return Err(e.into()),
            };

            if sav.etag != etag.to_owned() {
                return Err(error::Error::ETagError);
            }

            let access_token_hash = utils::hmac(&access_token);
            let sav_id: Option<String> = None;

            diesel::update(users::table.filter(users::access_token_hash.eq(access_token_hash)))
                .set(users::sav_id.eq(&sav_id))
                .execute(connection)?;

            self.delete_sav_by_id(connection, &sav.id)
        })
    }
}
