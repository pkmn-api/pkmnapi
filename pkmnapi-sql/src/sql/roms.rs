use diesel::prelude::*;

use crate::error::{self, Result};
use crate::models::*;
use crate::{PgPooledConnection, PkmnapiSQL};

impl PkmnapiSQL {
    /// Select row from `roms` by ID
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
    /// # let new_rom = sql.insert_rom(&connection, &String::from("foo"), &vec![0x01, 0x02, 0x03, 0x04]).unwrap();
    /// # let id = new_rom.id;
    /// let rom = sql.select_rom_by_id(&connection, &id).unwrap().unwrap();
    ///
    /// assert_eq!(rom.id.len(), 32);
    /// assert_eq!(rom.date_create.len(), 20);
    /// assert_eq!(rom.name, String::from("foo"));
    /// assert_eq!(rom.etag.len(), 36);
    /// assert_eq!(rom.rom_data_id.len(), 32);
    /// # sql.revert_migration();
    /// ```
    pub fn select_rom_by_id(
        &self,
        connection: &PgPooledConnection,
        id: &String,
    ) -> Result<Option<Rom>> {
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
    /// # sql.revert_migration();
    /// ```
    pub fn insert_rom(
        &self,
        connection: &PgPooledConnection,
        name: &String,
        data: &Vec<u8>,
    ) -> Result<Rom> {
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
    ///
    /// let sql = PkmnapiSQL::new();
    ///
    /// let connection = sql.get_connection().unwrap();
    /// # let new_rom = sql.insert_rom(&connection, &String::from("foo"), &vec![0x01, 0x02, 0x03, 0x04]).unwrap();
    /// # let id = new_rom.id;
    /// sql.delete_rom_by_id(&connection, &id).unwrap();
    /// # sql.revert_migration();
    /// ```
    pub fn delete_rom_by_id(&self, connection: &PgPooledConnection, id: &String) -> Result<()> {
        use crate::schema::roms;

        match diesel::delete(roms::table.filter(roms::id.eq(id))).execute(connection) {
            Ok(_) => Ok(()),
            Err(e) => return Err(e.into()),
        }
    }
}
