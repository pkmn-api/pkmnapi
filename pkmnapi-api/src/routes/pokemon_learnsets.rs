use pkmnapi_sql::*;
use rocket::response::status;
use rocket::State;
use rocket_contrib::json::{Json, JsonError, JsonValue};

use crate::guards::*;
use crate::requests::pokemon_learnsets::*;
use crate::responses::errors::*;
use crate::responses::pokemon_learnsets::*;
use crate::utils;

#[get("/pokemon/learnsets")]
pub fn get_pokemon_learnset_all(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
) -> Result<Json<PokemonLearnsetResponseAll>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let (min_pokedex_id, max_pokedex_id) = db.pokedex_id_bounds();
    let pokedex_ids: Vec<u8> = (min_pokedex_id..=max_pokedex_id)
        .map(|pokedex_id| pokedex_id as u8)
        .collect();
    let pokemon_learnsets = db.get_pokemon_learnset_all(&pokedex_ids)?;
    let move_ids = pokemon_learnsets
        .iter()
        .map(|(_, pokemon_learnset)| pokemon_learnset)
        .flatten()
        .map(|learnset| learnset.move_id)
        .collect();
    let move_names = db.get_move_name_all(&move_ids)?;

    let response = PokemonLearnsetResponseAll::new(&pokedex_ids, &pokemon_learnsets, &move_names);

    Ok(Json(response))
}

#[get("/pokemon/learnsets/<pokedex_id>")]
pub fn get_pokemon_learnset(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
    pokedex_id: u8,
) -> Result<Json<PokemonLearnsetResponse>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let pokemon_learnset = db.get_pokemon_learnset(&pokedex_id)?;
    let move_ids = pokemon_learnset
        .iter()
        .map(|learnset| learnset.move_id)
        .collect();
    let move_names = db.get_move_name_all(&move_ids)?;

    let response = PokemonLearnsetResponse::new(&pokedex_id, &pokemon_learnset, &move_names);

    Ok(Json(response))
}

#[post(
    "/pokemon/learnsets/<pokedex_id>",
    format = "application/json",
    data = "<data>"
)]
pub fn post_pokemon_learnset(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
    patch_description: Result<PatchDescription, PatchDescriptionError>,
    data: Result<Json<PokemonLearnsetRequest>, JsonError>,
    pokedex_id: u8,
) -> Result<status::Accepted<JsonValue>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let data = utils::get_data(data, BaseErrorResponseId::error_pokemon_learnsets_invalid)?;
    let (db, connection) = utils::get_db(&sql, &access_token)?;

    let pokemon_learnset = data.get_learnset();

    let patch = db.set_pokemon_learnset(&pokedex_id, &pokemon_learnset)?;

    utils::insert_rom_patch(
        sql,
        connection,
        access_token,
        patch,
        patch_description,
        BaseErrorResponseId::error_pokemon_learnsets,
    )?;

    Ok(status::Accepted(Some(json!({}))))
}
