use pkmnapi_db::*;
use pkmnapi_sql::*;
use rocket::response::status;
use rocket::State;
use rocket_contrib::json::{Json, JsonError, JsonValue};

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
    let access_token = utils::get_access_token(access_token)?;
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

    let pokedex_ids = map_pokemon
        .grass
        .pokemon
        .iter()
        .map(|pokemon| pokemon.pokedex_id)
        .collect();
    let pokemon_names =
        utils::get_pokemon_names(&db, &pokedex_ids, BaseErrorResponseId::error_map_pokemon)?;

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
    let access_token = utils::get_access_token(access_token)?;
    let data = utils::get_data(data, BaseErrorResponseId::error_map_pokemon_invalid)?;
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

    utils::insert_rom_patch(
        sql,
        connection,
        access_token,
        patch,
        patch_description,
        BaseErrorResponseId::error_map_pokemon,
    )?;

    Ok(status::Accepted(Some(json!({}))))
}
