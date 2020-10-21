use diesel::prelude::*;

use crate::error::{self, Result};
use crate::models::*;
use crate::{PgPooledConnection, PkmnapiSQL};

impl PkmnapiSQL {
    /// Select row from `savs` by ID
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
    /// # let new_sav = sql.insert_sav(&connection, &vec![0x01, 0x02, 0x03, 0x04]).unwrap();
    /// # let id = new_sav.id;
    /// let sav = sql.select_sav_by_id(&connection, &id).unwrap().unwrap();
    ///
    /// assert_eq!(sav.id.len(), 32);
    /// assert_eq!(sav.date_create.len(), 20);
    /// assert_eq!(sav.data, vec![0x01, 0x02, 0x03, 0x04]);
    /// assert_eq!(sav.etag.len(), 36);
    /// # sql.revert_migration();
    /// ```
    pub fn select_sav_by_id(
        &self,
        connection: &PgPooledConnection,
        id: &String,
    ) -> Result<Option<Sav>> {
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
    /// # sql.revert_migration();
    /// ```
    pub fn insert_sav(&self, connection: &PgPooledConnection, data: &Vec<u8>) -> Result<Sav> {
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
    ///
    /// let sql = PkmnapiSQL::new();
    ///
    /// let connection = sql.get_connection().unwrap();
    /// # let new_sav = sql.insert_sav(&connection, &vec![0x01, 0x02, 0x03, 0x04]).unwrap();
    /// # let id = new_sav.id;
    /// sql.delete_sav_by_id(&connection, &id).unwrap();
    /// # sql.revert_migration();
    /// ```
    pub fn delete_sav_by_id(&self, connection: &PgPooledConnection, id: &String) -> Result<()> {
        use crate::schema::savs;

        match diesel::delete(savs::table.filter(savs::id.eq(id))).execute(connection) {
            Ok(_) => Ok(()),
            Err(e) => return Err(e.into()),
        }
    }
}
