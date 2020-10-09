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

#[post("/imgs/pokemon_logo", format = "image/png", data = "<data>", rank = 1)]
pub fn post_pokemon_logo_png<'a>(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    data: Data,
    access_token: Result<AccessToken, AccessTokenError>,
    patch_description: Result<PatchDescription, PatchDescriptionError>,
) -> Result<status::Accepted<JsonValue>, ResponseError> {
    let access_token = match access_token {
        Ok(access_token) => access_token.into_inner(),
        Err(_) => return Err(AccessTokenErrorUnauthorized::new()),
    };

    let (db, connection) = utils::get_db(&sql, &access_token)?;

    let raw_data = {
        let mut raw_data = Vec::new();

        data.stream_to(&mut raw_data).unwrap();

        raw_data
    };

    let img = match Img::from_png(raw_data) {
        Ok(img) => img,
        Err(e) => {
            return Err(NotFoundError::new(
                BaseErrorResponseId::error_pokemon_logo_imgs,
                Some(e.to_string()),
            ))
        }
    };

    let patch = match db.set_pokemon_logo_img(&img) {
        Ok(patch) => patch,
        Err(e) => {
            return Err(NotFoundError::new(
                BaseErrorResponseId::error_pokemon_logo_imgs,
                Some(e.to_string()),
            ))
        }
    };

    let patch_description = match patch_description {
        Ok(patch_description) => patch_description.into_inner(),
        Err(_) => None,
    };

    if let Err(e) = sql.insert_rom_patch(
        &connection,
        &access_token,
        &patch.to_raw(),
        patch_description,
    ) {
        return Err(NotFoundError::new(
            BaseErrorResponseId::error_pokemon_pics,
            Some(e.to_string()),
        ));
    }

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
    let access_token = match access_token {
        Ok(access_token) => access_token.into_inner(),
        Err(_) => return Err(AccessTokenErrorUnauthorized::new()),
    };

    let (db, connection) = utils::get_db(&sql, &access_token)?;

    let raw_data = {
        let mut raw_data = Vec::new();

        data.stream_to(&mut raw_data).unwrap();

        raw_data
    };

    let img = match Img::from_jpeg(raw_data) {
        Ok(img) => img,
        Err(e) => {
            return Err(NotFoundError::new(
                BaseErrorResponseId::error_pokemon_logo_imgs,
                Some(e.to_string()),
            ))
        }
    };

    let patch = match db.set_pokemon_logo_img(&img) {
        Ok(patch) => patch,
        Err(e) => {
            return Err(NotFoundError::new(
                BaseErrorResponseId::error_pokemon_logo_imgs,
                Some(e.to_string()),
            ))
        }
    };

    let patch_description = match patch_description {
        Ok(patch_description) => patch_description.into_inner(),
        Err(_) => None,
    };

    if let Err(e) = sql.insert_rom_patch(
        &connection,
        &access_token,
        &patch.to_raw(),
        patch_description,
    ) {
        return Err(NotFoundError::new(
            BaseErrorResponseId::error_pokemon_pics,
            Some(e.to_string()),
        ));
    }

    Ok(status::Accepted(Some(json!({}))))
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
