use pkmnapi_db::types::*;
use pkmnapi_sql::*;
use rocket::response::status;
use rocket::State;
use rocket_contrib::json::{Json, JsonError, JsonValue};

use crate::guards::*;
use crate::requests::tm_moves::*;
use crate::responses::errors::*;
use crate::responses::tm_moves::*;
use crate::utils;

#[get("/tms/moves/<tm_id>")]
pub fn get_tm_move(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
    tm_id: u8,
) -> Result<Json<TMMoveResponse>, ResponseError> {
    let access_token = match access_token {
        Ok(access_token) => access_token.into_inner(),
        Err(_) => return Err(AccessTokenErrorUnauthorized::new()),
    };

    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let tm = match db.get_tm(&tm_id) {
        Ok(tm) => tm,
        Err(e) => {
            return Err(NotFoundError::new(
                BaseErrorResponseId::error_tm_moves,
                Some(e.to_string()),
            ))
        }
    };

    let move_name = match db.get_move_name(&tm.move_id) {
        Ok(move_name) => move_name,
        Err(e) => {
            return Err(NotFoundError::new(
                BaseErrorResponseId::error_tm_moves,
                Some(e.to_string()),
            ))
        }
    };

    let response = TMMoveResponse::new(&tm_id, &tm, &move_name);

    Ok(Json(response))
}

#[post("/tms/moves/<tm_id>", format = "application/json", data = "<data>")]
pub fn post_tm_move(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
    patch_description: Result<PatchDescription, PatchDescriptionError>,
    data: Result<Json<TMMoveRequest>, JsonError>,
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
                BaseErrorResponseId::error_tm_moves_invalid,
                Some(e.to_string()),
            ));
        }
        _ => {
            return Err(BadRequestError::new(
                BaseErrorResponseId::error_tm_moves_invalid,
                Some("An unknown error occurred".to_owned()),
            ));
        }
    };

    let (db, connection) = utils::get_db(&sql, &access_token)?;

    let tm = TM {
        move_id: data.get_move_id(),
    };

    let patch = match db.set_tm(&tm_id, &tm) {
        Ok(patch) => patch,
        Err(e) => {
            return Err(NotFoundError::new(
                BaseErrorResponseId::error_tm_moves,
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
            BaseErrorResponseId::error_tm_moves,
            Some(e.to_string()),
        ));
    }

    Ok(status::Accepted(Some(json!({}))))
}