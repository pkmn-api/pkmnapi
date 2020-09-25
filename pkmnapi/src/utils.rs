use pkmnapi_db::*;
use pkmnapi_sql::*;
use rocket::State;
use std::env;

use crate::responses::errors::*;

pub fn get_db(
    sql: &State<PkmnapiSQL>,
    access_token: &String,
) -> Result<(PkmnapiDB, PgPooledConnection), ResponseError> {
    let connection = sql.get_connection().unwrap();
    let rom_data = match sql.select_user_rom_data_by_access_token(&connection, &access_token) {
        Ok(Some(rom_data)) => rom_data,
        _ => return Err(RomResponseErrorNoRom::new()),
    };
    let sav = match sql.select_user_sav_by_access_token(&connection, &access_token) {
        Ok(Some(sav)) => Some(sav.data),
        _ => None,
    };

    let db = match PkmnapiDB::new(&rom_data.data, sav) {
        Ok(db) => db,
        Err(_) => return Err(RomResponseErrorInvalidRom::new()),
    };

    Ok((db, connection))
}

pub fn get_db_with_applied_patches(
    sql: &State<PkmnapiSQL>,
    access_token: &String,
) -> Result<(PkmnapiDB, PgPooledConnection), ResponseError> {
    let (mut db, connection) = get_db(sql, access_token)?;

    let rom_patches = match sql.select_rom_patches_by_access_token(&connection, &access_token) {
        Ok(patches) => patches,
        Err(_) => vec![],
    };

    let sav_patches = match sql.select_sav_patches_by_access_token(&connection, &access_token) {
        Ok(patches) => patches,
        Err(_) => vec![],
    };

    for patch in rom_patches {
        db.apply_patch(patch.data);
    }

    if let Some(ref mut sav) = db.sav {
        for patch in sav_patches {
            sav.apply_patch(patch.data);
        }
    }

    Ok((db, connection))
}

pub fn generate_url(route: &str, resource: Option<&String>) -> String {
    let version = env::var("API_VERSION").unwrap_or("1".to_owned());
    let port = env::var("ROCKET_PORT").unwrap_or("80".to_owned());
    let protocol = match port.as_str() {
        "433" => "https".to_owned(),
        _ => "http".to_owned(),
    };
    let address = env::var("ROCKET_ADDRESS").unwrap_or("localhost".to_owned());
    let host = match address.as_str() {
        "localhost" => format!("{}:{}", address, port),
        _ => address,
    };
    let resource = match &resource {
        Some(resource) => format!("/{}", resource),
        None => "".to_owned(),
    };

    format!("{}://{}/v{}/{}{}", protocol, host, version, route, resource)
}
