use pkmnapi_db::string::*;
use pkmnapi_db::*;
use pkmnapi_sql::*;
use rocket::response::status;
use rocket::State;
use rocket_contrib::json::{Json, JsonError, JsonValue};

use crate::guards::*;
use crate::requests::pokedex_entries::*;
use crate::responses::errors::*;
use crate::responses::pokedex_entries::*;
use crate::utils;

#[get("/pokedex/entries")]
pub fn get_pokedex_entry_all(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
) -> Result<Json<PokedexEntryResponseAll>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let (min_pokedex_id, max_pokedex_id) = db.pokedex_id_bounds();
    let pokedex_ids: Vec<u8> = (min_pokedex_id..=max_pokedex_id)
        .map(|pokedex_id| pokedex_id as u8)
        .collect();
    let pokedex_entries = db.get_pokedex_entry_all(&pokedex_ids)?;

    let response = PokedexEntryResponseAll::new(&pokedex_ids, &pokedex_entries);

    Ok(Json(response))
}

#[get("/pokedex/entries/<pokedex_id>")]
pub fn get_pokedex_entry(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
    pokedex_id: u8,
) -> Result<Json<PokedexEntryResponse>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let pokedex_entry = db.get_pokedex_entry(&pokedex_id)?;

    let response = PokedexEntryResponse::new(&pokedex_id, &pokedex_entry);

    Ok(Json(response))
}

#[post(
    "/pokedex/entries/<pokedex_id>",
    format = "application/json",
    data = "<data>"
)]
pub fn post_pokedex_entry(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
    patch_description: Result<PatchDescription, PatchDescriptionError>,
    data: Result<Json<PokedexEntryRequest>, JsonError>,
    pokedex_id: u8,
) -> Result<status::Accepted<JsonValue>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let data = utils::get_data(data, BaseErrorResponseId::error_pokedex_entries_invalid)?;
    let (db, connection) = utils::get_db(&sql, &access_token)?;

    let pokedex_entry = PokedexEntry {
        species: ROMString::from(data.get_species()),
        height: data.get_height(),
        weight: data.get_weight(),
    };

    let patch = db.set_pokedex_entry(&pokedex_id, &pokedex_entry)?;

    utils::insert_rom_patch(
        sql,
        connection,
        access_token,
        patch,
        patch_description,
        BaseErrorResponseId::error_pokedex_entries,
    )?;

    Ok(status::Accepted(Some(json!({}))))
}
