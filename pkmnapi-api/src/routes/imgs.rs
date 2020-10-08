use pkmnapi_sql::*;
use rocket::http::{ContentType, Header};
use rocket::response::Response;
use rocket::State;
use std::io::Cursor;

use crate::guards::*;
use crate::responses::errors::*;
use crate::utils;

#[get("/imgs/pokemon_logo", format = "image/png", rank = 1)]
pub fn get_pokemon_logo_png<'a>(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
) -> Result<Response<'a>, ResponseError> {
    let access_token = match access_token {
        Ok(access_token) => access_token.into_inner(),
        Err(_) => return Err(AccessTokenErrorUnauthorized::new()),
    };

    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let pokemong_logo = match db.get_pokemon_logo_img() {
        Ok(pokemong_logo) => pokemong_logo,
        Err(e) => {
            return Err(NotFoundError::new(
                BaseErrorResponseId::error_pokemon_logo_imgs,
                Some(e.to_string()),
            ))
        }
    };

    let img = match pokemong_logo.to_png() {
        Ok(img) => img,
        Err(e) => {
            return Err(NotFoundError::new(
                BaseErrorResponseId::error_pokemon_logo_imgs,
                Some(e.to_string()),
            ))
        }
    };

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
    let access_token = match access_token {
        Ok(access_token) => access_token.into_inner(),
        Err(_) => return Err(AccessTokenErrorUnauthorized::new()),
    };

    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let pokemong_logo = match db.get_pokemon_logo_img() {
        Ok(pokemong_logo) => pokemong_logo,
        Err(e) => {
            return Err(NotFoundError::new(
                BaseErrorResponseId::error_pokemon_logo_imgs,
                Some(e.to_string()),
            ))
        }
    };

    let img = match pokemong_logo.to_jpeg() {
        Ok(img) => img,
        Err(e) => {
            return Err(NotFoundError::new(
                BaseErrorResponseId::error_pokemon_logo_imgs,
                Some(e.to_string()),
            ))
        }
    };

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

#[get("/imgs/town_map", format = "image/png", rank = 1)]
pub fn get_town_map_png<'a>(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
) -> Result<Response<'a>, ResponseError> {
    let access_token = match access_token {
        Ok(access_token) => access_token.into_inner(),
        Err(_) => return Err(AccessTokenErrorUnauthorized::new()),
    };

    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let pokemong_logo = match db.get_town_map_img() {
        Ok(pokemong_logo) => pokemong_logo,
        Err(e) => {
            return Err(NotFoundError::new(
                BaseErrorResponseId::error_town_map_imgs,
                Some(e.to_string()),
            ))
        }
    };

    let img = match pokemong_logo.to_png() {
        Ok(img) => img,
        Err(e) => {
            return Err(NotFoundError::new(
                BaseErrorResponseId::error_town_map_imgs,
                Some(e.to_string()),
            ))
        }
    };

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
    let access_token = match access_token {
        Ok(access_token) => access_token.into_inner(),
        Err(_) => return Err(AccessTokenErrorUnauthorized::new()),
    };

    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let pokemong_logo = match db.get_town_map_img() {
        Ok(pokemong_logo) => pokemong_logo,
        Err(e) => {
            return Err(NotFoundError::new(
                BaseErrorResponseId::error_town_map_imgs,
                Some(e.to_string()),
            ))
        }
    };

    let img = match pokemong_logo.to_jpeg() {
        Ok(img) => img,
        Err(e) => {
            return Err(NotFoundError::new(
                BaseErrorResponseId::error_town_map_imgs,
                Some(e.to_string()),
            ))
        }
    };

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
