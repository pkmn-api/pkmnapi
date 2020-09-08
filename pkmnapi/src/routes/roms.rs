use pkmnapi_db::*;
use pkmnapi_sql::*;
use rocket::response::status;
use rocket::{Data, State};
use rocket_contrib::json::Json;

use crate::guards::*;
use crate::responses::errors::*;
use crate::responses::roms::*;

#[post("/roms", data = "<data>")]
pub fn post_rom(
    sql: State<PkmnapiSQL>,
    data: Data,
    access_token: Result<AccessToken, AccessTokenError>,
) -> Result<status::Created<Json<RomResponse>>, ResponseError> {
    let access_token = match access_token {
        Ok(access_token) => access_token.into_inner(),
        Err(_) => return Err(AccessTokenErrorUnauthorized::new()),
    };

    let rom = {
        let mut rom = Vec::new();

        data.stream_to(&mut rom).unwrap();

        rom
    };

    let db = match PkmnapiDB::new(&rom) {
        Ok(db) => db,
        Err(_) => return Err(RomResponseErrorInvalidRom::new()),
    };

    let connection = sql.get_connection().unwrap();
    let rom_sql = match sql.update_user_rom_by_access_token(
        &connection,
        &access_token,
        &db.header.title,
        &rom,
    ) {
        Ok(rom_sql) => rom_sql,
        Err(_) => return Err(RomResponseErrorRomExists::new()),
    };

    let response = RomResponse::new(&rom_sql);

    Ok(status::Created("foo".to_string(), Some(Json(response))))
}

#[get("/roms")]
pub fn get_rom(
    sql: State<PkmnapiSQL>,
    access_token: Result<AccessToken, AccessTokenError>,
) -> Result<Json<RomResponse>, ResponseError> {
    let access_token = match access_token {
        Ok(access_token) => access_token.into_inner(),
        Err(_) => return Err(AccessTokenErrorUnauthorized::new()),
    };

    let connection = sql.get_connection().unwrap();
    let rom_sql = match sql.select_user_rom_by_access_token(&connection, &access_token) {
        Ok(Some(rom_sql)) => rom_sql,
        _ => return Err(RomResponseErrorNoRom::new()),
    };

    let response = RomResponse::new(&rom_sql);

    Ok(Json(response))
}

#[delete("/roms")]
pub fn delete_rom(
    sql: State<PkmnapiSQL>,
    access_token: Result<AccessToken, AccessTokenError>,
) -> Result<status::NoContent, ResponseError> {
    let access_token = match access_token {
        Ok(access_token) => access_token.into_inner(),
        Err(_) => return Err(AccessTokenErrorUnauthorized::new()),
    };

    let connection = sql.get_connection().unwrap();
    match sql.delete_user_rom_by_access_token(&connection, &access_token) {
        Ok(_) => {}
        Err(_) => return Err(RomResponseErrorNoRom::new()),
    }

    Ok(status::NoContent)
}
