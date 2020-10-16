use pkmnapi_db::*;
use pkmnapi_sql::*;
use rocket::response::status;
use rocket::State;
use rocket_contrib::json::{Json, JsonError, JsonValue};

use crate::guards::*;
use crate::requests::pokemon_icons::*;
use crate::responses::errors::*;
use crate::responses::pokemon_icons::*;
use crate::utils;

#[get("/pokemon/icons")]
pub fn get_pokemon_icon_all(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
) -> Result<Json<PokemonIconResponseAll>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let (min_pokedex_id, max_pokedex_id) = db.pokedex_id_bounds();
    let pokedex_ids: Vec<u8> = (min_pokedex_id..=max_pokedex_id)
        .map(|pokedex_id| pokedex_id as u8)
        .collect();
    let pokemon_icons = db.get_pokemon_icon_all(&pokedex_ids)?;

    let response = PokemonIconResponseAll::new(&pokedex_ids, &pokemon_icons);

    Ok(Json(response))
}

#[get("/pokemon/icons/<pokedex_id>")]
pub fn get_pokemon_icon(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
    pokedex_id: u8,
) -> Result<Json<PokemonIconResponse>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let pokemon_icon = db.get_pokemon_icon(&pokedex_id)?;

    let response = PokemonIconResponse::new(&pokedex_id, &pokemon_icon);

    Ok(Json(response))
}

#[post(
    "/pokemon/icons/<pokedex_id>",
    format = "application/json",
    data = "<data>"
)]
pub fn post_pokemon_icon(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
    patch_description: Result<PatchDescription, PatchDescriptionError>,
    data: Result<Json<PokemonIconRequest>, JsonError>,
    pokedex_id: u8,
) -> Result<status::Accepted<JsonValue>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let data = utils::get_data(data, BaseErrorResponseId::error_pokemon_icons_invalid)?;
    let (db, connection) = utils::get_db(&sql, &access_token)?;

    let pokemon_icon = PokemonIcon::from(&data.get_icon_id());

    let patch = db.set_pokemon_icon(&pokedex_id, &pokemon_icon)?;

    utils::insert_rom_patch(
        sql,
        connection,
        access_token,
        patch,
        patch_description,
        BaseErrorResponseId::error_pokemon_icons,
    )?;

    Ok(status::Accepted(Some(json!({}))))
}
