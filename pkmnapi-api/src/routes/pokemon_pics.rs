use pkmnapi_db::pic::*;
use pkmnapi_db::*;
use pkmnapi_sql::*;
use rocket::http::{ContentType, Header};
use rocket::response::status;
use rocket::response::Response;
use rocket::{Data, State};
use rocket_contrib::json::JsonValue;
use std::io::Cursor;

use crate::guards::*;
use crate::responses::errors::*;
use crate::utils;

#[get(
    "/pokemon/pics/<pokedex_id>?<face>&<mirror>",
    format = "image/png",
    rank = 1
)]
pub fn get_pokemon_pic_png<'a>(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
    pokedex_id: u8,
    face: Option<String>,
    mirror: Option<bool>,
) -> Result<Response<'a>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let pic = db.get_pokemon_pic(&pokedex_id, &PokemonPicFace::from(face))?;
    let pokemon_name = db.get_pokemon_name(&pokedex_id)?;
    let img = pic.to_png(mirror.is_some())?;

    let response = Response::build()
        .header(ContentType::PNG)
        .header(Header::new(
            "Content-Disposition",
            format!(r#"attachment; filename="{}.png""#, pokemon_name.name),
        ))
        .sized_body(Cursor::new(img))
        .finalize();

    Ok(response)
}

#[get(
    "/pokemon/pics/<pokedex_id>?<face>&<mirror>",
    format = "image/jpeg",
    rank = 2
)]
pub fn get_pokemon_pic_jpeg<'a>(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
    pokedex_id: u8,
    face: Option<String>,
    mirror: Option<bool>,
) -> Result<Response<'a>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let pic = db.get_pokemon_pic(&pokedex_id, &PokemonPicFace::from(face))?;
    let pokemon_name = db.get_pokemon_name(&pokedex_id)?;
    let img = pic.to_jpeg(mirror.is_some())?;

    let response = Response::build()
        .header(ContentType::JPEG)
        .header(Header::new(
            "Content-Disposition",
            format!(r#"attachment; filename="{}.jpg""#, pokemon_name.name),
        ))
        .sized_body(Cursor::new(img))
        .finalize();

    Ok(response)
}

#[post(
    "/pokemon/pics/<pokedex_id>?<face>&<method>&<primary>",
    format = "image/png",
    data = "<data>",
    rank = 1
)]
pub fn post_pokemon_pic_png<'a>(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    data: Data,
    access_token: Result<AccessToken, AccessTokenError>,
    patch_description: Result<PatchDescription, PatchDescriptionError>,
    pokedex_id: u8,
    face: Option<String>,
    method: Option<u8>,
    primary: Option<u8>,
) -> Result<status::Accepted<JsonValue>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let (db, connection) = utils::get_db(&sql, &access_token)?;
    let raw_data = utils::get_data_raw(data);

    let pic = Pic::from_png(raw_data)?;
    let encoding_method = PicEncodingMethod::from(method.unwrap_or(0x01), primary.unwrap_or(0x00));
    let patch = db.set_pokemon_pic(
        &pokedex_id,
        &PokemonPicFace::from(face),
        &pic,
        encoding_method,
    )?;

    utils::insert_rom_patch(
        sql,
        connection,
        access_token,
        patch,
        patch_description,
        BaseErrorResponseId::error_pokemon_pics,
    )?;

    Ok(status::Accepted(Some(json!({}))))
}

#[post(
    "/pokemon/pics/<pokedex_id>?<face>&<method>&<primary>",
    format = "image/jpeg",
    data = "<data>",
    rank = 2
)]
pub fn post_pokemon_pic_jpeg<'a>(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    data: Data,
    access_token: Result<AccessToken, AccessTokenError>,
    patch_description: Result<PatchDescription, PatchDescriptionError>,
    pokedex_id: u8,
    face: Option<String>,
    method: Option<u8>,
    primary: Option<u8>,
) -> Result<status::Accepted<JsonValue>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let (db, connection) = utils::get_db(&sql, &access_token)?;
    let raw_data = utils::get_data_raw(data);

    let pic = Pic::from_jpeg(raw_data)?;
    let encoding_method = PicEncodingMethod::from(method.unwrap_or(0x01), primary.unwrap_or(0x00));
    let patch = db.set_pokemon_pic(
        &pokedex_id,
        &PokemonPicFace::from(face),
        &pic,
        encoding_method,
    )?;

    utils::insert_rom_patch(
        sql,
        connection,
        access_token,
        patch,
        patch_description,
        BaseErrorResponseId::error_pokemon_pics,
    )?;

    Ok(status::Accepted(Some(json!({}))))
}
