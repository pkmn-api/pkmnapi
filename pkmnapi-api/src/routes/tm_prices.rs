use pkmnapi_db::types::*;
use pkmnapi_sql::*;
use rocket::response::status;
use rocket::State;
use rocket_contrib::json::{Json, JsonError, JsonValue};

use crate::guards::*;
use crate::requests::tm_prices::*;
use crate::responses::errors::*;
use crate::responses::tm_prices::*;
use crate::utils;

#[get("/tms/prices/<tm_id>")]
pub fn get_tm_price(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
    tm_id: u8,
) -> Result<Json<TMPriceResponse>, ResponseError> {
    let access_token = match access_token {
        Ok(access_token) => access_token.into_inner(),
        Err(_) => return Err(AccessTokenErrorUnauthorized::new()),
    };

    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let tm_price = match db.get_tm_price(&tm_id) {
        Ok(tm_price) => tm_price,
        Err(e) => {
            return Err(NotFoundError::new(
                BaseErrorResponseId::error_tm_prices,
                Some(e.to_string()),
            ))
        }
    };

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
    let access_token = match access_token {
        Ok(access_token) => access_token.into_inner(),
        Err(_) => return Err(AccessTokenErrorUnauthorized::new()),
    };

    let data = match data {
        Ok(data) => data.into_inner(),
        Err(JsonError::Parse(_, e)) => {
            return Err(BadRequestError::new(
                BaseErrorResponseId::error_tm_prices_invalid,
                Some(e.to_string()),
            ));
        }
        _ => {
            return Err(BadRequestError::new(
                BaseErrorResponseId::error_tm_prices_invalid,
                Some("An unknown error occurred".to_owned()),
            ));
        }
    };

    let (db, connection) = utils::get_db(&sql, &access_token)?;

    let tm_price = TMPrice {
        value: data.get_price(),
    };

    let patch = match db.set_tm_price(&tm_id, &tm_price) {
        Ok(patch) => patch,
        Err(e) => {
            return Err(NotFoundError::new(
                BaseErrorResponseId::error_tm_prices,
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
            BaseErrorResponseId::error_tm_prices,
            Some(e.to_string()),
        ));
    }

    Ok(status::Accepted(Some(json!({}))))
}
