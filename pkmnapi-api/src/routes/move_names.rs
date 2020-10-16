use pkmnapi_db::string::*;
use pkmnapi_db::*;
use pkmnapi_sql::*;
use rocket::response::status;
use rocket::State;
use rocket_contrib::json::{Json, JsonError, JsonValue};

use crate::guards::*;
use crate::requests::move_names::*;
use crate::responses::errors::*;
use crate::responses::move_names::*;
use crate::utils;

#[get("/moves/names")]
pub fn get_move_name_all(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
) -> Result<Json<MoveNameResponseAll>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let (min_move_id, max_move_id) = db.move_id_bounds();
    let move_ids: Vec<u8> = (min_move_id..=max_move_id)
        .map(|move_id| move_id as u8)
        .collect();
    let move_names = db.get_move_name_all(&move_ids)?;

    let response = MoveNameResponseAll::new(&move_ids, &move_names);

    Ok(Json(response))
}

#[get("/moves/names/<move_id>")]
pub fn get_move_name(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
    move_id: u8,
) -> Result<Json<MoveNameResponse>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let move_name = db.get_move_name(&move_id)?;

    let response = MoveNameResponse::new(&move_id, &move_name);

    Ok(Json(response))
}

#[post("/moves/names/<move_id>", format = "application/json", data = "<data>")]
pub fn post_move_name(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
    patch_description: Result<PatchDescription, PatchDescriptionError>,
    data: Result<Json<MoveNameRequest>, JsonError>,
    move_id: u8,
) -> Result<status::Accepted<JsonValue>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let data = utils::get_data(data, BaseErrorResponseId::error_move_names_invalid)?;
    let (db, connection) = utils::get_db(&sql, &access_token)?;

    let move_name = MoveName {
        name: ROMString::from(data.get_name()),
    };

    let patch = db.set_move_name(&move_id, &move_name)?;

    utils::insert_rom_patch(
        sql,
        connection,
        access_token,
        patch,
        patch_description,
        BaseErrorResponseId::error_move_names,
    )?;

    Ok(status::Accepted(Some(json!({}))))
}
