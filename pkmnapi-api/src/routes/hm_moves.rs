use pkmnapi_db::*;
use pkmnapi_sql::*;
use rocket::response::status;
use rocket::State;
use rocket_contrib::json::{Json, JsonError, JsonValue};

use crate::guards::*;
use crate::requests::hm_moves::*;
use crate::responses::errors::*;
use crate::responses::hm_moves::*;
use crate::utils;

#[get("/hms/moves/<hm_id>")]
pub fn get_hm_move(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
    hm_id: u8,
) -> Result<Json<HMMoveResponse>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
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

    let response = HMMoveResponse::new(&hm_id, &hm, &move_name);

    Ok(Json(response))
}

#[post("/hms/moves/<hm_id>", format = "application/json", data = "<data>")]
pub fn post_hm_move(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
    patch_description: Result<PatchDescription, PatchDescriptionError>,
    data: Result<Json<HMMoveRequest>, JsonError>,
    hm_id: u8,
) -> Result<status::Accepted<JsonValue>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let data = utils::get_data(data, BaseErrorResponseId::error_hms_invalid)?;
    let (db, connection) = utils::get_db(&sql, &access_token)?;

    let hm = HM {
        move_id: data.get_move_id(),
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

    utils::insert_rom_patch(
        sql,
        connection,
        access_token,
        patch,
        patch_description,
        BaseErrorResponseId::error_hms,
    )?;

    Ok(status::Accepted(Some(json!({}))))
}
