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
use diesel::result::{DatabaseErrorKind, Error::DatabaseError};
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

use crate::models::*;

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
    connection: SqliteConnection,
}

impl PkmnapiSQL {
    /// Create new PkmnapiSQL
    ///
    /// # Panics
    ///
    /// Panics if the `DATABASE_URL` environment variable is not set
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
        let connection = SqliteConnection::establish(&database_url).unwrap();

        PkmnapiSQL { connection }
    }

    /// Select all rows from `rom_data` by ID
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
    /// # let new_rom_data = sql.insert_rom_data(&String::from("foo"), &vec![0x01, 0x02, 0x03, 0x04]).unwrap();
    /// # let id = new_rom_data.id;
    /// let rom_datum = sql.select_rom_data_by_id(&id).unwrap();
    ///
    /// assert_eq!(rom_datum.id.len(), 32);
    /// assert_eq!(rom_datum.name, String::from("foo"));
    /// assert_eq!(rom_datum.data, vec![0x01, 0x02, 0x03, 0x04]);
    /// # fs::remove_file("test.db");
    /// ```
    pub fn select_rom_data_by_id(&self, id: &String) -> Result<RomData, diesel::result::Error> {
        use crate::schema::rom_data;

        rom_data::table
            .filter(rom_data::id.eq(id))
            .first::<RomData>(&self.connection)
    }

    /// Insert new row into `rom_data`
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
    /// let new_rom_data = sql
    ///     .insert_rom_data(&String::from("foo"), &vec![0x01, 0x02, 0x03, 0x04])
    ///     .unwrap();
    ///
    /// assert_eq!(new_rom_data.id.len(), 32);
    /// assert_eq!(new_rom_data.name, String::from("foo"));
    /// assert_eq!(new_rom_data.data, vec![0x01, 0x02, 0x03, 0x04]);
    /// # fs::remove_file("test.db");
    /// ```
    pub fn insert_rom_data(
        &self,
        name: &String,
        data: &Vec<u8>,
    ) -> Result<RomData, diesel::result::Error> {
        use crate::schema::rom_data;

        let new_rom_data = NewRomData::new(&name, &data);

        match diesel::insert_or_ignore_into(rom_data::table)
            .values(&new_rom_data)
            .execute(&self.connection)
        {
            Ok(_) => self.select_rom_data_by_id(&new_rom_data.id),
            Err(e) => return Err(e),
        }
    }

    /// Select row from `roms` by ID
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
    /// # let new_rom = sql.insert_rom(&String::from("foo"), &vec![0x01, 0x02, 0x03, 0x04]).unwrap();
    /// # let id = new_rom.id;
    /// let rom = sql.select_rom_by_id(&id).unwrap();
    ///
    /// assert_eq!(rom.id.len(), 32);
    /// assert_eq!(rom.name, String::from("foo"));
    /// assert_eq!(rom.rom_data_id.len(), 32);
    /// # fs::remove_file("test.db");
    /// ```
    pub fn select_rom_by_id(&self, id: &String) -> Result<Rom, diesel::result::Error> {
        use crate::schema::rom_data;
        use crate::schema::roms;

        roms::table
            .filter(roms::id.eq(id))
            .inner_join(rom_data::table)
            .select((roms::id, roms::name, rom_data::id))
            .first::<Rom>(&self.connection)
    }

    /// Insert rows into `roms`
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
    /// let new_rom = sql
    ///     .insert_rom(&String::from("foo"), &vec![0x01, 0x02, 0x03, 0x04])
    ///     .unwrap();
    ///
    /// assert_eq!(new_rom.id.len(), 32);
    /// assert_eq!(new_rom.name, String::from("foo"));
    /// assert_eq!(new_rom.rom_data_id.len(), 32);
    /// # fs::remove_file("test.db");
    /// ```
    pub fn insert_rom(&self, name: &String, data: &Vec<u8>) -> Result<Rom, diesel::result::Error> {
        use crate::schema::roms;

        self.connection
            .transaction::<_, diesel::result::Error, _>(|| {
                let new_rom_data = self.insert_rom_data(&name, &data)?;
                let new_rom = NewRom::new(&name, &new_rom_data.id);

                match diesel::insert_into(roms::table)
                    .values(&new_rom)
                    .execute(&self.connection)
                {
                    Ok(_) => self.select_rom_by_id(&new_rom.id),
                    Err(e) => return Err(e),
                }
            })
    }

    /// Delete rom from `roms` by ID
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
    /// # let new_rom = sql.insert_rom(&String::from("foo"), &vec![0x01, 0x02, 0x03, 0x04]).unwrap();
    /// # let id = new_rom.id;
    ///
    /// sql.delete_rom_by_id(&id).unwrap();
    /// # fs::remove_file("test.db");
    /// ```
    pub fn delete_rom_by_id(&self, id: &String) -> Result<(), diesel::result::Error> {
        use crate::schema::roms;

        self.connection
            .transaction::<_, diesel::result::Error, _>(|| {
                match diesel::delete(roms::table.filter(roms::id.eq(id))).execute(&self.connection)
                {
                    Ok(_) => Ok(()),
                    Err(e) => Err(e),
                }
            })
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
    /// # use std::process::Command;
    /// # use std::fs;
    /// # use std::env;
    /// use pkmnapi_sql::*;
    /// # env::set_var("DATABASE_URL", "test.db");
    /// # Command::new("diesel").args(&["migration", "run"]).output();
    ///
    /// let sql = PkmnapiSQL::new();
    ///
    /// # let (new_user, access_token) = sql.insert_user(&String::from("foo@bar.com")).unwrap();
    /// let user = sql.select_user_by_access_token(&access_token).unwrap();
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
        access_token: &String,
    ) -> Result<User, diesel::result::Error> {
        use crate::schema::users;

        let access_token_hash = utils::hmac(&access_token);

        users::table
            .filter(users::access_token_hash.eq(access_token_hash))
            .select((
                users::id,
                users::date_create,
                users::date_expire,
                users::access_token_hash,
                users::rom_id,
            ))
            .first::<User>(&self.connection)
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
    /// # use std::process::Command;
    /// # use std::fs;
    /// # use std::env;
    /// use pkmnapi_sql::*;
    /// # env::set_var("DATABASE_URL", "test.db");
    /// # Command::new("diesel").args(&["migration", "run"]).output();
    ///
    /// let sql = PkmnapiSQL::new();
    ///
    /// let (new_user, access_token) = sql.insert_user(&String::from("foo@bar.com")).unwrap();
    ///
    /// assert_eq!(new_user.id, String::from("foo@bar.com"));
    /// assert_eq!(new_user.date_create.len(), 32);
    /// assert_eq!(new_user.date_expire.len(), 32);
    /// assert_eq!(new_user.access_token_hash.len(), 64);
    /// assert_eq!(new_user.rom_id, None);
    /// assert_eq!(access_token.len(), 64);
    /// # fs::remove_file("test.db");
    /// ```
    pub fn insert_user(&self, id: &String) -> Result<(User, String), diesel::result::Error> {
        use crate::schema::users;

        let (new_user, access_token) = NewUser::new(&id);

        match diesel::insert_into(users::table)
            .values(&new_user)
            .execute(&self.connection)
        {
            Ok(_) => match self.select_user_by_access_token(&access_token) {
                Ok(new_user) => Ok((new_user, access_token)),
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
                    .execute(&self.connection)
                    .ok();

                match self.select_user_by_access_token(&access_token) {
                    Ok(new_user) => Ok((new_user, access_token)),
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
    /// # use std::process::Command;
    /// # use std::fs;
    /// # use std::env;
    /// use pkmnapi_sql::*;
    /// # env::set_var("DATABASE_URL", "test.db");
    /// # Command::new("diesel").args(&["migration", "run"]).output();
    ///
    /// let sql = PkmnapiSQL::new();
    ///
    /// # let (new_user, access_token) = sql.insert_user(&String::from("foo@bar.com")).unwrap();
    /// # sql.update_user_rom_by_access_token(&access_token, &String::from("foo"), &vec![0x01, 0x02, 0x03, 0x04]).unwrap();
    /// let rom = sql.select_user_rom_by_access_token(&access_token).unwrap();
    ///
    /// assert_eq!(rom.id.len(), 32);
    /// assert_eq!(rom.name, String::from("foo"));
    /// assert_eq!(rom.rom_data_id.len(), 32);
    /// # fs::remove_file("test.db");
    /// ```
    pub fn select_user_rom_by_access_token(
        &self,
        access_token: &String,
    ) -> Result<Rom, diesel::result::Error> {
        use crate::schema::roms;
        use crate::schema::users;

        let access_token_hash = utils::hmac(&access_token);

        users::table
            .filter(users::access_token_hash.eq(access_token_hash))
            .inner_join(roms::table)
            .select((roms::id, roms::name, roms::rom_data_id))
            .first::<Rom>(&self.connection)
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
    /// # let (new_user, access_token) = sql.insert_user(&String::from("foo@bar.com")).unwrap();
    /// let rom = sql
    ///     .update_user_rom_by_access_token(
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
        access_token: &String,
        name: &String,
        data: &Vec<u8>,
    ) -> Result<Rom, diesel::result::Error> {
        use crate::schema::users;

        self.connection
            .transaction::<_, diesel::result::Error, _>(|| {
                let new_rom = self.insert_rom(&name, &data)?;
                let user = self.select_user_by_access_token(&access_token)?;

                let rom_id = match user.rom_id {
                    Some(_) => return Err(diesel::result::Error::RollbackTransaction),
                    None => new_rom.id,
                };

                let access_token_hash = utils::hmac(&access_token);

                diesel::update(users::table.filter(users::access_token_hash.eq(access_token_hash)))
                    .set(users::rom_id.eq(&rom_id))
                    .execute(&self.connection)?;

                self.select_rom_by_id(&rom_id)
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
    /// # let (new_user, access_token) = sql.insert_user(&String::from("foo@bar.com")).unwrap();
    /// # sql.update_user_rom_by_access_token(&access_token, &String::from("foo"), &vec![0x01, 0x02, 0x03, 0x04]).unwrap();
    /// sql.delete_user_rom_by_access_token(&access_token);
    /// # fs::remove_file("test.db");
    /// ```
    pub fn delete_user_rom_by_access_token(
        &self,
        access_token: &String,
    ) -> Result<(), diesel::result::Error> {
        use crate::schema::users;

        self.connection
            .transaction::<_, diesel::result::Error, _>(|| {
                let rom = self.select_user_rom_by_access_token(&access_token)?;
                let access_token_hash = utils::hmac(&access_token);
                let rom_id: Option<String> = None;

                diesel::update(users::table.filter(users::access_token_hash.eq(access_token_hash)))
                    .set(users::rom_id.eq(&rom_id))
                    .execute(&self.connection)?;

                self.delete_rom_by_id(&rom.id)
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
    /// # let (new_user, access_token) = sql.insert_user(&String::from("foo@bar.com")).unwrap();
    /// # sql.update_user_rom_by_access_token(&access_token, &String::from("foo"), &vec![0x01, 0x02, 0x03, 0x04]).unwrap();
    /// let rom_datum = sql.select_user_rom_data_by_access_token(&access_token).unwrap();
    ///
    /// assert_eq!(rom_datum.id.len(), 32);
    /// assert_eq!(rom_datum.name, String::from("foo"));
    /// assert_eq!(rom_datum.data, vec![0x01, 0x02, 0x03, 0x04]);
    /// # fs::remove_file("test.db");
    /// ```
    pub fn select_user_rom_data_by_access_token(
        &self,
        access_token: &String,
    ) -> Result<RomData, diesel::result::Error> {
        use crate::schema::rom_data;
        use crate::schema::roms;
        use crate::schema::users;

        let access_token_hash = utils::hmac(&access_token);

        users::table
            .filter(users::access_token_hash.eq(access_token_hash))
            .inner_join(roms::table.inner_join(rom_data::table))
            .select((rom_data::id, rom_data::name, rom_data::data))
            .first::<RomData>(&self.connection)
    }
}
