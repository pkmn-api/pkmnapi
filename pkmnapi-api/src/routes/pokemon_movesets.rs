use pkmnapi_sql::*;
use rocket::response::status;
use rocket::State;
use rocket_contrib::json::{Json, JsonError, JsonValue};
use rocket_okapi::openapi;

use crate::guards::*;
use crate::requests::pokemon_movesets::*;
use crate::responses::errors::*;
use crate::responses::pokemon_movesets::*;
use crate::utils;

#[openapi]
#[get("/pokemon/movesets")]
pub fn get_pokemon_moveset_all(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
) -> Result<Json<PokemonMovesetResponseAll>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let (min_pokedex_id, max_pokedex_id) = db.pokedex_id_bounds();
    let pokedex_ids: Vec<u8> = (min_pokedex_id..=max_pokedex_id)
        .map(|pokedex_id| pokedex_id as u8)
        .collect();
    let pokemon_movesets = db.get_pokemon_moveset_all(&pokedex_ids)?;
    let move_ids = pokemon_movesets
        .iter()
        .map(|(_, pokemon_moveset)| pokemon_moveset.to_vec())
        .flatten()
        .collect();
    let move_names = db.get_move_name_all(&move_ids)?;

    let response = PokemonMovesetResponseAll::new(&pokedex_ids, &pokemon_movesets, &move_names);

    Ok(Json(response))
}

#[openapi]
#[get("/pokemon/movesets/<pokedex_id>")]
pub fn get_pokemon_moveset(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
    pokedex_id: u8,
) -> Result<Json<PokemonMovesetResponse>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let pokemon_moveset = db.get_pokemon_moveset(&pokedex_id)?;
    let move_names = db.get_move_name_all(&pokemon_moveset)?;

    let response = PokemonMovesetResponse::new(&pokedex_id, &pokemon_moveset, &move_names);

    Ok(Json(response))
}

#[openapi]
#[post(
    "/pokemon/movesets/<pokedex_id>",
    format = "application/json",
    data = "<data>"
)]
pub fn post_pokemon_moveset(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
    patch_description: Result<PatchDescription, PatchDescriptionError>,
    data: Result<Json<PokemonMovesetRequest>, JsonError>,
    pokedex_id: u8,
) -> Result<status::Accepted<JsonValue>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let data = utils::get_data(data, BaseErrorResponseId::error_pokemon_movesets_invalid)?;
    let (db, connection) = utils::get_db(&sql, &access_token)?;

    let pokemon_moveset = data.get_moveset();

    let patch = db.set_pokemon_moveset(&pokedex_id, &pokemon_moveset)?;

    utils::insert_rom_patch(
        sql,
        connection,
        access_token,
        patch,
        patch_description,
        BaseErrorResponseId::error_pokemon_movesets,
    )?;

    Ok(status::Accepted(Some(json!({}))))
}
