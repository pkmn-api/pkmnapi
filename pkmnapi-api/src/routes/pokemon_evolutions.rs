use pkmnapi_db::*;
use pkmnapi_sql::*;
use rocket::response::status;
use rocket::State;
use rocket_contrib::json::{Json, JsonError, JsonValue};

use crate::guards::*;
use crate::requests::pokemon_evolutions::*;
use crate::responses::errors::*;
use crate::responses::pokemon_evolutions::*;
use crate::utils;

#[get("/pokemon/evolutions/<pokedex_id>")]
pub fn get_pokemon_evolutions(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
    pokedex_id: u8,
) -> Result<Json<PokemonEvolutionsResponse>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let pokemon_evolutions = match db.get_pokemon_evolutions(&pokedex_id) {
        Ok(pokemon_evolutions) => pokemon_evolutions,
        Err(e) => {
            return Err(NotFoundError::new(
                BaseErrorResponseId::error_pokemon_evolutions,
                Some(e.to_string()),
            ))
        }
    };

    let pokedex_ids = pokemon_evolutions
        .iter()
        .map(|pokemon_evolution| match pokemon_evolution {
            PokemonEvolution::LEVEL(evolution) => evolution.pokedex_id,
            PokemonEvolution::ITEM(evolution) => evolution.pokedex_id,
            PokemonEvolution::TRADE(evolution) => evolution.pokedex_id,
        })
        .collect();
    let pokemon_names = utils::get_pokemon_names(
        &db,
        &pokedex_ids,
        BaseErrorResponseId::error_pokemon_evolutions,
    )?;

    let item_ids = pokemon_evolutions
        .iter()
        .filter_map(|pokemon_evolution| match pokemon_evolution {
            PokemonEvolution::LEVEL(_) => None,
            PokemonEvolution::ITEM(evolution) => Some(evolution.item_id),
            PokemonEvolution::TRADE(_) => None,
        })
        .collect();
    let item_names = utils::get_item_names(
        &db,
        &item_ids,
        BaseErrorResponseId::error_pokemon_evolutions,
    )?;

    let response =
        PokemonEvolutionsResponse::new(&pokedex_id, &pokemon_evolutions, pokemon_names, item_names);

    Ok(Json(response))
}

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

    let patch = match db.set_pokemon_evolutions(&pokedex_id, &pokemon_evolutions) {
        Ok(patch) => patch,
        Err(e) => {
            return Err(NotFoundError::new(
                BaseErrorResponseId::error_pokemon_evolutions,
                Some(e.to_string()),
            ))
        }
    };

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
