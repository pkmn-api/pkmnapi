use pkmnapi_db::*;
use pkmnapi_sql::*;
use rocket::http::{ContentType, Header, Status};
use rocket::response::status;
use rocket::response::Response;
use rocket::{Data, State};
use std::io::Cursor;

use crate::guards::*;
use crate::responses::errors::*;
use crate::responses::roms::*;
use crate::utils;

#[post("/roms", data = "<data>")]
pub fn post_rom<'a>(
    sql: State<PkmnapiSQL>,
    data: Data,
    access_token: Result<AccessToken, AccessTokenError>,
) -> Result<Response<'a>, ResponseError> {
    let access_token = match access_token {
        Ok(access_token) => access_token.into_inner(),
        Err(_) => return Err(AccessTokenErrorUnauthorized::new()),
    };

    let rom = {
        let mut rom = Vec::new();

        data.stream_to(&mut rom).unwrap();

        rom
    };

    let db = match PkmnapiDB::new(&rom, None) {
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
    let body = serde_json::to_string(&response).unwrap();

    let response = Response::build()
        .status(Status::Created)
        .header(ContentType::JSON)
        .header(Header::new("Location", utils::generate_url("roms", None)))
        .header(Header::new("ETag", rom_sql.etag))
        .sized_body(Cursor::new(body))
        .finalize();

    Ok(response)
}

#[get("/roms")]
pub fn get_rom<'a>(
    sql: State<PkmnapiSQL>,
    access_token: Result<AccessToken, AccessTokenError>,
) -> Result<Response<'a>, ResponseError> {
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
    let body = serde_json::to_string(&response).unwrap();

    let response = Response::build()
        .header(ContentType::JSON)
        .header(Header::new("ETag", rom_sql.etag))
        .sized_body(Cursor::new(body))
        .finalize();

    Ok(response)
}

#[delete("/roms")]
pub fn delete_rom(
    sql: State<PkmnapiSQL>,
    access_token: Result<AccessToken, AccessTokenError>,
    if_match: Result<IfMatch, IfMatchError>,
) -> Result<status::NoContent, ResponseError> {
    let access_token = match access_token {
        Ok(access_token) => access_token.into_inner(),
        Err(_) => return Err(AccessTokenErrorUnauthorized::new()),
    };

    let etag = match if_match {
        Ok(if_match) => if_match.into_inner(),
        Err(_) => return Err(ETagErrorMissing::new()),
    };

    let connection = sql.get_connection().unwrap();
    match sql.delete_user_rom_by_access_token(&connection, &access_token, &etag) {
        Ok(_) => {}
        Err(pkmnapi_sql::error::Error::ETagError) => return Err(ETagErrorMismatch::new()),
        Err(_) => return Err(RomResponseErrorNoRom::new()),
    }

    Ok(status::NoContent)
}
