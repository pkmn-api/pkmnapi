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

    pub fn get_connection(&self) -> Result<SqlitePooledConnection, diesel::result::Error> {
        match self.connection.get() {
            Ok(connection) => Ok(connection),
            Err(_) => Err(diesel::result::Error::NotFound),
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
    ) -> Result<Option<RomData>, diesel::result::Error> {
        use crate::schema::rom_data;

        match rom_data::table
            .filter(rom_data::id.eq(id))
            .first::<RomData>(connection)
        {
            Ok(rom_data) => Ok(Some(rom_data)),
            Err(diesel::result::Error::NotFound) => Ok(None),
            Err(e) => Err(e),
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
    ) -> Result<RomData, diesel::result::Error> {
        use crate::schema::rom_data;

        let new_rom_data = NewRomData::new(&name, &data);

        match diesel::insert_or_ignore_into(rom_data::table)
            .values(&new_rom_data)
            .execute(connection)
        {
            Ok(_) => match self.select_rom_data_by_id(connection, &new_rom_data.id) {
                Ok(Some(rom_data)) => Ok(rom_data),
                Ok(None) => Err(diesel::result::Error::NotFound),
                Err(e) => return Err(e),
            },
            Err(e) => return Err(e),
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
    /// assert_eq!(rom.name, String::from("foo"));
    /// assert_eq!(rom.rom_data_id.len(), 32);
    /// # fs::remove_file("test.db");
    /// ```
    pub fn select_rom_by_id(
        &self,
        connection: &SqlitePooledConnection,
        id: &String,
    ) -> Result<Option<Rom>, diesel::result::Error> {
        use crate::schema::rom_data;
        use crate::schema::roms;

        match roms::table
            .filter(roms::id.eq(id))
            .inner_join(rom_data::table)
            .select((roms::id, roms::name, rom_data::id))
            .first::<Rom>(connection)
        {
            Ok(rom) => Ok(Some(rom)),
            Err(diesel::result::Error::NotFound) => Ok(None),
            Err(e) => Err(e),
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
    /// assert_eq!(new_rom.rom_data_id.len(), 32);
    /// # fs::remove_file("test.db");
    /// ```
    pub fn insert_rom(
        &self,
        connection: &SqlitePooledConnection,
        name: &String,
        data: &Vec<u8>,
    ) -> Result<Rom, diesel::result::Error> {
        use crate::schema::roms;

        match connection.transaction::<_, diesel::result::Error, _>(|| {
            let new_rom_data = self.insert_rom_data(connection, &name, &data)?;
            let new_rom = NewRom::new(&name, &new_rom_data.id);

            match diesel::insert_into(roms::table)
                .values(&new_rom)
                .execute(connection)
            {
                Ok(_) => Ok(new_rom.id),
                Err(e) => return Err(e),
            }
        }) {
            Ok(new_rom_id) => match self.select_rom_by_id(connection, &new_rom_id) {
                Ok(Some(rom)) => Ok(rom),
                Ok(None) => Err(diesel::result::Error::NotFound),
                Err(e) => return Err(e),
            },
            Err(e) => return Err(e),
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
    ) -> Result<(), diesel::result::Error> {
        use crate::schema::roms;

        match diesel::delete(roms::table.filter(roms::id.eq(id))).execute(connection) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
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
    /// assert_eq!(user.date_create.len(), 32);
    /// assert_eq!(user.date_expire.len(), 32);
    /// assert_eq!(user.access_token_hash.len(), 64);
    /// assert_eq!(user.rom_id, None);
    /// # fs::remove_file("test.db");
    /// ```
    pub fn select_user_by_access_token(
        &self,
        connection: &SqlitePooledConnection,
        access_token: &String,
    ) -> Result<Option<User>, diesel::result::Error> {
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
            ))
            .first::<User>(connection)
        {
            Ok(user) => Ok(Some(user)),
            Err(diesel::result::Error::NotFound) => Ok(None),
            Err(e) => Err(e),
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
    /// assert_eq!(new_user.date_create.len(), 32);
    /// assert_eq!(new_user.date_expire.len(), 32);
    /// assert_eq!(new_user.access_token_hash.len(), 64);
    /// assert_eq!(new_user.rom_id, None);
    /// assert_eq!(access_token.len(), 64);
    /// # fs::remove_file("test.db");
    /// ```
    pub fn insert_user(
        &self,
        connection: &SqlitePooledConnection,
        id: &String,
    ) -> Result<(User, String), diesel::result::Error> {
        use crate::schema::users;

        let (new_user, access_token) = NewUser::new(&id);

        match diesel::insert_into(users::table)
            .values(&new_user)
            .execute(connection)
        {
            Ok(_) => match self.select_user_by_access_token(connection, &access_token) {
                Ok(Some(new_user)) => Ok((new_user, access_token)),
                Ok(None) => return Err(diesel::result::Error::NotFound),
                Err(e) => return Err(e),
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
                    Ok(None) => return Err(diesel::result::Error::NotFound),
                    Err(e) => return Err(e),
                }
            }
            Err(e) => return Err(e),
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
    /// assert_eq!(rom.name, String::from("foo"));
    /// assert_eq!(rom.rom_data_id.len(), 32);
    /// # fs::remove_file("test.db");
    /// ```
    pub fn select_user_rom_by_access_token(
        &self,
        connection: &SqlitePooledConnection,
        access_token: &String,
    ) -> Result<Option<Rom>, diesel::result::Error> {
        use crate::schema::roms;
        use crate::schema::users;

        let access_token_hash = utils::hmac(&access_token);

        match users::table
            .filter(users::access_token_hash.eq(access_token_hash))
            .inner_join(roms::table)
            .select((roms::id, roms::name, roms::rom_data_id))
            .first::<Rom>(connection)
        {
            Ok(rom) => Ok(Some(rom)),
            Err(diesel::result::Error::NotFound) => Ok(None),
            Err(e) => Err(e),
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
    /// assert_eq!(rom.rom_data_id.len(), 32);
    /// # fs::remove_file("test.db");
    /// ```
    pub fn update_user_rom_by_access_token(
        &self,
        connection: &SqlitePooledConnection,
        access_token: &String,
        name: &String,
        data: &Vec<u8>,
    ) -> Result<Rom, diesel::result::Error> {
        use crate::schema::users;

        connection.transaction::<_, diesel::result::Error, _>(|| {
            let new_rom = self.insert_rom(connection, &name, &data)?;
            let user = match self.select_user_by_access_token(connection, &access_token) {
                Ok(Some(user)) => user,
                Ok(None) => return Err(diesel::result::Error::NotFound),
                Err(e) => return Err(e),
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
                Ok(None) => Err(diesel::result::Error::NotFound),
                Err(e) => return Err(e),
            }
        })
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
    /// # sql.update_user_rom_by_access_token(&connection, &access_token, &String::from("foo"), &vec![0x01, 0x02, 0x03, 0x04]).unwrap();
    /// sql.delete_user_rom_by_access_token(&connection, &access_token);
    /// # fs::remove_file("test.db");
    /// ```
    pub fn delete_user_rom_by_access_token(
        &self,
        connection: &SqlitePooledConnection,
        access_token: &String,
    ) -> Result<(), diesel::result::Error> {
        use crate::schema::users;

        connection.transaction::<_, diesel::result::Error, _>(|| {
            let rom = match self.select_user_rom_by_access_token(connection, &access_token) {
                Ok(Some(rom)) => rom,
                Ok(None) => return Err(diesel::result::Error::NotFound),
                Err(e) => return Err(e),
            };

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
    ) -> Result<Option<RomData>, diesel::result::Error> {
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
            Err(e) => Err(e),
        }
    }

    /// Select row in `patches` by ID
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
    /// # let new_patch = sql.insert_patch(&connection, &access_token, &vec![0x01, 0x02, 0x03, 0x04], None).unwrap();
    /// # let id = new_patch.id;
    /// let patch = sql.select_patch_by_id(&connection, &access_token, &id).unwrap().unwrap();
    ///
    /// assert_eq!(patch.id.len(), 32);
    /// assert_eq!(patch.data, vec![0x01, 0x02, 0x03, 0x04]);
    /// # fs::remove_file("test.db");
    /// ```
    pub fn select_patch_by_id(
        &self,
        connection: &SqlitePooledConnection,
        access_token: &String,
        id: &String,
    ) -> Result<Option<Patch>, diesel::result::Error> {
        use crate::schema::patches;
        use crate::schema::users;

        let access_token_hash = utils::hmac(&access_token);

        match users::table
            .filter(users::access_token_hash.eq(access_token_hash))
            .inner_join(patches::table)
            .filter(patches::id.eq(id))
            .select((patches::id, patches::data, patches::description))
            .first::<Patch>(connection)
        {
            Ok(patch) => Ok(Some(patch)),
            Err(diesel::result::Error::NotFound) => Ok(None),
            Err(e) => Err(e),
        }
    }

    /// Select rows in `patches` by access token
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
    /// # sql.insert_patch(&connection, &access_token, &vec![0x01, 0x02, 0x03, 0x04], None).unwrap();
    /// let patches = sql.select_patches_by_access_token(&connection, &access_token).unwrap();
    /// let patch = &patches[0];
    ///
    /// assert_eq!(patch.id.len(), 32);
    /// assert_eq!(patch.data, vec![0x01, 0x02, 0x03, 0x04]);
    /// # fs::remove_file("test.db");
    /// ```
    pub fn select_patches_by_access_token(
        &self,
        connection: &SqlitePooledConnection,
        access_token: &String,
    ) -> Result<Vec<Patch>, diesel::result::Error> {
        use crate::schema::patches;
        use crate::schema::users;

        let access_token_hash = utils::hmac(&access_token);

        users::table
            .filter(users::access_token_hash.eq(access_token_hash))
            .inner_join(patches::table)
            .select((patches::id, patches::data, patches::description))
            .get_results::<Patch>(connection)
    }

    /// Insert new row into `patches`
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
    /// let new_patch = sql
    ///     .insert_patch(
    ///         &connection,
    ///         &access_token,
    ///         &vec![0x01, 0x02, 0x03, 0x04],
    ///         None
    ///     )
    ///     .unwrap();
    ///
    /// assert_eq!(new_patch.id.len(), 32);
    /// assert_eq!(new_patch.data, vec![0x01, 0x02, 0x03, 0x04]);
    /// # fs::remove_file("test.db");
    /// ```
    pub fn insert_patch(
        &self,
        connection: &SqlitePooledConnection,
        access_token: &String,
        data: &Vec<u8>,
        description: Option<String>,
    ) -> Result<Patch, diesel::result::Error> {
        use crate::schema::patches;

        let user = match self.select_user_by_access_token(&connection, &access_token) {
            Ok(Some(user)) => user,
            Ok(None) => return Err(diesel::result::Error::NotFound),
            Err(e) => return Err(e),
        };

        let new_patch = NewPatch::new(&user.id, &data, description);

        match diesel::insert_or_ignore_into(patches::table)
            .values(&new_patch)
            .execute(connection)
        {
            Ok(_) => match self.select_patch_by_id(connection, &access_token, &new_patch.id) {
                Ok(Some(patch)) => Ok(patch),
                Ok(None) => Err(diesel::result::Error::NotFound),
                Err(e) => return Err(e),
            },
            Err(e) => return Err(e),
        }
    }

    /// Delete patch from `patches` by ID
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
    /// # let new_patch = sql.insert_patch(&connection, &access_token, &vec![0x01, 0x02, 0x03, 0x04], None).unwrap();
    /// # let id = new_patch.id;
    /// sql.delete_patch_by_id(&connection, &access_token, &id).unwrap();
    /// # fs::remove_file("test.db");
    /// ```
    pub fn delete_patch_by_id(
        &self,
        connection: &SqlitePooledConnection,
        access_token: &String,
        id: &String,
    ) -> Result<(), diesel::result::Error> {
        use crate::schema::patches;

        connection.transaction::<_, diesel::result::Error, _>(|| {
            let user = match self.select_user_by_access_token(connection, &access_token) {
                Ok(Some(user)) => user,
                Ok(None) => return Err(diesel::result::Error::NotFound),
                Err(e) => return Err(e),
            };

            match diesel::delete(
                patches::table
                    .filter(patches::user_id.eq(user.id))
                    .filter(patches::id.eq(id)),
            )
            .execute(connection)
            {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            }
        })
    }
}
