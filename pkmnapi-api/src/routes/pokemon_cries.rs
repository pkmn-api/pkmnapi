use pkmnapi_db::cry::*;
use pkmnapi_sql::*;
use rocket::http::{ContentType, Header};
use rocket::response::status;
use rocket::response::Response;
use rocket::State;
use rocket_contrib::json::{Json, JsonError, JsonValue};
use rocket_okapi::openapi;
use std::io::Cursor;

use crate::guards::*;
use crate::requests::pokemon_cries::*;
use crate::responses::errors::*;
use crate::responses::pokemon_cries::*;
use crate::utils;

#[openapi]
#[get("/pokemon/cries", format = "application/json")]
pub fn get_pokemon_cry_all(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
) -> Result<Json<PokemonCryResponseAll>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let (min_pokedex_id, max_pokedex_id) = db.pokedex_id_bounds();
    let pokedex_ids: Vec<u8> = (min_pokedex_id..=max_pokedex_id)
        .map(|pokedex_id| pokedex_id as u8)
        .collect();
    let pokemon_cries = db.get_pokemon_cry_all(&pokedex_ids)?;

    let response = PokemonCryResponseAll::new(&pokedex_ids, &pokemon_cries);

    Ok(Json(response))
}

#[openapi]
#[get("/pokemon/cries/<pokedex_id>", format = "application/json", rank = 1)]
pub fn get_pokemon_cry_json(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
    pokedex_id: u8,
) -> Result<Json<PokemonCryResponse>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let pokemon_cry = db.get_pokemon_cry(&pokedex_id)?;

    let response = PokemonCryResponse::new(&pokedex_id, &pokemon_cry);

    Ok(Json(response))
}

#[get("/pokemon/cries/<pokedex_id>", format = "audio/wav", rank = 2)]
pub fn get_pokemon_cry_wav<'a>(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
    pokedex_id: u8,
) -> Result<Response<'a>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let pokemon_cry = db.get_pokemon_cry(&pokedex_id)?;
    let pokemon_name = db.get_pokemon_name(&pokedex_id)?;
    let wav = pokemon_cry.to_wav(48000)?;

    let response = Response::build()
        .header(ContentType::WAV)
        .header(Header::new(
            "Content-Disposition",
            format!(r#"attachment; filename="{}.wav""#, pokemon_name.name),
        ))
        .sized_body(Cursor::new(wav))
        .finalize();

    Ok(response)
}

#[openapi]
#[post(
    "/pokemon/cries/<pokedex_id>",
    format = "application/json",
    data = "<data>"
)]
pub fn post_pokemon_cry(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
    patch_description: Result<PatchDescription, PatchDescriptionError>,
    data: Result<Json<PokemonCryRequest>, JsonError>,
    pokedex_id: u8,
) -> Result<status::Accepted<JsonValue>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let data = utils::get_data(data, BaseErrorResponseId::error_pokemon_cries_invalid)?;
    let (db, connection) = utils::get_db(&sql, &access_token)?;

    let pokemon_cry = Cry {
        base: data.get_base(),
        pitch: data.get_pitch(),
        length: data.get_length(),
        ..Default::default()
    };

    let patch = db.set_pokemon_cry(&pokedex_id, &pokemon_cry)?;

    utils::insert_rom_patch(
        sql,
        connection,
        access_token,
        patch,
        patch_description,
        BaseErrorResponseId::error_pokemon_cries,
    )?;

    Ok(status::Accepted(Some(json!({}))))
}
