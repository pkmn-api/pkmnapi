use pkmnapi_db::types::*;
use pkmnapi_sql::*;
use rocket::response::status;
use rocket::State;
use rocket_contrib::json::{Json, JsonError, JsonValue};
use std::collections::HashMap;

use crate::guards::*;
use crate::requests::map_pokemon::*;
use crate::responses::errors::*;
use crate::responses::map_pokemon::*;
use crate::utils;

#[get("/maps/pokemon/<map_id>")]
pub fn get_map_pokemon(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
    map_id: u8,
) -> Result<Json<MapPokemonResponse>, ResponseError> {
    let access_token = match access_token {
        Ok(access_token) => access_token.into_inner(),
        Err(_) => return Err(AccessTokenErrorUnauthorized::new()),
    };

    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let map_pokemon = match db.get_map_pokemon(&map_id) {
        Ok(map_pokemon) => map_pokemon,
        Err(e) => {
            return Err(NotFoundError::new(
                BaseErrorResponseId::error_map_pokemon,
                Some(e.to_string()),
            ))
        }
    };

    let mut pokemon_names: HashMap<u8, PokemonName> = HashMap::new();

    if let Err(e) = map_pokemon
        .grass
        .pokemon
        .iter()
        .map(|pokemon| {
            let pokemon_name = match db.get_pokemon_name(&pokemon.pokedex_id) {
                Ok(pokemon_name) => pokemon_name,
                Err(e) => {
                    return Err(NotFoundError::new(
                        BaseErrorResponseId::error_map_pokemon,
                        Some(e.to_string()),
                    ))
                }
            };

            pokemon_names.insert(pokemon.pokedex_id, pokemon_name);

            Ok(())
        })
        .collect::<Result<Vec<_>, ResponseError>>()
    {
        return Err(e);
    }

    let response = MapPokemonResponse::new(&map_id, &map_pokemon, pokemon_names);

    Ok(Json(response))
}

#[post("/maps/pokemon/<map_id>", format = "application/json", data = "<data>")]
pub fn post_map_pokemon(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
    patch_description: Result<PatchDescription, PatchDescriptionError>,
    data: Result<Json<MapPokemonRequest>, JsonError>,
    map_id: u8,
) -> Result<status::Accepted<JsonValue>, ResponseError> {
    let access_token = match access_token {
        Ok(access_token) => access_token.into_inner(),
        Err(_) => return Err(AccessTokenErrorUnauthorized::new()),
    };

    let data = match data {
        Ok(data) => data.into_inner(),
        Err(JsonError::Parse(_, e)) => {
            return Err(BadRequestError::new(
                BaseErrorResponseId::error_map_pokemon_invalid,
                Some(e.to_string()),
            ));
        }
        _ => {
            return Err(BadRequestError::new(
                BaseErrorResponseId::error_map_pokemon_invalid,
                Some("An unknown error occurred".to_owned()),
            ));
        }
    };

    let (db, connection) = utils::get_db(&sql, &access_token)?;

    let map_pokemon = MapPokemon {
        grass: data.get_grass(),
        water: data.get_water(),
    };

    let patch = match db.set_map_pokemon(&map_id, &map_pokemon) {
        Ok(patch) => patch,
        Err(e) => {
            return Err(NotFoundError::new(
                BaseErrorResponseId::error_map_pokemon,
                Some(e.to_string()),
            ))
        }
    };

    let patch_description = match patch_description {
        Ok(patch_description) => patch_description.into_inner(),
        Err(_) => None,
    };

    if let Err(e) = sql.insert_rom_patch(
        &connection,
        &access_token,
        &patch.to_raw(),
        patch_description,
    ) {
        return Err(NotFoundError::new(
            BaseErrorResponseId::error_map_pokemon,
            Some(e.to_string()),
        ));
    }

    Ok(status::Accepted(Some(json!({}))))
}
