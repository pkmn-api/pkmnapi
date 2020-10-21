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

#[get("/hms/moves")]
pub fn get_hm_move_all(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
) -> Result<Json<HMMoveResponseAll>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let (min_hm_id, max_hm_id) = db.hm_id_bounds();
    let hm_ids: Vec<u8> = (min_hm_id..=max_hm_id).map(|hm_id| hm_id as u8).collect();
    let hm_moves = db.get_hm_move_all(&hm_ids)?;
    let move_ids: Vec<u8> = hm_moves.iter().map(|(_, hm)| hm.move_id).collect();
    let move_names = db.get_move_name_all(&move_ids)?;

    let response = HMMoveResponseAll::new(&hm_ids, &hm_moves, &move_names);

    Ok(Json(response))
}

#[get("/hms/moves/<hm_id>")]
pub fn get_hm_move(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
    hm_id: u8,
) -> Result<Json<HMMoveResponse>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let hm_move = db.get_hm_move(&hm_id)?;
    let move_name = db.get_move_name(&hm_move.move_id)?;

    let response = HMMoveResponse::new(&hm_id, &hm_move, &move_name);

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

    let hm_move = HMMove {
        move_id: data.get_move_id(),
    };

    let patch = db.set_hm_move(&hm_id, &hm_move)?;

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
