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

#[get("/maps/pokemon")]
pub fn get_map_pokemon_all(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
) -> Result<Json<MapPokemonResponseAll>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let (min_map_id, max_map_id) = db.map_id_bounds();
    let map_ids = (min_map_id..=max_map_id)
        .map(|map_id| map_id as u8)
        .collect();
    let map_pokemon = db.get_map_pokemon_all(&map_ids)?;
    let pokedex_ids = map_pokemon
        .iter()
        .map(|(_, map_pokemon)| {
            vec![
                map_pokemon
                    .grass
                    .pokemon
                    .iter()
                    .map(|pokemon| pokemon.pokedex_id)
                    .collect::<Vec<u8>>(),
                map_pokemon
                    .water
                    .pokemon
                    .iter()
                    .map(|pokemon| pokemon.pokedex_id)
                    .collect::<Vec<u8>>(),
            ]
        })
        .flatten()
        .flatten()
        .collect::<Vec<u8>>();
    let pokemon_names = db.get_pokemon_name_all(&pokedex_ids)?;

    let response = MapPokemonResponseAll::new(&map_ids, &map_pokemon, &pokemon_names);

    Ok(Json(response))
}

#[get("/maps/pokemon/<map_id>")]
pub fn get_map_pokemon(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
    map_id: u8,
) -> Result<Json<MapPokemonResponse>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let map_pokemon = db.get_map_pokemon(&map_id)?;
    let pokedex_ids = vec![
        map_pokemon
            .grass
            .pokemon
            .iter()
            .map(|pokemon| pokemon.pokedex_id)
            .collect::<Vec<u8>>(),
        map_pokemon
            .water
            .pokemon
            .iter()
            .map(|pokemon| pokemon.pokedex_id)
            .collect::<Vec<u8>>(),
    ]
    .concat();
    let pokemon_names = db.get_pokemon_name_all(&pokedex_ids)?;

    let response = MapPokemonResponse::new(&map_id, &map_pokemon, &pokemon_names);

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

    let patch = db.set_map_pokemon(&map_id, &map_pokemon)?;

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
