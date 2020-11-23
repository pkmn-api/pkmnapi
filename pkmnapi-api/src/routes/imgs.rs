use pkmnapi_db::img::*;
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

#[get("/imgs/game_boy", format = "image/png", rank = 1)]
pub fn get_game_boy_png<'a>(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
) -> Result<Response<'a>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let game_boy = db.get_game_boy_img()?;
    let img = game_boy.to_png()?;

    let response = Response::build()
        .header(ContentType::PNG)
        .header(Header::new(
            "Content-Disposition",
            r#"attachment; filename="game_boy.png""#,
        ))
        .sized_body(Cursor::new(img))
        .finalize();

    Ok(response)
}

#[get("/imgs/game_boy", format = "image/jpeg", rank = 2)]
pub fn get_game_boy_jpeg<'a>(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
) -> Result<Response<'a>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let game_boy = db.get_game_boy_img()?;
    let img = game_boy.to_jpeg()?;

    let response = Response::build()
        .header(ContentType::JPEG)
        .header(Header::new(
            "Content-Disposition",
            r#"attachment; filename="game_boy.jpg""#,
        ))
        .sized_body(Cursor::new(img))
        .finalize();

    Ok(response)
}

#[get("/imgs/pokemon_logo", format = "image/png", rank = 1)]
pub fn get_pokemon_logo_png<'a>(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
) -> Result<Response<'a>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let pokemon_logo = db.get_pokemon_logo_img()?;
    let img = pokemon_logo.to_png()?;

    let response = Response::build()
        .header(ContentType::PNG)
        .header(Header::new(
            "Content-Disposition",
            r#"attachment; filename="pokemon_logo.png""#,
        ))
        .sized_body(Cursor::new(img))
        .finalize();

    Ok(response)
}

#[get("/imgs/pokemon_logo", format = "image/jpeg", rank = 2)]
pub fn get_pokemon_logo_jpeg<'a>(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
) -> Result<Response<'a>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let pokemon_logo = db.get_pokemon_logo_img()?;
    let img = pokemon_logo.to_jpeg()?;

    let response = Response::build()
        .header(ContentType::JPEG)
        .header(Header::new(
            "Content-Disposition",
            r#"attachment; filename="pokemon_logo.jpg""#,
        ))
        .sized_body(Cursor::new(img))
        .finalize();

    Ok(response)
}

#[post("/imgs/pokemon_logo", format = "image/png", data = "<data>", rank = 1)]
pub fn post_pokemon_logo_png<'a>(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    data: Data,
    access_token: Result<AccessToken, AccessTokenError>,
    patch_description: Result<PatchDescription, PatchDescriptionError>,
) -> Result<status::Accepted<JsonValue>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let (db, connection) = utils::get_db(&sql, &access_token)?;
    let raw_data = utils::get_data_raw(data);

    let img = Img::from_png(raw_data)?;
    let patch = db.set_pokemon_logo_img(&img)?;

    utils::insert_rom_patch(
        sql,
        connection,
        access_token,
        patch,
        patch_description,
        BaseErrorResponseId::error_pokemon_logo_imgs,
    )?;

    Ok(status::Accepted(Some(json!({}))))
}

#[post("/imgs/pokemon_logo", format = "image/jpeg", data = "<data>", rank = 2)]
pub fn post_pokemon_logo_jpeg<'a>(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    data: Data,
    access_token: Result<AccessToken, AccessTokenError>,
    patch_description: Result<PatchDescription, PatchDescriptionError>,
) -> Result<status::Accepted<JsonValue>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let (db, connection) = utils::get_db(&sql, &access_token)?;
    let raw_data = utils::get_data_raw(data);

    let img = Img::from_jpeg(raw_data)?;
    let patch = db.set_pokemon_logo_img(&img)?;

    utils::insert_rom_patch(
        sql,
        connection,
        access_token,
        patch,
        patch_description,
        BaseErrorResponseId::error_pokemon_logo_imgs,
    )?;

    Ok(status::Accepted(Some(json!({}))))
}

#[get("/imgs/town_map", format = "image/png", rank = 1)]
pub fn get_town_map_png<'a>(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
) -> Result<Response<'a>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let town_map = db.get_town_map_img()?;
    let img = town_map.to_png()?;

    let response = Response::build()
        .header(ContentType::PNG)
        .header(Header::new(
            "Content-Disposition",
            r#"attachment; filename="town_map.png""#,
        ))
        .sized_body(Cursor::new(img))
        .finalize();

    Ok(response)
}

#[get("/imgs/town_map", format = "image/jpeg", rank = 2)]
pub fn get_town_map_jpeg<'a>(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
) -> Result<Response<'a>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let town_map = db.get_town_map_img()?;
    let img = town_map.to_jpeg()?;

    let response = Response::build()
        .header(ContentType::JPEG)
        .header(Header::new(
            "Content-Disposition",
            r#"attachment; filename="town_map.jpg""#,
        ))
        .sized_body(Cursor::new(img))
        .finalize();

    Ok(response)
}
