//! Models module

use crate::schema::{patches, rom_data, roms, users};
use crate::utils;
use chrono::{prelude::*, Duration};

/// Queryable struct of data from `rom_data`
#[derive(Debug, PartialEq, Queryable)]
pub struct RomData {
    pub id: String,
    pub name: String,
    pub data: Vec<u8>,
}

/// Insertable struct of data into `rom_data`
#[derive(Debug, Insertable, PartialEq)]
#[table_name = "rom_data"]
pub struct NewRomData {
    pub id: String,
    pub name: String,
    pub data: Vec<u8>,
}

impl NewRomData {
    /// Create new rom_data entry
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_sql::models::*;
    ///
    /// let new_rom_data = NewRomData::new(&String::from("foo"), &vec![0x01, 0x02, 0x03, 0x04]);
    ///
    /// assert_eq!(
    ///     new_rom_data.id,
    ///     String::from("08d6c05a21512a79a1dfeb9d2a8f262f")
    /// );
    /// assert_eq!(new_rom_data.name, String::from("foo"));
    /// assert_eq!(new_rom_data.data, vec![0x01, 0x02, 0x03, 0x04]);
    /// ```
    pub fn new(name: &String, data: &Vec<u8>) -> Self {
        let id = format!("{:02x}", md5::compute(&data));

        NewRomData {
            id,
            name: name.to_string(),
            data: data.to_vec(),
        }
    }
}

/// Queryable struct of data from `roms`
#[derive(Debug, Queryable)]
pub struct Rom {
    pub id: String,
    pub name: String,
    pub rom_data_id: String,
}

/// Insertable struct of data into `roms`
#[derive(Debug, Insertable, PartialEq)]
#[table_name = "roms"]
pub struct NewRom {
    pub id: String,
    pub name: String,
    pub rom_data_id: String,
}

impl NewRom {
    /// Create new roms entry
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_sql::models::*;
    ///
    /// let new_rom = NewRom::new(&String::from("foo"), &String::from("bar"));
    ///
    /// assert_eq!(new_rom.id.len(), 32);
    /// assert_eq!(new_rom.name, String::from("foo"));
    /// assert_eq!(new_rom.rom_data_id, String::from("bar"));
    /// ```
    pub fn new(name: &String, rom_data_id: &String) -> Self {
        let id = utils::random_id(32);

        NewRom {
            id,
            name: name.to_string(),
            rom_data_id: rom_data_id.to_string(),
        }
    }
}

/// Queryable struct of data from `users`
#[derive(Debug, Queryable)]
pub struct User {
    pub id: String,
    pub date_create: String,
    pub date_expire: String,
    pub access_token_hash: String,
    pub rom_id: Option<String>,
}

/// Insertable struct of data into `users`
#[derive(Debug, Insertable, PartialEq)]
#[table_name = "users"]
pub struct NewUser {
    pub id: String,
    pub date_create: String,
    pub date_expire: String,
    pub access_token_hash: String,
    pub rom_id: Option<String>,
}

impl NewUser {
    /// Create new roms entry
    ///
    /// # Panics
    ///
    /// Panics if the `SECRET_KEY` environment variable is not set
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_sql::models::*;
    /// # use std::env;
    /// # env::set_var("SECRET_KEY", "1337");
    ///
    /// let (new_user, access_token) = NewUser::new(&String::from("foo@bar.com"));
    ///
    /// assert_eq!(new_user.id, String::from("foo@bar.com"));
    /// assert_eq!(new_user.date_create.len(), 32);
    /// assert_eq!(new_user.date_expire.len(), 32);
    /// assert_eq!(new_user.access_token_hash.len(), 64);
    /// assert_eq!(new_user.rom_id, None);
    /// assert_eq!(access_token.len(), 64);
    /// ```
    pub fn new(id: &String) -> (Self, String) {
        let date_create = Utc::now().to_rfc3339();
        let date_expire = (Utc::now() + Duration::days(1)).to_rfc3339();
        let access_token = utils::random_id(64);
        let rom_id = None;

        let access_token_hash = utils::hmac(&access_token);

        let new_user = NewUser {
            id: id.to_string(),
            date_create,
            date_expire,
            access_token_hash,
            rom_id,
        };

        (new_user, access_token)
    }
}

/// Queryable struct of data from `patches`
#[derive(Debug, Queryable, PartialEq)]
pub struct Patch {
    pub id: String,
    pub data: Vec<u8>,
    pub description: Option<String>
}

/// Insertable struct of data into `patches`
#[derive(Debug, Insertable, PartialEq)]
#[table_name = "patches"]
pub struct NewPatch {
    pub id: String,
    pub user_id: String,
    pub data: Vec<u8>,
    pub description: Option<String>
}

impl NewPatch {
    /// Create new patch entry
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_sql::models::*;
    ///
    /// let new_patch = NewPatch::new(&String::from("foo"), &vec![0x01, 0x02, 0x03, 0x04]);
    ///
    /// assert_eq!(new_patch.id.len(), 32);
    /// assert_eq!(new_patch.user_id, String::from("foo"));
    /// assert_eq!(new_patch.data, vec![0x01, 0x02, 0x03, 0x04]);
    /// assert_eq!(new_patch.description, None);
    /// ```
    pub fn new(user_id: &String, data: &Vec<u8>, description: Option<String>) -> Self {
        let id = utils::random_id(32);

        NewPatch {
            id,
            user_id: user_id.to_string(),
            data: data.to_vec(),
            description
        }
    }
}
