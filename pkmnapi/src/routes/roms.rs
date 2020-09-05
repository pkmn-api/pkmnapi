use pkmnapi_db::*;
use pkmnapi_sql::*;
use rocket::response::status;
use rocket::{Data, State};
use rocket_contrib::json::Json;

use crate::guards::access_tokens::AccessToken;
use crate::responses::errors::*;
use crate::responses::roms::*;

#[post("/roms", data = "<data>")]
pub fn post_rom(
    sql: State<PkmnapiSQL>,
    data: Data,
    access_token: AccessToken,
) -> Result<status::Created<Json<RomResponse>>, ResponseError> {
    let rom = {
        let mut rom = Vec::new();

        data.stream_to(&mut rom).unwrap();

        rom
    };

    let db = match PkmnapiDB::new(&rom) {
        Ok(db) => db,
        Err(_) => {
            let response = RomResponseErrorInvalidRom::new();

            return Err(ResponseError::RomResponseErrorInvalidRom(
                status::BadRequest(Some(Json(response))),
            ));
        }
    };

    let access_token = access_token.into_inner();

    let connection = match sql.get_connection() {
        Ok(connection) => connection,
        Err(e) => panic!("FAILED: {:?}", e),
    };
    let rom_sql = match sql.update_user_rom_by_access_token(
        &connection,
        &access_token,
        &db.header.title,
        &rom,
    ) {
        Ok(rom_sql) => rom_sql,
        Err(_) => {
            let response = RomResponseErrorRomExists::new();

            return Err(ResponseError::RomResponseErrorRomExists(status::Forbidden(
                Some(Json(response)),
            )));
        }
    };

    let response = RomResponse::new(&rom_sql);

    Ok(status::Created("foo".to_string(), Some(Json(response))))
}

#[get("/roms")]
pub fn get_rom(
    sql: State<PkmnapiSQL>,
    access_token: AccessToken,
) -> Result<Json<RomResponse>, ResponseError> {
    let access_token = access_token.into_inner();

    let connection = sql.get_connection().unwrap();
    let rom_sql = match sql.select_user_rom_by_access_token(&connection, &access_token) {
        Ok(rom_sql) => rom_sql,
        Err(_) => {
            let response = RomResponseErrorNoRom::new();

            return Err(ResponseError::RomResponseErrorNoRom(status::BadRequest(
                Some(Json(response)),
            )));
        }
    };

    let response = RomResponse::new(&rom_sql);

    Ok(Json(response))
}

#[delete("/roms")]
pub fn delete_rom(
    sql: State<PkmnapiSQL>,
    access_token: AccessToken,
) -> Result<status::NoContent, ResponseError> {
    let access_token = access_token.into_inner();

    let connection = sql.get_connection().unwrap();
    match sql.delete_user_rom_by_access_token(&connection, &access_token) {
        Ok(_) => {}
        Err(_) => {
            let response = RomResponseErrorNoRom::new();

            return Err(ResponseError::RomResponseErrorNoRom(status::BadRequest(
                Some(Json(response)),
            )));
        }
    }

    Ok(status::NoContent)
}
