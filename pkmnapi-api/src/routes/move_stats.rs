use pkmnapi_db::*;
use pkmnapi_sql::*;
use rocket::response::status;
use rocket::State;
use rocket_contrib::json::{Json, JsonError, JsonValue};

use crate::guards::*;
use crate::requests::move_stats::*;
use crate::responses::errors::*;
use crate::responses::move_stats::*;
use crate::utils;

#[get("/moves/stats")]
pub fn get_move_stats_all(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
) -> Result<Json<MoveStatsResponseAll>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let (min_move_id, max_move_id) = db.move_id_bounds();
    let move_ids: Vec<u8> = (min_move_id..=max_move_id)
        .map(|move_id| move_id as u8)
        .collect();
    let move_stats = db.get_move_stats_all(&move_ids)?;
    let type_ids = move_stats
        .iter()
        .map(|(_, move_stats)| move_stats.type_id)
        .collect();
    let type_names = db.get_type_name_all(&type_ids)?;

    let response = MoveStatsResponseAll::new(&move_ids, &move_stats, &type_names);

    Ok(Json(response))
}

#[get("/moves/stats/<move_id>")]
pub fn get_move_stats(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
    move_id: u8,
) -> Result<Json<MoveStatsResponse>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let move_stats = db.get_move_stats(&move_id)?;
    let type_name = db.get_type_name(&move_stats.type_id)?;

    let response = MoveStatsResponse::new(&move_id, &move_stats, &type_name);

    Ok(Json(response))
}

#[post("/moves/stats/<move_id>", format = "application/json", data = "<data>")]
pub fn post_move_stats(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
    patch_description: Result<PatchDescription, PatchDescriptionError>,
    data: Result<Json<MoveStatsRequest>, JsonError>,
    move_id: u8,
) -> Result<status::Accepted<JsonValue>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let data = utils::get_data(data, BaseErrorResponseId::error_move_stats_invalid)?;
    let (db, connection) = utils::get_db(&sql, &access_token)?;

    let move_stats = MoveStats {
        move_id: move_id,
        effect: data.get_effect(),
        power: data.get_power(),
        type_id: data.get_type_id(),
        accuracy: data.get_accuracy(),
        pp: data.get_pp(),
    };

    let patch = db.set_move_stats(&move_id, &move_stats)?;

    utils::insert_rom_patch(
        sql,
        connection,
        access_token,
        patch,
        patch_description,
        BaseErrorResponseId::error_move_stats,
    )?;

    Ok(status::Accepted(Some(json!({}))))
}
