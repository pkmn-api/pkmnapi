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
    _rate_limit: RateLimit,
    data: Data,
    access_token: Result<AccessToken, AccessTokenError>,
) -> Result<Response<'a>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let rom_data = utils::get_data_raw(data);

    let db = match PkmnapiDB::new(&rom_data, None) {
        Ok(db) => db,
        Err(_) => return Err(RomErrorInvalidRom::new()),
    };

    let connection = sql.get_connection().unwrap();
    let rom = match sql.update_user_rom_by_access_token(
        &connection,
        &access_token,
        &db.header.title,
        &rom_data,
    ) {
        Ok(rom) => rom,
        Err(_) => return Err(RomErrorRomExists::new()),
    };

    let response = RomResponse::new(&rom);
    let body = serde_json::to_string(&response).unwrap();

    let response = Response::build()
        .status(Status::Created)
        .header(ContentType::JSON)
        .header(Header::new("Location", utils::generate_url("roms", None)))
        .header(Header::new("ETag", rom.etag))
        .sized_body(Cursor::new(body))
        .finalize();

    Ok(response)
}

#[get("/roms")]
pub fn get_rom<'a>(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
) -> Result<Response<'a>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;

    let connection = sql.get_connection().unwrap();
    let rom_sql = match sql.select_user_rom_by_access_token(&connection, &access_token) {
        Ok(Some(rom_sql)) => rom_sql,
        _ => return Err(RomErrorNoRom::new()),
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
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
    if_match: Result<IfMatch, IfMatchError>,
) -> Result<status::NoContent, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let etag = utils::get_etag(if_match)?;
    let connection = sql.get_connection().unwrap();

    match sql.delete_user_rom_by_access_token(&connection, &access_token, &etag) {
        Ok(_) => {}
        Err(pkmnapi_sql::error::Error::ETagError) => return Err(ETagErrorMismatch::new()),
        Err(_) => return Err(RomErrorNoRom::new()),
    }

    Ok(status::NoContent)
}
