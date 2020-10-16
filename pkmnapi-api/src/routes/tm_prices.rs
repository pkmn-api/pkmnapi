use pkmnapi_db::*;
use pkmnapi_sql::*;
use rocket::response::status;
use rocket::State;
use rocket_contrib::json::{Json, JsonError, JsonValue};

use crate::guards::*;
use crate::requests::tm_prices::*;
use crate::responses::errors::*;
use crate::responses::tm_prices::*;
use crate::utils;

#[get("/tms/prices")]
pub fn get_tm_price_all(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
) -> Result<Json<TMPriceResponseAll>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let (min_tm_id, max_tm_id) = db.tm_id_bounds();
    let tm_ids: Vec<u8> = (min_tm_id..=max_tm_id).map(|tm_id| tm_id as u8).collect();
    let tm_prices = db.get_tm_price_all(&tm_ids)?;

    let response = TMPriceResponseAll::new(&tm_ids, &tm_prices);

    Ok(Json(response))
}

#[get("/tms/prices/<tm_id>")]
pub fn get_tm_price(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
    tm_id: u8,
) -> Result<Json<TMPriceResponse>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let tm_price = db.get_tm_price(&tm_id)?;

    let response = TMPriceResponse::new(&tm_id, &tm_price);

    Ok(Json(response))
}

#[post("/tms/prices/<tm_id>", format = "application/json", data = "<data>")]
pub fn post_tm_price(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
    patch_description: Result<PatchDescription, PatchDescriptionError>,
    data: Result<Json<TMPriceRequest>, JsonError>,
    tm_id: u8,
) -> Result<status::Accepted<JsonValue>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let data = utils::get_data(data, BaseErrorResponseId::error_tm_prices_invalid)?;
    let (db, connection) = utils::get_db(&sql, &access_token)?;

    let tm_price = TMPrice {
        value: data.get_price(),
    };

    let patch = db.set_tm_price(&tm_id, &tm_price)?;

    utils::insert_rom_patch(
        sql,
        connection,
        access_token,
        patch,
        patch_description,
        BaseErrorResponseId::error_tm_prices,
    )?;

    Ok(status::Accepted(Some(json!({}))))
}
