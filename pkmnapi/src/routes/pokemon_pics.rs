use pkmnapi_db::pic::*;
use pkmnapi_db::types::*;
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

#[get("/pokemon/pics/<pokedex_id>?<face>", format = "image/png", rank = 1)]
pub fn get_pokemon_pic_png<'a>(
    sql: State<PkmnapiSQL>,
    access_token: Result<AccessToken, AccessTokenError>,
    pokedex_id: u8,
    face: Option<String>,
) -> Result<Response<'a>, ResponseError> {
    let access_token = match access_token {
        Ok(access_token) => access_token.into_inner(),
        Err(_) => return Err(AccessTokenErrorUnauthorized::new()),
    };

    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let pic = match db.get_pokemon_pic(&pokedex_id, &PokemonPicFace::from(face)) {
        Ok(pic) => pic,
        Err(e) => return Err(PokemonPicResponseError::new(&e.to_string())),
    };

    let pokemon_name = match db.get_pokemon_name(&pokedex_id) {
        Ok(pokemon_name) => pokemon_name,
        Err(e) => return Err(PokemonPicResponseError::new(&e.to_string())),
    };

    let img = match pic.to_png() {
        Ok(img) => img,
        Err(e) => return Err(PokemonPicResponseError::new(&e.to_string())),
    };

    let response = Response::build()
        .header(ContentType::new("image", "png"))
        .header(Header::new(
            "Content-Disposition",
            format!(r#"attachment; filename="{}.png""#, pokemon_name.name),
        ))
        .sized_body(Cursor::new(img))
        .finalize();

    Ok(response)
}

#[get("/pokemon/pics/<pokedex_id>?<face>", format = "image/jpeg", rank = 2)]
pub fn get_pokemon_pic_jpeg<'a>(
    sql: State<PkmnapiSQL>,
    access_token: Result<AccessToken, AccessTokenError>,
    pokedex_id: u8,
    face: Option<String>,
) -> Result<Response<'a>, ResponseError> {
    let access_token = match access_token {
        Ok(access_token) => access_token.into_inner(),
        Err(_) => return Err(AccessTokenErrorUnauthorized::new()),
    };

    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let pic = match db.get_pokemon_pic(&pokedex_id, &PokemonPicFace::from(face)) {
        Ok(pic) => pic,
        Err(e) => return Err(PokemonPicResponseError::new(&e.to_string())),
    };

    let pokemon_name = match db.get_pokemon_name(&pokedex_id) {
        Ok(pokemon_name) => pokemon_name,
        Err(e) => return Err(PokemonPicResponseError::new(&e.to_string())),
    };

    let img = match pic.to_jpeg() {
        Ok(img) => img,
        Err(e) => return Err(PokemonPicResponseError::new(&e.to_string())),
    };

    let response = Response::build()
        .header(ContentType::new("image", "jpeg"))
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
    data: Data,
    access_token: Result<AccessToken, AccessTokenError>,
    patch_description: Result<PatchDescription, PatchDescriptionError>,
    pokedex_id: u8,
    face: Option<String>,
    method: Option<u8>,
    primary: Option<u8>,
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

    let pic = match Pic::from_png(raw_data) {
        Ok(pic) => pic,
        Err(e) => return Err(PokemonPicResponseError::new(&e.to_string())),
    };

    let encoding_method = PicEncodingMethod::from(method.unwrap_or(0x01), primary.unwrap_or(0x00));

    let patch = match db.set_pokemon_pic(
        &pokedex_id,
        &PokemonPicFace::from(face),
        &pic,
        encoding_method,
    ) {
        Ok(patch) => patch,
        Err(e) => return Err(PokemonPicResponseError::new(&e.to_string())),
    };

    let patch_description = match patch_description {
        Ok(patch_description) => patch_description.into_inner(),
        Err(_) => None,
    };

    match sql.insert_rom_patch(
        &connection,
        &access_token,
        &patch.to_raw(),
        patch_description,
    ) {
        Ok(_) => {}
        Err(e) => return Err(PokemonPicResponseError::new(&e.to_string())),
    };

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
    data: Data,
    access_token: Result<AccessToken, AccessTokenError>,
    patch_description: Result<PatchDescription, PatchDescriptionError>,
    pokedex_id: u8,
    face: Option<String>,
    method: Option<u8>,
    primary: Option<u8>,
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

    let pic = match Pic::from_jpeg(raw_data) {
        Ok(pic) => pic,
        Err(e) => return Err(PokemonPicResponseError::new(&e.to_string())),
    };

    let encoding_method = PicEncodingMethod::from(method.unwrap_or(0x01), primary.unwrap_or(0x00));

    let patch = match db.set_pokemon_pic(
        &pokedex_id,
        &PokemonPicFace::from(face),
        &pic,
        encoding_method,
    ) {
        Ok(patch) => patch,
        Err(e) => return Err(PokemonPicResponseError::new(&e.to_string())),
    };

    let patch_description = match patch_description {
        Ok(patch_description) => patch_description.into_inner(),
        Err(_) => None,
    };

    match sql.insert_rom_patch(
        &connection,
        &access_token,
        &patch.to_raw(),
        patch_description,
    ) {
        Ok(_) => {}
        Err(e) => return Err(PokemonPicResponseError::new(&e.to_string())),
    };

    Ok(status::Accepted(Some(json!({}))))
}
