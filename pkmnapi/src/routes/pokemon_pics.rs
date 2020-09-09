use pkmnapi_db::types::*;
use pkmnapi_sql::*;
use rocket::http::{ContentType, Header};
use rocket::response::Response;
use rocket::State;
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

    let db = utils::get_db_with_applied_patches(sql, &access_token)?;

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

    let db = utils::get_db_with_applied_patches(sql, &access_token)?;

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
