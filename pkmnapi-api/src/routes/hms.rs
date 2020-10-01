use pkmnapi_db::types::*;
use pkmnapi_sql::*;
use rocket::response::status;
use rocket::State;
use rocket_contrib::json::{Json, JsonError, JsonValue};

use crate::guards::*;
use crate::requests::hms::*;
use crate::responses::errors::*;
use crate::responses::hms::*;
use crate::utils;

#[get("/hms/<hm_id>")]
pub fn get_hm(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
    hm_id: u8,
) -> Result<Json<HMResponse>, ResponseError> {
    let access_token = match access_token {
        Ok(access_token) => access_token.into_inner(),
        Err(_) => return Err(AccessTokenErrorUnauthorized::new()),
    };

    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let hm = match db.get_hm(&hm_id) {
        Ok(hm) => hm,
        Err(e) => {
            return Err(NotFoundError::new(
                BaseErrorResponseId::error_hms,
                Some(e.to_string()),
            ))
        }
    };

    let move_name = match db.get_move_name(&hm.move_id) {
        Ok(move_name) => move_name,
        Err(e) => {
            return Err(NotFoundError::new(
                BaseErrorResponseId::error_hms,
                Some(e.to_string()),
            ))
        }
    };

    let response = HMResponse::new(&hm_id, &hm, &move_name);

    Ok(Json(response))
}

#[post("/hms/<hm_id>", format = "application/json", data = "<data>")]
pub fn post_hm(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
    patch_description: Result<PatchDescription, PatchDescriptionError>,
    data: Result<Json<HMRequest>, JsonError>,
    hm_id: u8,
) -> Result<status::Accepted<JsonValue>, ResponseError> {
    let access_token = match access_token {
        Ok(access_token) => access_token.into_inner(),
        Err(_) => return Err(AccessTokenErrorUnauthorized::new()),
    };

    let data = match data {
        Ok(data) => data.into_inner(),
        Err(JsonError::Parse(_, e)) => {
            return Err(BadRequestError::new(
                BaseErrorResponseId::error_hms_invalid,
                Some(e.to_string()),
            ));
        }
        _ => {
            return Err(BadRequestError::new(
                BaseErrorResponseId::error_hms_invalid,
                Some("An unknown error occurred".to_owned()),
            ));
        }
    };

    let (db, connection) = utils::get_db(&sql, &access_token)?;

    let hm = HM {
        move_id: data.get_move_id().parse::<u8>().unwrap(),
    };

    let patch = match db.set_hm(&hm_id, &hm) {
        Ok(patch) => patch,
        Err(e) => {
            return Err(NotFoundError::new(
                BaseErrorResponseId::error_hms,
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
            BaseErrorResponseId::error_hms,
            Some(e.to_string()),
        ));
    }

    Ok(status::Accepted(Some(json!({}))))
}
