use pkmnapi_db::types::*;
use pkmnapi_sql::*;
use rocket::response::status;
use rocket::State;
use rocket_contrib::json::{Json, JsonError, JsonValue};
use std::collections::HashMap;

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
    let access_token = match access_token {
        Ok(access_token) => access_token.into_inner(),
        Err(_) => return Err(AccessTokenErrorUnauthorized::new()),
    };

    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let pokemon_evolutions = match db.get_pokemon_evolutions(&pokedex_id) {
        Ok(pokemon_evolutions) => pokemon_evolutions,
        Err(e) => return Err(PokemonEvolutionsResponseError::new(&e.to_string())),
    };

    let mut pokemon_names: HashMap<u8, PokemonName> = HashMap::new();

    match pokemon_evolutions
        .iter()
        .map(|pokemon_evolution| match pokemon_evolution {
            PokemonEvolution::LEVEL(evolution) => evolution.pokedex_id,
            PokemonEvolution::ITEM(evolution) => evolution.pokedex_id,
            PokemonEvolution::TRADE(evolution) => evolution.pokedex_id,
        })
        .map(|pokedex_id| {
            let pokemon_name = match db.get_pokemon_name(&pokedex_id) {
                Ok(pokemon_name) => pokemon_name,
                Err(e) => return Err(PokemonEvolutionsResponseError::new(&e.to_string())),
            };

            pokemon_names.insert(pokedex_id, pokemon_name);

            Ok(())
        })
        .collect::<Result<Vec<_>, ResponseError>>()
    {
        Ok(_) => {}
        Err(e) => return Err(e),
    };

    let response = PokemonEvolutionsResponse::new(&pokedex_id, &pokemon_evolutions, pokemon_names);

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
    let access_token = match access_token {
        Ok(access_token) => access_token.into_inner(),
        Err(_) => return Err(AccessTokenErrorUnauthorized::new()),
    };

    let data = match data {
        Ok(data) => data.into_inner(),
        Err(JsonError::Parse(_, e)) => {
            return Err(PokemonEvolutionsResponseErrorInvalid::new(&e.to_string()));
        }
        _ => {
            return Err(PokemonEvolutionsResponseErrorInvalid::new(
                &"An unknown error occurred".to_owned(),
            ));
        }
    };

    let (db, connection) = utils::get_db(&sql, &access_token)?;

    let pokemon_evolutions = data.get_evolutions();

    let patch = match db.set_pokemon_evolutions(&pokedex_id, &pokemon_evolutions) {
        Ok(patch) => patch,
        Err(e) => return Err(PokemonEvolutionsResponseError::new(&e.to_string())),
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
        Err(e) => return Err(PokemonEvolutionsResponseError::new(&e.to_string())),
    };

    Ok(status::Accepted(Some(json!({}))))
}
