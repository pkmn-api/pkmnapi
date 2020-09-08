use pkmnapi_db::*;
use pkmnapi_sql::*;
use rocket::State;
use std::env;

use crate::responses::errors::*;

pub fn get_db_with_applied_patches(
    sql: State<PkmnapiSQL>,
    access_token: &String,
) -> Result<PkmnapiDB, ResponseError> {
    let connection = sql.get_connection().unwrap();
    let rom_data_sql = match sql.select_user_rom_data_by_access_token(&connection, &access_token) {
        Ok(Some(rom_sql)) => rom_sql,
        _ => return Err(RomResponseErrorNoRom::new()),
    };

    let patches = match sql.select_patches_by_access_token(&connection, &access_token) {
        Ok(patches) => patches,
        Err(_) => vec![],
    };

    let mut db = match PkmnapiDB::new(&rom_data_sql.data) {
        Ok(db) => db,
        Err(_) => return Err(RomResponseErrorInvalidRom::new()),
    };

    for patch in patches {
        db.apply_patch(patch.data);
    }

    Ok(db)
}

pub fn generate_url(route: &str, resource: Option<&String>) -> String {
    let version = env::var("API_VERSION").unwrap_or("1".to_string());
    let port = env::var("ROCKET_PORT").unwrap_or("80".to_string());
    let protocol = match port.as_str() {
        "433" => "https".to_string(),
        _ => "http".to_string(),
    };
    let address = env::var("ROCKET_ADDRESS").unwrap_or("localhost".to_string());
    let host = match address.as_str() {
        "localhost" => format!("{}:{}", address, port),
        _ => address,
    };
    let resource = match &resource {
        Some(resource) => format!("/{}", resource),
        None => "".to_string(),
    };

    format!("{}://{}/v{}/{}{}", protocol, host, version, route, resource)
}
