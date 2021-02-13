use pkmnapi_db::*;
use pkmnapi_sql::*;
use rocket::response::status;
use rocket::State;
use rocket_contrib::json::{Json, JsonError, JsonValue};
use rocket_okapi::openapi;

use crate::guards::*;
use crate::requests::pokemon_stats::*;
use crate::responses::errors::*;
use crate::responses::pokemon_stats::*;
use crate::utils;

#[openapi]
#[get("/pokemon/stats")]
pub fn get_pokemon_stats_all(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
) -> Result<Json<PokemonStatsResponseAll>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let (min_pokedex_id, max_pokedex_id) = db.pokedex_id_bounds();
    let pokedex_ids: Vec<u8> = (min_pokedex_id..=max_pokedex_id)
        .map(|pokedex_id| pokedex_id as u8)
        .collect();
    let pokemon_stats = db.get_pokemon_stats_all(&pokedex_ids)?;
    let type_ids = pokemon_stats
        .iter()
        .map(|(_, pokemon_stats)| pokemon_stats.type_ids.to_vec())
        .flatten()
        .collect();
    let type_names = db.get_type_name_all(&type_ids)?;

    let response = PokemonStatsResponseAll::new(&pokedex_ids, &pokemon_stats, &type_names);

    Ok(Json(response))
}

#[openapi]
#[get("/pokemon/stats/<pokedex_id>")]
pub fn get_pokemon_stats(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
    pokedex_id: u8,
) -> Result<Json<PokemonStatsResponse>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let pokemon_stats = db.get_pokemon_stats(&pokedex_id)?;
    let type_names = db.get_type_name_all(&pokemon_stats.type_ids)?;

    let response = PokemonStatsResponse::new(&pokedex_id, &pokemon_stats, &type_names);

    Ok(Json(response))
}

#[openapi]
#[post(
    "/pokemon/stats/<pokedex_id>",
    format = "application/json",
    data = "<data>"
)]
pub fn post_pokemon_stats(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
    patch_description: Result<PatchDescription, PatchDescriptionError>,
    data: Result<Json<PokemonStatsRequest>, JsonError>,
    pokedex_id: u8,
) -> Result<status::Accepted<JsonValue>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let data = utils::get_data(data, BaseErrorResponseId::error_pokemon_stats_invalid)?;
    let (db, connection) = utils::get_db(&sql, &access_token)?;

    let pokemon_stats = PokemonStats {
        pokedex_id: pokedex_id,
        base_hp: data.get_base_hp(),
        base_attack: data.get_base_attack(),
        base_defence: data.get_base_defence(),
        base_speed: data.get_base_speed(),
        base_special: data.get_base_special(),
        type_ids: data.get_type_ids(),
        catch_rate: data.get_catch_rate(),
        base_exp_yield: data.get_base_exp_yield(),
        growth_rate: data.get_growth_rate(),
    };

    let patch = db.set_pokemon_stats(&pokedex_id, &pokemon_stats)?;

    utils::insert_rom_patch(
        sql,
        connection,
        access_token,
        patch,
        patch_description,
        BaseErrorResponseId::error_pokemon_stats,
    )?;

    Ok(status::Accepted(Some(json!({}))))
}
