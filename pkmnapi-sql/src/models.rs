//! Models module

use chrono::{prelude::*, Duration};

use crate::schema::{rom_data, rom_patches, roms, sav_patches, savs, users};
use crate::utils;

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
        let id = utils::hash(&data);

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
    pub date_create: String,
    pub name: String,
    pub etag: String,
    pub rom_data_id: String,
}

/// Insertable struct of data into `roms`
#[derive(Debug, Insertable, PartialEq)]
#[table_name = "roms"]
pub struct NewRom {
    pub id: String,
    pub date_create: String,
    pub name: String,
    pub etag: String,
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
    /// assert_eq!(new_rom.date_create.len(), 20);
    /// assert_eq!(new_rom.name, String::from("foo"));
    /// assert_eq!(new_rom.etag.len(), 36);
    /// assert_eq!(new_rom.rom_data_id, String::from("bar"));
    /// ```
    pub fn new(name: &String, rom_data_id: &String) -> Self {
        let id = utils::random_id(32);
        let date_create = Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true);
        let etag = utils::etag(
            &[
                id.as_bytes(),
                date_create.as_bytes(),
                name.as_bytes(),
                rom_data_id.as_bytes(),
            ]
            .concat(),
        );

        NewRom {
            id,
            date_create,
            name: name.to_string(),
            etag,
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
    pub delete_code: Option<String>,
    pub rom_id: Option<String>,
    pub sav_id: Option<String>,
}

impl User {
    pub fn seconds_to_expiration(&self) -> i64 {
        match DateTime::parse_from_rfc3339(&self.date_expire) {
            Ok(date_expire) => date_expire.timestamp() - Utc::now().timestamp(),
            _ => 0,
        }
    }
}

/// Insertable struct of data into `users`
#[derive(Debug, Insertable, PartialEq)]
#[table_name = "users"]
pub struct NewUser {
    pub id: String,
    pub date_create: String,
    pub date_expire: String,
    pub access_token_hash: String,
    pub delete_code: Option<String>,
    pub rom_id: Option<String>,
    pub sav_id: Option<String>,
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
    /// assert_eq!(new_user.date_create.len(), 20);
    /// assert_eq!(new_user.date_expire.len(), 20);
    /// assert_eq!(new_user.access_token_hash.len(), 64);
    /// assert_eq!(new_user.delete_code, None);
    /// assert_eq!(new_user.rom_id, None);
    /// assert_eq!(new_user.sav_id, None);
    /// assert_eq!(access_token.len(), 64);
    /// ```
    pub fn new(id: &String) -> (Self, String) {
        let date_create = Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true);
        let date_expire =
            (Utc::now() + Duration::seconds(600)).to_rfc3339_opts(SecondsFormat::Secs, true);
        let access_token = utils::random_id(64);
        let delete_code = None;
        let rom_id = None;
        let sav_id = None;

        let access_token_hash = utils::hmac(&access_token);

        let new_user = NewUser {
            id: id.to_string(),
            date_create,
            date_expire,
            access_token_hash,
            delete_code,
            rom_id,
            sav_id,
        };

        (new_user, access_token)
    }
}

/// Queryable struct of data from `rom_patches`
#[derive(Debug, Queryable, PartialEq)]
pub struct RomPatch {
    pub id: String,
    pub date_create: String,
    pub data: Vec<u8>,
    pub description: Option<String>,
    pub etag: String,
}

/// Insertable struct of data into `rom_patches`
#[derive(Debug, Insertable, PartialEq)]
#[table_name = "rom_patches"]
pub struct NewRomPatch {
    pub id: String,
    pub date_create: String,
    pub user_id: String,
    pub data: Vec<u8>,
    pub description: Option<String>,
    pub etag: String,
}

impl NewRomPatch {
    /// Create new patch entry
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_sql::models::*;
    ///
    /// let new_patch = NewRomPatch::new(&String::from("foo"), &vec![0x01, 0x02, 0x03, 0x04], None);
    ///
    /// assert_eq!(new_patch.id.len(), 32);
    /// assert_eq!(new_patch.date_create.len(), 20);
    /// assert_eq!(new_patch.user_id, String::from("foo"));
    /// assert_eq!(new_patch.data, vec![0x01, 0x02, 0x03, 0x04]);
    /// assert_eq!(new_patch.description, None);
    /// assert_eq!(new_patch.etag.len(), 36);
    /// ```
    pub fn new(user_id: &String, data: &Vec<u8>, description: Option<String>) -> Self {
        let id = utils::random_id(32);
        let date_create = Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true);
        let etag = utils::etag(
            &[
                id.as_bytes(),
                date_create.as_bytes(),
                user_id.as_bytes(),
                data,
                description
                    .to_owned()
                    .unwrap_or(String::from(""))
                    .as_bytes(),
            ]
            .concat(),
        );

        NewRomPatch {
            id,
            date_create,
            user_id: user_id.to_string(),
            data: data.to_vec(),
            description,
            etag,
        }
    }
}

/// Queryable struct of data from `savs`
#[derive(Debug, Queryable)]
pub struct Sav {
    pub id: String,
    pub date_create: String,
    pub data: Vec<u8>,
    pub etag: String,
}

/// Insertable struct of data into `savs`
#[derive(Debug, Insertable, PartialEq)]
#[table_name = "savs"]
pub struct NewSav {
    pub id: String,
    pub date_create: String,
    pub data: Vec<u8>,
    pub etag: String,
}

impl NewSav {
    /// Create new savs entry
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_sql::models::*;
    ///
    /// let new_sav = NewSav::new(&vec![0x01, 0x02, 0x03, 0x04]);
    ///
    /// assert_eq!(new_sav.id.len(), 32);
    /// assert_eq!(new_sav.date_create.len(), 20);
    /// assert_eq!(new_sav.data, vec![0x01, 0x02, 0x03, 0x04]);
    /// assert_eq!(new_sav.etag.len(), 36);
    /// ```
    pub fn new(data: &Vec<u8>) -> Self {
        let id = utils::random_id(32);
        let date_create = Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true);
        let etag = utils::etag(&[id.as_bytes(), date_create.as_bytes(), data].concat());

        NewSav {
            id,
            date_create,
            data: data.to_vec(),
            etag,
        }
    }
}

/// Queryable struct of data from `sav_patches`
#[derive(Debug, Queryable, PartialEq)]
pub struct SavPatch {
    pub id: String,
    pub date_create: String,
    pub data: Vec<u8>,
    pub description: Option<String>,
    pub etag: String,
}

/// Insertable struct of data into `sav_patches`
#[derive(Debug, Insertable, PartialEq)]
#[table_name = "sav_patches"]
pub struct NewSavPatch {
    pub id: String,
    pub date_create: String,
    pub user_id: String,
    pub data: Vec<u8>,
    pub description: Option<String>,
    pub etag: String,
}

impl NewSavPatch {
    /// Create new patch entry
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_sql::models::*;
    ///
    /// let new_patch = NewSavPatch::new(&String::from("foo"), &vec![0x01, 0x02, 0x03, 0x04], None);
    ///
    /// assert_eq!(new_patch.id.len(), 32);
    /// assert_eq!(new_patch.date_create.len(), 20);
    /// assert_eq!(new_patch.user_id, String::from("foo"));
    /// assert_eq!(new_patch.data, vec![0x01, 0x02, 0x03, 0x04]);
    /// assert_eq!(new_patch.description, None);
    /// assert_eq!(new_patch.etag.len(), 36);
    /// ```
    pub fn new(user_id: &String, data: &Vec<u8>, description: Option<String>) -> Self {
        let id = utils::random_id(32);
        let date_create = Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true);
        let etag = utils::etag(
            &[
                id.as_bytes(),
                date_create.as_bytes(),
                user_id.as_bytes(),
                data,
                description
                    .to_owned()
                    .unwrap_or(String::from(""))
                    .as_bytes(),
            ]
            .concat(),
        );

        NewSavPatch {
            id,
            date_create,
            user_id: user_id.to_string(),
            data: data.to_vec(),
            description,
            etag,
        }
    }
}
