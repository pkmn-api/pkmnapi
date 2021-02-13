use pkmnapi_db::*;
use pkmnapi_sql::*;
use rocket::response::status;
use rocket::State;
use rocket_contrib::json::{Json, JsonError, JsonValue};
use rocket_okapi::openapi;

use crate::guards::*;
use crate::requests::pokemon_evolutions::*;
use crate::responses::errors::*;
use crate::responses::pokemon_evolutions::*;
use crate::utils;

#[openapi]
#[get("/pokemon/evolutions")]
pub fn get_pokemon_evolutions_all(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
) -> Result<Json<PokemonEvolutionsResponseAll>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let (min_pokedex_id, max_pokedex_id) = db.pokedex_id_bounds();
    let pokedex_ids: Vec<u8> = (min_pokedex_id..=max_pokedex_id)
        .map(|pokedex_id| pokedex_id as u8)
        .collect();
    let pokemon_evolutions = db.get_pokemon_evolutions_all(&pokedex_ids)?;
    let pokemon_evolutions_pokedex_ids = pokemon_evolutions
        .iter()
        .map(|(_, pokemon_evolution)| pokemon_evolution)
        .flatten()
        .map(|pokemon_evolution| match pokemon_evolution {
            PokemonEvolution::LEVEL(evolution) => evolution.pokedex_id,
            PokemonEvolution::ITEM(evolution) => evolution.pokedex_id,
            PokemonEvolution::TRADE(evolution) => evolution.pokedex_id,
        })
        .collect();
    let pokemon_names = db.get_pokemon_name_all(&pokemon_evolutions_pokedex_ids)?;
    let item_ids = pokemon_evolutions
        .iter()
        .map(|(_, pokemon_evolution)| pokemon_evolution)
        .flatten()
        .filter_map(|pokemon_evolution| match pokemon_evolution {
            PokemonEvolution::LEVEL(_) => None,
            PokemonEvolution::ITEM(evolution) => Some(evolution.item_id),
            PokemonEvolution::TRADE(_) => None,
        })
        .collect();
    let item_names = db.get_item_name_all(&item_ids)?;

    let response = PokemonEvolutionsResponseAll::new(
        &pokedex_ids,
        &pokemon_evolutions,
        &pokemon_names,
        &item_names,
    );

    Ok(Json(response))
}

#[openapi]
#[get("/pokemon/evolutions/<pokedex_id>")]
pub fn get_pokemon_evolutions(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
    pokedex_id: u8,
) -> Result<Json<PokemonEvolutionsResponse>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let pokemon_evolutions = db.get_pokemon_evolutions(&pokedex_id)?;
    let pokedex_ids = pokemon_evolutions
        .iter()
        .map(|pokemon_evolution| match pokemon_evolution {
            PokemonEvolution::LEVEL(evolution) => evolution.pokedex_id,
            PokemonEvolution::ITEM(evolution) => evolution.pokedex_id,
            PokemonEvolution::TRADE(evolution) => evolution.pokedex_id,
        })
        .collect();
    let pokemon_names = db.get_pokemon_name_all(&pokedex_ids)?;
    let item_ids = pokemon_evolutions
        .iter()
        .filter_map(|pokemon_evolution| match pokemon_evolution {
            PokemonEvolution::LEVEL(_) => None,
            PokemonEvolution::ITEM(evolution) => Some(evolution.item_id),
            PokemonEvolution::TRADE(_) => None,
        })
        .collect();
    let item_names = db.get_item_name_all(&item_ids)?;

    let response = PokemonEvolutionsResponse::new(
        &pokedex_id,
        &pokemon_evolutions,
        &pokemon_names,
        &item_names,
    );

    Ok(Json(response))
}

#[openapi]
#[post(
    "/pokemon/evolutions/<pokedex_id>",
    format = "application/json",
    data = "<data>"
)]
pub fn post_pokemon_evolutions(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
    patch_description: Result<PatchDescription, PatchDescriptionError>,
    data: Result<Json<PokemonEvolutionsRequest>, JsonError>,
    pokedex_id: u8,
) -> Result<status::Accepted<JsonValue>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let data = utils::get_data(data, BaseErrorResponseId::error_pokemon_evolutions_invalid)?;
    let (db, connection) = utils::get_db(&sql, &access_token)?;

    let pokemon_evolutions = data.get_evolutions();

    let patch = db.set_pokemon_evolutions(&pokedex_id, &pokemon_evolutions)?;

    utils::insert_rom_patch(
        sql,
        connection,
        access_token,
        patch,
        patch_description,
        BaseErrorResponseId::error_pokemon_evolutions,
    )?;

    Ok(status::Accepted(Some(json!({}))))
}
