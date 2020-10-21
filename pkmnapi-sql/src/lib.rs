//! Pkmnapi SQL module
//!
//! # Example
//!
//! ```
//! use pkmnapi_sql::*;
//! # use std::process::Command;
//!
//! let sql = PkmnapiSQL::new();
//! # sql.revert_migration();
//! ```

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
extern crate dotenv;

mod sql;

pub use sql::*;
pub mod error;
pub mod models;
pub mod schema;
pub mod utils;

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel_migrations::{embed_migrations, revert_latest_migration, RunMigrationsError};
use dotenv::dotenv;
use std::env;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

embed_migrations!();

/// PkmnapiSQL
///
/// # Example
///
/// ```
/// use pkmnapi_sql::*;
/// # use std::process::Command;
///
/// let sql = PkmnapiSQL::new();
/// # sql.revert_migration();
/// ```
pub struct PkmnapiSQL {
    connection: PgPool,
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
    /// # use std::process::Command;
    ///
    /// let sql = PkmnapiSQL::new();
    /// # sql.revert_migration();
    /// ```
    pub fn new() -> PkmnapiSQL {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let connection = Pool::builder()
            .build(manager)
            .expect("Failed to create database connection pool");

        let migration_connection = connection
            .get()
            .expect("Failed to create migration connection");

        PkmnapiSQL::run_migration(&migration_connection).expect("Failed to run migration");

        PkmnapiSQL { connection }
    }

    fn run_migration(connection: &PgConnection) -> Result<(), RunMigrationsError> {
        embedded_migrations::run_with_output(connection, &mut std::io::stdout())
    }

    pub fn revert_migration(&self) {
        let connection = self
            .get_connection()
            .expect("Failed to create migration connection");

        revert_latest_migration(&connection).expect("Failed to revert migration");
    }

    pub fn get_connection(&self) -> Result<PgPooledConnection, error::Error> {
        match self.connection.get() {
            Ok(connection) => Ok(connection),
            Err(_) => return Err(diesel::result::Error::NotFound.into()),
        }
    }
}
