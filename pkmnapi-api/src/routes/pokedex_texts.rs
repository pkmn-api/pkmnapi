use pkmnapi_db::string::*;
use pkmnapi_db::*;
use pkmnapi_sql::*;
use rocket::response::status;
use rocket::State;
use rocket_contrib::json::{Json, JsonError, JsonValue};

use crate::guards::*;
use crate::requests::pokedex_texts::*;
use crate::responses::errors::*;
use crate::responses::pokedex_texts::*;
use crate::utils;

#[get("/pokedex/texts")]
pub fn get_pokedex_text_all(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
) -> Result<Json<PokedexTextResponseAll>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let (min_pokedex_id, max_pokedex_id) = db.pokedex_id_bounds();
    let pokedex_ids: Vec<u8> = (min_pokedex_id..=max_pokedex_id)
        .map(|pokedex_id| pokedex_id as u8)
        .collect();
    let pokedex_texts = db.get_pokedex_text_all(&pokedex_ids)?;

    let response = PokedexTextResponseAll::new(&pokedex_ids, &pokedex_texts);

    Ok(Json(response))
}

#[get("/pokedex/texts/<pokedex_id>")]
pub fn get_pokedex_text(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
    pokedex_id: u8,
) -> Result<Json<PokedexTextResponse>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let pokedex_text = db.get_pokedex_text(&pokedex_id)?;

    let response = PokedexTextResponse::new(&pokedex_id, &pokedex_text);

    Ok(Json(response))
}

#[post(
    "/pokedex/texts/<pokedex_id>",
    format = "application/json",
    data = "<data>"
)]
pub fn post_pokedex_text(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
    patch_description: Result<PatchDescription, PatchDescriptionError>,
    data: Result<Json<PokedexTextRequest>, JsonError>,
    pokedex_id: u8,
) -> Result<status::Accepted<JsonValue>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let data = utils::get_data(data, BaseErrorResponseId::error_pokedex_texts_invalid)?;
    let (db, connection) = utils::get_db(&sql, &access_token)?;

    let pokedex_text = PokedexText {
        text: ROMString::from(data.get_text()),
    };

    let patch = db.set_pokedex_text(&pokedex_id, &pokedex_text)?;

    utils::insert_rom_patch(
        sql,
        connection,
        access_token,
        patch,
        patch_description,
        BaseErrorResponseId::error_pokedex_texts,
    )?;

    Ok(status::Accepted(Some(json!({}))))
}
