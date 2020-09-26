use pkmnapi_db::types::*;
use pkmnapi_sql::*;
use rocket::response::status;
use rocket::State;
use rocket_contrib::json::{Json, JsonError, JsonValue};

use crate::guards::*;
use crate::requests::pokemon_stats::*;
use crate::responses::errors::*;
use crate::responses::pokemon_stats::*;
use crate::utils;

#[get("/pokemon/stats/<pokedex_id>")]
pub fn get_pokemon_stats(
    sql: State<PkmnapiSQL>,
    access_token: Result<AccessToken, AccessTokenError>,
    pokedex_id: u8,
) -> Result<Json<PokemonStatsResponse>, ResponseError> {
    let access_token = match access_token {
        Ok(access_token) => access_token.into_inner(),
        Err(_) => return Err(AccessTokenErrorUnauthorized::new()),
    };

    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let pokemon_stats = match db.get_pokemon_stats(&pokedex_id) {
        Ok(pokemon_stats) => pokemon_stats,
        Err(e) => return Err(PokemonStatsResponseError::new(&e.to_string())),
    };

    let type_names: Result<Vec<TypeName>, _> = pokemon_stats
        .type_ids
        .iter()
        .map(|type_id| match db.get_type_name(type_id) {
            Ok(type_name) => Ok(type_name),
            Err(e) => return Err(PokemonStatsResponseError::new(&e.to_string())),
        })
        .collect();

    let type_names = match type_names {
        Ok(type_names) => type_names,
        Err(e) => return Err(e),
    };

    let response = PokemonStatsResponse::new(&pokedex_id, &pokemon_stats, type_names);

    Ok(Json(response))
}

#[post(
    "/pokemon/stats/<pokedex_id>",
    format = "application/json",
    data = "<data>"
)]
pub fn post_pokemon_stats(
    sql: State<PkmnapiSQL>,
    access_token: Result<AccessToken, AccessTokenError>,
    patch_description: Result<PatchDescription, PatchDescriptionError>,
    data: Result<Json<PokemonStatsRequest>, JsonError>,
    pokedex_id: u8,
) -> Result<status::Accepted<JsonValue>, ResponseError> {
    let access_token = match access_token {
        Ok(access_token) => access_token.into_inner(),
        Err(_) => return Err(AccessTokenErrorUnauthorized::new()),
    };

    let data = match data {
        Ok(data) => data.into_inner(),
        Err(JsonError::Parse(_, e)) => {
            return Err(PokemonStatsResponseErrorInvalid::new(&e.to_string()));
        }
        _ => {
            return Err(PokemonStatsResponseErrorInvalid::new(
                &"An unknown error occurred".to_owned(),
            ));
        }
    };

    let (db, connection) = utils::get_db(&sql, &access_token)?;

    let pokemon_stats = PokemonStats {
        pokedex_id: pokedex_id,
        base_hp: data.get_base_hp(),
        base_attack: data.get_base_attack(),
        base_defence: data.get_base_defence(),
        base_speed: data.get_base_speed(),
        base_special: data.get_base_special(),
        type_ids: data
            .get_type_ids()
            .iter()
            .map(|type_id| type_id.parse::<u8>().unwrap())
            .collect(),
        catch_rate: data.get_catch_rate(),
        base_exp_yield: data.get_base_exp_yield(),
    };

    let patch = match db.set_pokemon_stats(&pokedex_id, &pokemon_stats) {
        Ok(patch) => patch,
        Err(e) => return Err(PokemonStatsResponseError::new(&e.to_string())),
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
        Err(e) => return Err(PokemonStatsResponseError::new(&e.to_string())),
    };

    Ok(status::Accepted(Some(json!({}))))
}
