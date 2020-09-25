//! Pkmnapi SQL module
//!
//! # Example
//!
//! ```
//! use pkmnapi_sql::*;
//!
//! let sql = PkmnapiSQL::new();
//! ```

#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod error;
pub mod models;
pub mod schema;
pub mod utils;

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::result::{DatabaseErrorKind, Error::DatabaseError};
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

use crate::models::*;

pub type SqlitePool = Pool<ConnectionManager<SqliteConnection>>;
pub type SqlitePooledConnection = PooledConnection<ConnectionManager<SqliteConnection>>;

/// PkmnapiSQL
///
/// # Example
///
/// ```
/// use pkmnapi_sql::*;
///
/// let sql = PkmnapiSQL::new();
/// ```
pub struct PkmnapiSQL {
    connection: SqlitePool,
}

impl PkmnapiSQL {
    /// Create new PkmnapiSQL
    ///
    /// # Panics
    ///
    /// Panics if the `DATABASE_URL` environment variable is not set
    ///
    /// Also panics if it was unable to create a database connection pool
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_sql::*;
    ///
    /// let sql = PkmnapiSQL::new();
    /// ```
    pub fn new() -> PkmnapiSQL {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let manager = ConnectionManager::<SqliteConnection>::new(database_url);
        let connection = Pool::builder()
            .build(manager)
            .expect("Failed to create database connection pool");

        PkmnapiSQL { connection }
    }

    pub fn get_connection(&self) -> Result<SqlitePooledConnection, error::Error> {
        match self.connection.get() {
            Ok(connection) => Ok(connection),
            Err(_) => return Err(diesel::result::Error::NotFound.into()),
        }
    }

    /// Select all rows from `rom_data` by ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_sql::*;
    /// # use std::process::Command;
    /// # use std::fs;
    /// # use std::env;
    /// # env::set_var("DATABASE_URL", "test.db");
    /// # Command::new("diesel").args(&["migration", "run"]).output();
    ///
    /// let sql = PkmnapiSQL::new();
    ///
    /// let connection = sql.get_connection().unwrap();
    /// # let new_rom_data = sql.insert_rom_data(&connection, &String::from("foo"), &vec![0x01, 0x02, 0x03, 0x04]).unwrap();
    /// # let id = new_rom_data.id;
    /// let rom_datum = sql.select_rom_data_by_id(&connection, &id).unwrap().unwrap();
    ///
    /// assert_eq!(rom_datum.id.len(), 32);
    /// assert_eq!(rom_datum.name, String::from("foo"));
    /// assert_eq!(rom_datum.data, vec![0x01, 0x02, 0x03, 0x04]);
    /// # fs::remove_file("test.db");
    /// ```
    pub fn select_rom_data_by_id(
        &self,
        connection: &SqlitePooledConnection,
        id: &String,
    ) -> Result<Option<RomData>, error::Error> {
        use crate::schema::rom_data;

        match rom_data::table
            .filter(rom_data::id.eq(id))
            .first::<RomData>(connection)
        {
            Ok(rom_data) => Ok(Some(rom_data)),
            Err(diesel::result::Error::NotFound) => Ok(None),
            Err(e) => return Err(e.into()),
        }
    }

    /// Insert new row into `rom_data`
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_sql::*;
    /// # use std::process::Command;
    /// # use std::fs;
    /// # use std::env;
    /// # env::set_var("DATABASE_URL", "test.db");
    /// # Command::new("diesel").args(&["migration", "run"]).output();
    ///
    /// let sql = PkmnapiSQL::new();
    ///
    /// let connection = sql.get_connection().unwrap();
    /// let new_rom_data = sql
    ///     .insert_rom_data(
    ///         &connection,
    ///         &String::from("foo"),
    ///         &vec![0x01, 0x02, 0x03, 0x04],
    ///     )
    ///     .unwrap();
    ///
    /// assert_eq!(new_rom_data.id.len(), 32);
    /// assert_eq!(new_rom_data.name, String::from("foo"));
    /// assert_eq!(new_rom_data.data, vec![0x01, 0x02, 0x03, 0x04]);
    /// # fs::remove_file("test.db");
    /// ```
    pub fn insert_rom_data(
        &self,
        connection: &SqlitePooledConnection,
        name: &String,
        data: &Vec<u8>,
    ) -> Result<RomData, error::Error> {
        use crate::schema::rom_data;

        let new_rom_data = NewRomData::new(&name, &data);

        match diesel::insert_or_ignore_into(rom_data::table)
            .values(&new_rom_data)
            .execute(connection)
        {
            Ok(_) => match self.select_rom_data_by_id(connection, &new_rom_data.id) {
                Ok(Some(rom_data)) => Ok(rom_data),
                Ok(None) => return Err(diesel::result::Error::NotFound.into()),
                Err(e) => return Err(e.into()),
            },
            Err(e) => return Err(e.into()),
        }
    }

    /// Select row from `roms` by ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_sql::*;
    /// # use std::process::Command;
    /// # use std::fs;
    /// # use std::env;
    /// # env::set_var("DATABASE_URL", "test.db");
    /// # Command::new("diesel").args(&["migration", "run"]).output();
    ///
    /// let sql = PkmnapiSQL::new();
    ///
    /// let connection = sql.get_connection().unwrap();
    /// # let new_rom = sql.insert_rom(&connection, &String::from("foo"), &vec![0x01, 0x02, 0x03, 0x04]).unwrap();
    /// # let id = new_rom.id;
    /// let rom = sql.select_rom_by_id(&connection, &id).unwrap().unwrap();
    ///
    /// assert_eq!(rom.id.len(), 32);
    /// assert_eq!(rom.date_create.len(), 20);
    /// assert_eq!(rom.name, String::from("foo"));
    /// assert_eq!(rom.etag.len(), 36);
    /// assert_eq!(rom.rom_data_id.len(), 32);
    /// # fs::remove_file("test.db");
    /// ```
    pub fn select_rom_by_id(
        &self,
        connection: &SqlitePooledConnection,
        id: &String,
    ) -> Result<Option<Rom>, error::Error> {
        use crate::schema::rom_data;
        use crate::schema::roms;

        match roms::table
            .filter(roms::id.eq(id))
            .inner_join(rom_data::table)
            .select((
                roms::id,
                roms::date_create,
                roms::name,
                roms::etag,
                rom_data::id,
            ))
            .first::<Rom>(connection)
        {
            Ok(rom) => Ok(Some(rom)),
            Err(diesel::result::Error::NotFound) => Ok(None),
            Err(e) => return Err(e.into()),
        }
    }

    /// Insert rows into `roms`
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_sql::*;
    /// # use std::process::Command;
    /// # use std::fs;
    /// # use std::env;
    /// # env::set_var("DATABASE_URL", "test.db");
    /// # Command::new("diesel").args(&["migration", "run"]).output();
    ///
    /// let sql = PkmnapiSQL::new();
    ///
    /// let connection = sql.get_connection().unwrap();
    /// let new_rom = sql
    ///     .insert_rom(
    ///         &connection,
    ///         &String::from("foo"),
    ///         &vec![0x01, 0x02, 0x03, 0x04],
    ///     )
    ///     .unwrap();
    ///
    /// assert_eq!(new_rom.id.len(), 32);
    /// assert_eq!(new_rom.name, String::from("foo"));
    /// assert_eq!(new_rom.etag.len(), 36);
    /// assert_eq!(new_rom.rom_data_id.len(), 32);
    /// # fs::remove_file("test.db");
    /// ```
    pub fn insert_rom(
        &self,
        connection: &SqlitePooledConnection,
        name: &String,
        data: &Vec<u8>,
    ) -> Result<Rom, error::Error> {
        use crate::schema::roms;

        match connection.transaction::<_, error::Error, _>(|| {
            let new_rom_data = self.insert_rom_data(connection, &name, &data)?;
            let new_rom = NewRom::new(&name, &new_rom_data.id);

            match diesel::insert_into(roms::table)
                .values(&new_rom)
                .execute(connection)
            {
                Ok(_) => Ok(new_rom.id),
                Err(e) => return Err(e.into()),
            }
        }) {
            Ok(new_rom_id) => match self.select_rom_by_id(connection, &new_rom_id) {
                Ok(Some(rom)) => Ok(rom),
                Ok(None) => return Err(diesel::result::Error::NotFound.into()),
                Err(e) => return Err(e.into()),
            },
            Err(e) => return Err(e.into()),
        }
    }

    /// Delete rom from `roms` by ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_sql::*;
    /// # use std::process::Command;
    /// # use std::fs;
    /// # use std::env;
    /// # env::set_var("DATABASE_URL", "test.db");
    /// # Command::new("diesel").args(&["migration", "run"]).output();
    ///
    /// let sql = PkmnapiSQL::new();
    ///
    /// let connection = sql.get_connection().unwrap();
    /// # let new_rom = sql.insert_rom(&connection, &String::from("foo"), &vec![0x01, 0x02, 0x03, 0x04]).unwrap();
    /// # let id = new_rom.id;
    /// sql.delete_rom_by_id(&connection, &id).unwrap();
    /// # fs::remove_file("test.db");
    /// ```
    pub fn delete_rom_by_id(
        &self,
        connection: &SqlitePooledConnection,
        id: &String,
    ) -> Result<(), error::Error> {
        use crate::schema::roms;

        match diesel::delete(roms::table.filter(roms::id.eq(id))).execute(connection) {
            Ok(_) => Ok(()),
            Err(e) => return Err(e.into()),
        }
    }

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
    /// # use std::fs;
    /// # use std::env;
    /// # env::set_var("DATABASE_URL", "test.db");
    /// # Command::new("diesel").args(&["migration", "run"]).output();
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
    /// assert_eq!(user.rom_id, None);
    /// assert_eq!(user.sav_id, None);
    /// # fs::remove_file("test.db");
    /// ```
    pub fn select_user_by_id(
        &self,
        connection: &SqlitePooledConnection,
        id: &String,
    ) -> Result<Option<User>, error::Error> {
        use crate::schema::users;

        match users::table
            .filter(users::id.eq(id))
            .select((
                users::id,
                users::date_create,
                users::date_expire,
                users::access_token_hash,
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
    /// # use std::fs;
    /// # use std::env;
    /// # env::set_var("DATABASE_URL", "test.db");
    /// # Command::new("diesel").args(&["migration", "run"]).output();
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
    /// assert_eq!(user.rom_id, None);
    /// assert_eq!(user.sav_id, None);
    /// # fs::remove_file("test.db");
    /// ```
    pub fn select_user_by_access_token(
        &self,
        connection: &SqlitePooledConnection,
        access_token: &String,
    ) -> Result<Option<User>, error::Error> {
        use crate::schema::users;

        let access_token_hash = utils::hmac(&access_token);

        match users::table
            .filter(users::access_token_hash.eq(access_token_hash))
            .select((
                users::id,
                users::date_create,
                users::date_expire,
                users::access_token_hash,
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
    /// # use std::fs;
    /// # use std::env;
    /// # env::set_var("DATABASE_URL", "test.db");
    /// # Command::new("diesel").args(&["migration", "run"]).output();
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
    /// assert_eq!(new_user.rom_id, None);
    /// assert_eq!(new_user.sav_id, None);
    /// assert_eq!(access_token.len(), 64);
    /// # fs::remove_file("test.db");
    /// ```
    pub fn insert_user(
        &self,
        connection: &SqlitePooledConnection,
        id: &String,
    ) -> Result<(User, String), error::Error> {
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
    /// # use std::fs;
    /// # use std::env;
    /// # env::set_var("DATABASE_URL", "test.db");
    /// # Command::new("diesel").args(&["migration", "run"]).output();
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
    /// # fs::remove_file("test.db");
    /// ```
    pub fn select_user_rom_by_access_token(
        &self,
        connection: &SqlitePooledConnection,
        access_token: &String,
    ) -> Result<Option<Rom>, error::Error> {
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
    /// # use std::fs;
    /// # use std::env;
    /// # env::set_var("DATABASE_URL", "test.db");
    /// # Command::new("diesel").args(&["migration", "run"]).output();
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
    /// # fs::remove_file("test.db");
    /// ```
    pub fn update_user_rom_by_access_token(
        &self,
        connection: &SqlitePooledConnection,
        access_token: &String,
        name: &String,
        data: &Vec<u8>,
    ) -> Result<Rom, error::Error> {
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
    /// # use std::fs;
    /// # use std::env;
    /// use pkmnapi_sql::*;
    /// # env::set_var("DATABASE_URL", "test.db");
    /// # Command::new("diesel").args(&["migration", "run"]).output();
    ///
    /// let sql = PkmnapiSQL::new();
    ///
    /// let connection = sql.get_connection().unwrap();
    /// # let (new_user, access_token) = sql.insert_user(&connection, &String::from("foo@bar.com")).unwrap();
    /// # let rom = sql.update_user_rom_by_access_token(&connection, &access_token, &String::from("foo"), &vec![0x01, 0x02, 0x03, 0x04]).unwrap();
    /// # let etag = rom.etag;
    /// sql.delete_user_rom_by_access_token(&connection, &access_token, &etag);
    /// # fs::remove_file("test.db");
    /// ```
    pub fn delete_user_rom_by_access_token(
        &self,
        connection: &SqlitePooledConnection,
        access_token: &String,
        etag: &String,
    ) -> Result<(), error::Error> {
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
    /// # use std::fs;
    /// # use std::env;
    /// use pkmnapi_sql::*;
    /// # env::set_var("DATABASE_URL", "test.db");
    /// # Command::new("diesel").args(&["migration", "run"]).output();
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
    /// # fs::remove_file("test.db");
    /// ```
    pub fn select_user_rom_data_by_access_token(
        &self,
        connection: &SqlitePooledConnection,
        access_token: &String,
    ) -> Result<Option<RomData>, error::Error> {
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

    /// Select row in `rom_patches` by ID
    ///
    /// # Example
    ///
    /// ```
    /// # use std::process::Command;
    /// # use std::fs;
    /// # use std::env;
    /// use pkmnapi_sql::*;
    /// # env::set_var("DATABASE_URL", "test.db");
    /// # Command::new("diesel").args(&["migration", "run"]).output();
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
    /// # fs::remove_file("test.db");
    /// ```
    pub fn select_rom_patch_by_id(
        &self,
        connection: &SqlitePooledConnection,
        access_token: &String,
        id: &String,
    ) -> Result<Option<RomPatch>, error::Error> {
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
    /// # use std::fs;
    /// # use std::env;
    /// use pkmnapi_sql::*;
    /// # env::set_var("DATABASE_URL", "test.db");
    /// # Command::new("diesel").args(&["migration", "run"]).output();
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
    /// # fs::remove_file("test.db");
    /// ```
    pub fn select_rom_patches_by_access_token(
        &self,
        connection: &SqlitePooledConnection,
        access_token: &String,
    ) -> Result<Vec<RomPatch>, error::Error> {
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
    /// # use std::fs;
    /// # use std::env;
    /// # env::set_var("DATABASE_URL", "test.db");
    /// # Command::new("diesel").args(&["migration", "run"]).output();
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
    /// # fs::remove_file("test.db");
    /// ```
    pub fn insert_rom_patch(
        &self,
        connection: &SqlitePooledConnection,
        access_token: &String,
        data: &Vec<u8>,
        description: Option<String>,
    ) -> Result<RomPatch, error::Error> {
        use crate::schema::rom_patches;

        let user = match self.select_user_by_access_token(&connection, &access_token) {
            Ok(Some(user)) => user,
            Ok(None) => return Err(diesel::result::Error::NotFound.into()),
            Err(e) => return Err(e.into()),
        };

        let new_rom_patch = NewRomPatch::new(&user.id, &data, description);

        match diesel::insert_or_ignore_into(rom_patches::table)
            .values(&new_rom_patch)
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
    /// # use std::fs;
    /// # use std::env;
    /// # env::set_var("DATABASE_URL", "test.db");
    /// # Command::new("diesel").args(&["migration", "run"]).output();
    ///
    /// let sql = PkmnapiSQL::new();
    ///
    /// let connection = sql.get_connection().unwrap();
    /// # let (new_user, access_token) = sql.insert_user(&connection, &String::from("foo@bar.com")).unwrap();
    /// # let new_rom_patch = sql.insert_rom_patch(&connection, &access_token, &vec![0x01, 0x02, 0x03, 0x04], None).unwrap();
    /// # let id = new_rom_patch.id;
    /// # let etag = new_rom_patch.etag;
    /// sql.delete_rom_patch_by_id(&connection, &access_token, &id, &etag).unwrap();
    /// # fs::remove_file("test.db");
    /// ```
    pub fn delete_rom_patch_by_id(
        &self,
        connection: &SqlitePooledConnection,
        access_token: &String,
        id: &String,
        etag: &String,
    ) -> Result<(), error::Error> {
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

    /// Select row from `savs` by ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_sql::*;
    /// # use std::process::Command;
    /// # use std::fs;
    /// # use std::env;
    /// # env::set_var("DATABASE_URL", "test.db");
    /// # Command::new("diesel").args(&["migration", "run"]).output();
    ///
    /// let sql = PkmnapiSQL::new();
    ///
    /// let connection = sql.get_connection().unwrap();
    /// # let new_sav = sql.insert_sav(&connection, &vec![0x01, 0x02, 0x03, 0x04]).unwrap();
    /// # let id = new_sav.id;
    /// let sav = sql.select_sav_by_id(&connection, &id).unwrap().unwrap();
    ///
    /// assert_eq!(sav.id.len(), 32);
    /// assert_eq!(sav.date_create.len(), 20);
    /// assert_eq!(sav.data, vec![0x01, 0x02, 0x03, 0x04]);
    /// assert_eq!(sav.etag.len(), 36);
    /// # fs::remove_file("test.db");
    /// ```
    pub fn select_sav_by_id(
        &self,
        connection: &SqlitePooledConnection,
        id: &String,
    ) -> Result<Option<Sav>, error::Error> {
        use crate::schema::savs;

        match savs::table
            .filter(savs::id.eq(id))
            .select((savs::id, savs::date_create, savs::data, savs::etag))
            .first::<Sav>(connection)
        {
            Ok(sav) => Ok(Some(sav)),
            Err(diesel::result::Error::NotFound) => Ok(None),
            Err(e) => return Err(e.into()),
        }
    }

    /// Insert new row into `savs`
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_sql::*;
    /// # use std::process::Command;
    /// # use std::fs;
    /// # use std::env;
    /// # env::set_var("DATABASE_URL", "test.db");
    /// # Command::new("diesel").args(&["migration", "run"]).output();
    ///
    /// let sql = PkmnapiSQL::new();
    ///
    /// let connection = sql.get_connection().unwrap();
    /// let new_sav = sql
    ///     .insert_sav(&connection, &vec![0x01, 0x02, 0x03, 0x04])
    ///     .unwrap();
    ///
    /// assert_eq!(new_sav.id.len(), 32);
    /// assert_eq!(new_sav.date_create.len(), 20);
    /// assert_eq!(new_sav.data, vec![0x01, 0x02, 0x03, 0x04]);
    /// assert_eq!(new_sav.etag.len(), 36);
    /// # fs::remove_file("test.db");
    /// ```
    pub fn insert_sav(
        &self,
        connection: &SqlitePooledConnection,
        data: &Vec<u8>,
    ) -> Result<Sav, error::Error> {
        use crate::schema::savs;

        match connection.transaction::<_, error::Error, _>(|| {
            let new_sav = NewSav::new(data);

            match diesel::insert_into(savs::table)
                .values(&new_sav)
                .execute(connection)
            {
                Ok(_) => Ok(new_sav.id),
                Err(e) => return Err(e.into()),
            }
        }) {
            Ok(new_sav_id) => match self.select_sav_by_id(connection, &new_sav_id) {
                Ok(Some(sav)) => Ok(sav),
                Ok(None) => return Err(diesel::result::Error::NotFound.into()),
                Err(e) => return Err(e.into()),
            },
            Err(e) => return Err(e.into()),
        }
    }

    /// Delete sav from `savs` by ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_sql::*;
    /// # use std::process::Command;
    /// # use std::fs;
    /// # use std::env;
    /// # env::set_var("DATABASE_URL", "test.db");
    /// # Command::new("diesel").args(&["migration", "run"]).output();
    ///
    /// let sql = PkmnapiSQL::new();
    ///
    /// let connection = sql.get_connection().unwrap();
    /// # let new_sav = sql.insert_sav(&connection, &vec![0x01, 0x02, 0x03, 0x04]).unwrap();
    /// # let id = new_sav.id;
    /// sql.delete_sav_by_id(&connection, &id).unwrap();
    /// # fs::remove_file("test.db");
    /// ```
    pub fn delete_sav_by_id(
        &self,
        connection: &SqlitePooledConnection,
        id: &String,
    ) -> Result<(), error::Error> {
        use crate::schema::savs;

        match diesel::delete(savs::table.filter(savs::id.eq(id))).execute(connection) {
            Ok(_) => Ok(()),
            Err(e) => return Err(e.into()),
        }
    }

    /// Select row in `sav_patches` by ID
    ///
    /// # Example
    ///
    /// ```
    /// # use std::process::Command;
    /// # use std::fs;
    /// # use std::env;
    /// use pkmnapi_sql::*;
    /// # env::set_var("DATABASE_URL", "test.db");
    /// # Command::new("diesel").args(&["migration", "run"]).output();
    ///
    /// let sql = PkmnapiSQL::new();
    ///
    /// let connection = sql.get_connection().unwrap();
    /// # let (new_user, access_token) = sql.insert_user(&connection, &String::from("foo@bar.com")).unwrap();
    /// # let new_sav_patch = sql.insert_sav_patch(&connection, &access_token, &vec![0x01, 0x02, 0x03, 0x04], None).unwrap();
    /// # let id = new_sav_patch.id;
    /// let patch = sql.select_sav_patch_by_id(&connection, &access_token, &id).unwrap().unwrap();
    ///
    /// assert_eq!(patch.id.len(), 32);
    /// assert_eq!(patch.date_create.len(), 20);
    /// assert_eq!(patch.data, vec![0x01, 0x02, 0x03, 0x04]);
    /// assert_eq!(patch.description, None);
    /// assert_eq!(patch.etag.len(), 36);
    /// # fs::remove_file("test.db");
    /// ```
    pub fn select_sav_patch_by_id(
        &self,
        connection: &SqlitePooledConnection,
        access_token: &String,
        id: &String,
    ) -> Result<Option<SavPatch>, error::Error> {
        use crate::schema::sav_patches;
        use crate::schema::users;

        let access_token_hash = utils::hmac(&access_token);

        match users::table
            .filter(users::access_token_hash.eq(access_token_hash))
            .inner_join(sav_patches::table)
            .filter(sav_patches::id.eq(id))
            .select((
                sav_patches::id,
                sav_patches::date_create,
                sav_patches::data,
                sav_patches::description,
                sav_patches::etag,
            ))
            .first::<SavPatch>(connection)
        {
            Ok(patch) => Ok(Some(patch)),
            Err(diesel::result::Error::NotFound) => Ok(None),
            Err(e) => return Err(e.into()),
        }
    }

    /// Select rows in `sav_patches` by access token
    ///
    /// # Example
    ///
    /// ```
    /// # use std::process::Command;
    /// # use std::fs;
    /// # use std::env;
    /// use pkmnapi_sql::*;
    /// # env::set_var("DATABASE_URL", "test.db");
    /// # Command::new("diesel").args(&["migration", "run"]).output();
    ///
    /// let sql = PkmnapiSQL::new();
    ///
    /// let connection = sql.get_connection().unwrap();
    /// # let (new_user, access_token) = sql.insert_user(&connection, &String::from("foo@bar.com")).unwrap();
    /// # sql.insert_sav_patch(&connection, &access_token, &vec![0x01, 0x02, 0x03, 0x04], None).unwrap();
    /// let sav_patches = sql.select_sav_patches_by_access_token(&connection, &access_token).unwrap();
    /// let patch = &sav_patches[0];
    ///
    /// assert_eq!(patch.id.len(), 32);
    /// assert_eq!(patch.date_create.len(), 20);
    /// assert_eq!(patch.data, vec![0x01, 0x02, 0x03, 0x04]);
    /// assert_eq!(patch.description, None);
    /// assert_eq!(patch.etag.len(), 36);
    /// # fs::remove_file("test.db");
    /// ```
    pub fn select_sav_patches_by_access_token(
        &self,
        connection: &SqlitePooledConnection,
        access_token: &String,
    ) -> Result<Vec<SavPatch>, error::Error> {
        use crate::schema::sav_patches;
        use crate::schema::users;

        let access_token_hash = utils::hmac(&access_token);

        match users::table
            .filter(users::access_token_hash.eq(access_token_hash))
            .inner_join(sav_patches::table)
            .select((
                sav_patches::id,
                sav_patches::date_create,
                sav_patches::data,
                sav_patches::description,
                sav_patches::etag,
            ))
            .get_results::<SavPatch>(connection)
        {
            Ok(sav_patches) => Ok(sav_patches),
            Err(e) => return Err(e.into()),
        }
    }

    /// Insert new row into `sav_patches`
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_sql::*;
    /// # use std::process::Command;
    /// # use std::fs;
    /// # use std::env;
    /// # env::set_var("DATABASE_URL", "test.db");
    /// # Command::new("diesel").args(&["migration", "run"]).output();
    ///
    /// let sql = PkmnapiSQL::new();
    ///
    /// let connection = sql.get_connection().unwrap();
    /// # let (new_user, access_token) = sql.insert_user(&connection, &String::from("foo@bar.com")).unwrap();
    /// let new_sav_patch = sql
    ///     .insert_sav_patch(
    ///         &connection,
    ///         &access_token,
    ///         &vec![0x01, 0x02, 0x03, 0x04],
    ///         None
    ///     )
    ///     .unwrap();
    ///
    /// assert_eq!(new_sav_patch.id.len(), 32);
    /// assert_eq!(new_sav_patch.data, vec![0x01, 0x02, 0x03, 0x04]);
    /// # fs::remove_file("test.db");
    /// ```
    pub fn insert_sav_patch(
        &self,
        connection: &SqlitePooledConnection,
        access_token: &String,
        data: &Vec<u8>,
        description: Option<String>,
    ) -> Result<SavPatch, error::Error> {
        use crate::schema::sav_patches;

        let user = match self.select_user_by_access_token(&connection, &access_token) {
            Ok(Some(user)) => user,
            Ok(None) => return Err(diesel::result::Error::NotFound.into()),
            Err(e) => return Err(e.into()),
        };

        let new_sav_patch = NewSavPatch::new(&user.id, &data, description);

        match diesel::insert_or_ignore_into(sav_patches::table)
            .values(&new_sav_patch)
            .execute(connection)
        {
            Ok(_) => {
                match self.select_sav_patch_by_id(connection, &access_token, &new_sav_patch.id) {
                    Ok(Some(patch)) => Ok(patch),
                    Ok(None) => return Err(diesel::result::Error::NotFound.into()),
                    Err(e) => return Err(e.into()),
                }
            }
            Err(e) => return Err(e.into()),
        }
    }

    /// Delete patch from `sav_patches` by ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_sql::*;
    /// # use std::process::Command;
    /// # use std::fs;
    /// # use std::env;
    /// # env::set_var("DATABASE_URL", "test.db");
    /// # Command::new("diesel").args(&["migration", "run"]).output();
    ///
    /// let sql = PkmnapiSQL::new();
    ///
    /// let connection = sql.get_connection().unwrap();
    /// # let (new_user, access_token) = sql.insert_user(&connection, &String::from("foo@bar.com")).unwrap();
    /// # let new_sav_patch = sql.insert_sav_patch(&connection, &access_token, &vec![0x01, 0x02, 0x03, 0x04], None).unwrap();
    /// # let id = new_sav_patch.id;
    /// # let etag = new_sav_patch.etag;
    /// sql.delete_sav_patch_by_id(&connection, &access_token, &id, &etag).unwrap();
    /// # fs::remove_file("test.db");
    /// ```
    pub fn delete_sav_patch_by_id(
        &self,
        connection: &SqlitePooledConnection,
        access_token: &String,
        id: &String,
        etag: &String,
    ) -> Result<(), error::Error> {
        use crate::schema::sav_patches;

        connection.transaction::<_, error::Error, _>(|| {
            let user = match self.select_user_by_access_token(connection, &access_token) {
                Ok(Some(user)) => user,
                Ok(None) => return Err(diesel::result::Error::NotFound.into()),
                Err(e) => return Err(e.into()),
            };

            let patch = match self.select_sav_patch_by_id(connection, &access_token, id) {
                Ok(Some(patch)) => patch,
                Ok(None) => return Err(diesel::result::Error::NotFound.into()),
                Err(e) => return Err(e.into()),
            };

            if patch.etag != etag.to_owned() {
                return Err(error::Error::ETagError);
            }

            match diesel::delete(
                sav_patches::table
                    .filter(sav_patches::user_id.eq(user.id))
                    .filter(sav_patches::id.eq(id)),
            )
            .execute(connection)
            {
                Ok(_) => Ok(()),
                Err(e) => return Err(e.into()),
            }
        })
    }

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
    /// # use std::fs;
    /// # use std::env;
    /// # env::set_var("DATABASE_URL", "test.db");
    /// # Command::new("diesel").args(&["migration", "run"]).output();
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
    /// # fs::remove_file("test.db");
    /// ```
    pub fn select_user_sav_by_access_token(
        &self,
        connection: &SqlitePooledConnection,
        access_token: &String,
    ) -> Result<Option<Sav>, error::Error> {
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
    /// # use std::fs;
    /// # use std::env;
    /// # env::set_var("DATABASE_URL", "test.db");
    /// # Command::new("diesel").args(&["migration", "run"]).output();
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
    /// # fs::remove_file("test.db");
    /// ```
    pub fn update_user_sav_by_access_token(
        &self,
        connection: &SqlitePooledConnection,
        access_token: &String,
        data: &Vec<u8>,
    ) -> Result<Sav, error::Error> {
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
    /// # use std::fs;
    /// # use std::env;
    /// use pkmnapi_sql::*;
    /// # env::set_var("DATABASE_URL", "test.db");
    /// # Command::new("diesel").args(&["migration", "run"]).output();
    ///
    /// let sql = PkmnapiSQL::new();
    ///
    /// let connection = sql.get_connection().unwrap();
    /// # let (new_user, access_token) = sql.insert_user(&connection, &String::from("foo@bar.com")).unwrap();
    /// # let sav = sql.update_user_sav_by_access_token(&connection, &access_token, &vec![0x01, 0x02, 0x03, 0x04]).unwrap();
    /// # let etag = sav.etag;
    /// sql.delete_user_sav_by_access_token(&connection, &access_token, &etag);
    /// # fs::remove_file("test.db");
    /// ```
    pub fn delete_user_sav_by_access_token(
        &self,
        connection: &SqlitePooledConnection,
        access_token: &String,
        etag: &String,
    ) -> Result<(), error::Error> {
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
