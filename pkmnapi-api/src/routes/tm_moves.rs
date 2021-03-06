use pkmnapi_db::*;
use pkmnapi_sql::*;
use rocket::response::status;
use rocket::State;
use rocket_contrib::json::{Json, JsonError, JsonValue};
use rocket_okapi::openapi;

use crate::guards::*;
use crate::requests::tm_moves::*;
use crate::responses::errors::*;
use crate::responses::tm_moves::*;
use crate::utils;

#[openapi]
#[get("/tms/moves")]
pub fn get_tm_move_all(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
) -> Result<Json<TMMoveResponseAll>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let (min_tm_id, max_tm_id) = db.tm_id_bounds();
    let tm_ids: Vec<u8> = (min_tm_id..=max_tm_id).map(|tm_id| tm_id as u8).collect();
    let tm_moves = db.get_tm_move_all(&tm_ids)?;
    let move_ids = tm_moves.iter().map(|(_, tm)| tm.move_id).collect();
    let move_names = db.get_move_name_all(&move_ids)?;

    let response = TMMoveResponseAll::new(&tm_ids, &tm_moves, &move_names);

    Ok(Json(response))
}

#[openapi]
#[get("/tms/moves/<tm_id>")]
pub fn get_tm_move(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
    tm_id: u8,
) -> Result<Json<TMMoveResponse>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let tm_move = db.get_tm_move(&tm_id)?;
    let move_name = db.get_move_name(&tm_move.move_id)?;

    let response = TMMoveResponse::new(&tm_id, &tm_move, &move_name);

    Ok(Json(response))
}

#[openapi]
#[post("/tms/moves/<tm_id>", format = "application/json", data = "<data>")]
pub fn post_tm_move(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
    patch_description: Result<PatchDescription, PatchDescriptionError>,
    data: Result<Json<TMMoveRequest>, JsonError>,
    tm_id: u8,
) -> Result<status::Accepted<JsonValue>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let data = utils::get_data(data, BaseErrorResponseId::error_tm_moves_invalid)?;
    let (db, connection) = utils::get_db(&sql, &access_token)?;

    let tm_move = TMMove {
        move_id: data.get_move_id(),
    };

    let patch = db.set_tm_move(&tm_id, &tm_move)?;

    utils::insert_rom_patch(
        sql,
        connection,
        access_token,
        patch,
        patch_description,
        BaseErrorResponseId::error_tm_moves,
    )?;

    Ok(status::Accepted(Some(json!({}))))
}
