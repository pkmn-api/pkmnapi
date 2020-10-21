use diesel::prelude::*;
use diesel::result::{DatabaseErrorKind, Error::DatabaseError};

use crate::error::{self, Result};
use crate::models::*;
use crate::utils;
use crate::{PgPooledConnection, PkmnapiSQL};

impl PkmnapiSQL {
    /// Select row from `users` by ID
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
    /// # let (new_user, _access_token) = sql.insert_user(&connection, &String::from("foo@bar.com")).unwrap();
    /// let user = sql.select_user_by_id(&connection, &new_user.id).unwrap().unwrap();
    ///
    /// assert_eq!(user.id, String::from("foo@bar.com"));
    /// assert_eq!(user.date_create.len(), 20);
    /// assert_eq!(user.date_expire.len(), 20);
    /// assert_eq!(user.access_token_hash.len(), 64);
    /// assert_eq!(user.delete_code, None);
    /// assert_eq!(user.rom_id, None);
    /// assert_eq!(user.sav_id, None);
    /// # sql.revert_migration();
    /// ```
    pub fn select_user_by_id(
        &self,
        connection: &PgPooledConnection,
        id: &String,
    ) -> Result<Option<User>> {
        use crate::schema::users;

        match users::table
            .filter(users::id.eq(id))
            .select((
                users::id,
                users::date_create,
                users::date_expire,
                users::access_token_hash,
                users::delete_code,
                users::rom_id,
                users::sav_id,
            ))
            .first::<User>(connection)
        {
            Ok(user) => Ok(Some(user)),
            Err(diesel::result::Error::NotFound) => Ok(None),
            Err(e) => return Err(e.into()),
        }
    }

    /// Select row from `users` by access_token
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
    /// let user = sql.select_user_by_access_token(&connection, &access_token).unwrap().unwrap();
    ///
    /// assert_eq!(user.id, String::from("foo@bar.com"));
    /// assert_eq!(user.date_create.len(), 20);
    /// assert_eq!(user.date_expire.len(), 20);
    /// assert_eq!(user.access_token_hash.len(), 64);
    /// assert_eq!(user.delete_code, None);
    /// assert_eq!(user.rom_id, None);
    /// assert_eq!(user.sav_id, None);
    /// # sql.revert_migration();
    /// ```
    pub fn select_user_by_access_token(
        &self,
        connection: &PgPooledConnection,
        access_token: &String,
    ) -> Result<Option<User>> {
        use crate::schema::users;

        let access_token_hash = utils::hmac(&access_token);

        match users::table
            .filter(users::access_token_hash.eq(access_token_hash))
            .select((
                users::id,
                users::date_create,
                users::date_expire,
                users::access_token_hash,
                users::delete_code,
                users::rom_id,
                users::sav_id,
            ))
            .first::<User>(connection)
        {
            Ok(user) => Ok(Some(user)),
            Err(diesel::result::Error::NotFound) => Ok(None),
            Err(e) => return Err(e.into()),
        }
    }

    /// Insert row into `users`
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
    /// let (new_user, access_token) = sql
    ///     .insert_user(&connection, &String::from("foo@bar.com"))
    ///     .unwrap();
    ///
    /// assert_eq!(new_user.id, String::from("foo@bar.com"));
    /// assert_eq!(new_user.date_create.len(), 20);
    /// assert_eq!(new_user.date_expire.len(), 20);
    /// assert_eq!(new_user.access_token_hash.len(), 64);
    /// assert_eq!(new_user.delete_code, None);
    /// assert_eq!(new_user.rom_id, None);
    /// assert_eq!(new_user.sav_id, None);
    /// assert_eq!(access_token.len(), 64);
    /// # sql.revert_migration();
    /// ```
    pub fn insert_user(
        &self,
        connection: &PgPooledConnection,
        id: &String,
    ) -> Result<(User, String)> {
        use crate::schema::users;

        let (new_user, access_token) = NewUser::new(&id);

        match diesel::insert_into(users::table)
            .values(&new_user)
            .execute(connection)
        {
            Ok(_) => match self.select_user_by_access_token(connection, &access_token) {
                Ok(Some(new_user)) => Ok((new_user, access_token)),
                Ok(None) => return Err(diesel::result::Error::NotFound.into()),
                Err(e) => return Err(e.into()),
            },
            Err(DatabaseError(DatabaseErrorKind::UniqueViolation, _)) => {
                let (updated_user, access_token) = NewUser::new(&id);

                diesel::update(users::table.filter(users::id.eq(id)))
                    .set((
                        users::access_token_hash.eq(&updated_user.access_token_hash),
                        users::date_create.eq(&updated_user.date_create),
                        users::date_expire.eq(&updated_user.date_expire),
                    ))
                    .execute(connection)
                    .ok();

                match self.select_user_by_access_token(&connection, &access_token) {
                    Ok(Some(new_user)) => Ok((new_user, access_token)),
                    Ok(None) => return Err(diesel::result::Error::NotFound.into()),
                    Err(e) => return Err(e.into()),
                }
            }
            Err(e) => return Err(e.into()),
        }
    }

    /// Update row in `users`
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
    /// # let (user, access_token) = sql.insert_user(&connection, &String::from("foo@bar.com")).unwrap();
    /// # let user_id = user.id;
    ///
    /// let user = sql.update_user_delete_code_by_id(&connection, &user_id).unwrap().unwrap();
    ///
    /// assert_eq!(user.id, String::from("foo@bar.com"));
    /// assert_eq!(user.date_create.len(), 20);
    /// assert_eq!(user.date_expire.len(), 20);
    /// assert_eq!(user.access_token_hash.len(), 64);
    /// assert_eq!(user.delete_code.unwrap().len(), 64);
    /// assert_eq!(user.rom_id, None);
    /// assert_eq!(user.sav_id, None);
    /// assert_eq!(access_token.len(), 64);
    /// # sql.revert_migration();
    /// ```
    pub fn update_user_delete_code_by_id(
        &self,
        connection: &PgPooledConnection,
        id: &String,
    ) -> Result<Option<User>> {
        use crate::schema::users;

        let delete_code = Some(utils::random_id(64));

        match connection.transaction::<_, error::Error, _>(|| {
            diesel::update(users::table.filter(users::id.eq(id)))
                .set(users::delete_code.eq(&delete_code))
                .execute(connection)?;

            self.select_user_by_id(connection, &id)
        }) {
            Ok(Some(user)) => Ok(Some(user)),
            Ok(None) => Ok(None),
            Err(e) => return Err(e.into()),
        }
    }

    /// Delete user from `users` by ID
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
    /// # let (user, access_token) = sql.insert_user(&connection, &String::from("foo@bar.com")).unwrap();
    /// # let user = sql.update_user_delete_code_by_id(&connection, &user.id).unwrap().unwrap();
    /// # let user_id = user.id;
    /// # let delete_code = user.delete_code.unwrap();
    /// # let _rom = sql.update_user_rom_by_access_token(&connection, &access_token, &user_id, &vec![0x01, 0x02, 0x03, 0x04]).unwrap();
    /// # let _rom_patch = sql.insert_rom_patch(&connection, &access_token, &vec![0x01, 0x02, 0x03, 0x04], None).unwrap();
    ///
    /// sql.delete_user_by_id(&connection, &user_id, &delete_code).unwrap();
    /// # sql.revert_migration();
    /// ```
    pub fn delete_user_by_id(
        &self,
        connection: &PgPooledConnection,
        id: &String,
        delete_code: &String,
    ) -> Result<()> {
        use crate::schema::rom_patches;
        use crate::schema::sav_patches;
        use crate::schema::users;

        match connection.transaction::<_, diesel::result::Error, _>(|| {
            match diesel::delete(rom_patches::table.filter(rom_patches::user_id.eq(id)))
                .execute(connection)
            {
                Ok(_) => {}
                Err(_) => return Err(diesel::result::Error::RollbackTransaction),
            };

            match diesel::delete(sav_patches::table.filter(sav_patches::user_id.eq(id)))
                .execute(connection)
            {
                Ok(_) => {}
                Err(_) => return Err(diesel::result::Error::RollbackTransaction),
            };

            diesel::delete(
                users::table
                    .filter(users::id.eq(id))
                    .filter(users::delete_code.eq(delete_code)),
            )
            .execute(connection)
        }) {
            Ok(_) => Ok(()),
            Err(e) => return Err(e.into()),
        }
    }
}
