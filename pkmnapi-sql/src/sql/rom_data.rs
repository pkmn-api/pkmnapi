use diesel::prelude::*;

use crate::error::Result;
use crate::models::*;
use crate::{PgPooledConnection, PkmnapiSQL};

impl PkmnapiSQL {
    /// Select all rows from `rom_data` by ID
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
    /// # let new_rom_data = sql.insert_rom_data(&connection, &String::from("foo"), &vec![0x01, 0x02, 0x03, 0x04]).unwrap();
    /// # let id = new_rom_data.id;
    /// let rom_datum = sql.select_rom_data_by_id(&connection, &id).unwrap().unwrap();
    ///
    /// assert_eq!(rom_datum.id.len(), 32);
    /// assert_eq!(rom_datum.name, String::from("foo"));
    /// assert_eq!(rom_datum.data, vec![0x01, 0x02, 0x03, 0x04]);
    /// # sql.revert_migration();
    /// ```
    pub fn select_rom_data_by_id(
        &self,
        connection: &PgPooledConnection,
        id: &String,
    ) -> Result<Option<RomData>> {
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
    /// # sql.revert_migration();
    /// ```
    pub fn insert_rom_data(
        &self,
        connection: &PgPooledConnection,
        name: &String,
        data: &Vec<u8>,
    ) -> Result<RomData> {
        use crate::schema::rom_data;

        let new_rom_data = NewRomData::new(&name, &data);

        match diesel::insert_into(rom_data::table)
            .values(&new_rom_data)
            .on_conflict_do_nothing()
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
}
