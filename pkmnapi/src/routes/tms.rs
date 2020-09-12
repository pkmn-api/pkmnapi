use pkmnapi_db::types::*;
use pkmnapi_db::*;
use pkmnapi_sql::*;
use rocket::response::status;
use rocket::State;
use rocket_contrib::json::{Json, JsonError, JsonValue};

use crate::guards::*;
use crate::requests::tms::*;
use crate::responses::errors::*;
use crate::responses::tms::*;
use crate::utils;

#[get("/tms/<tm_id>")]
pub fn get_tm(
    sql: State<PkmnapiSQL>,
    access_token: Result<AccessToken, AccessTokenError>,
    tm_id: u8,
) -> Result<Json<TMResponse>, ResponseError> {
    let access_token = match access_token {
        Ok(access_token) => access_token.into_inner(),
        Err(_) => return Err(AccessTokenErrorUnauthorized::new()),
    };

    let db = utils::get_db_with_applied_patches(sql, &access_token)?;

    let tm = match db.get_tm(&tm_id) {
        Ok(tm) => tm,
        Err(e) => return Err(TMResponseError::new(&e.to_string())),
    };

    let move_name = match db.get_move_name(&tm.move_id) {
        Ok(move_name) => move_name,
        Err(e) => return Err(TMResponseError::new(&e.to_string())),
    };

    let response = TMResponse::new(&tm_id, &tm, &move_name);

    Ok(Json(response))
}

#[post("/tms/<tm_id>", format = "application/json", data = "<data>")]
pub fn post_tm(
    sql: State<PkmnapiSQL>,
    access_token: Result<AccessToken, AccessTokenError>,
    patch_description: Result<PatchDescription, PatchDescriptionError>,
    data: Result<Json<TMRequest>, JsonError>,
    tm_id: u8,
) -> Result<status::Accepted<JsonValue>, ResponseError> {
    let access_token = match access_token {
        Ok(access_token) => access_token.into_inner(),
        Err(_) => return Err(AccessTokenErrorUnauthorized::new()),
    };

    let data = match data {
        Ok(data) => data.into_inner(),
        Err(JsonError::Parse(_, e)) => {
            return Err(TMResponseErrorInvalid::new(&e.to_string()));
        }
        _ => {
            return Err(TMResponseErrorInvalid::new(
                &"An unknown error occurred".to_owned(),
            ));
        }
    };

    let connection = sql.get_connection().unwrap();
    let rom_data_sql = match sql.select_user_rom_data_by_access_token(&connection, &access_token) {
        Ok(Some(rom_sql)) => rom_sql,
        _ => return Err(RomResponseErrorNoRom::new()),
    };

    let db = match PkmnapiDB::new(&rom_data_sql.data) {
        Ok(db) => db,
        Err(_) => return Err(RomResponseErrorInvalidRom::new()),
    };

    let tm = TM {
        move_id: data.get_move_id().parse::<u8>().unwrap(),
    };

    let patch = match db.set_tm(&tm_id, &tm) {
        Ok(patch) => patch,
        Err(e) => return Err(TMResponseError::new(&e.to_string())),
    };

    let patch_description = match patch_description {
        Ok(patch_description) => patch_description.into_inner(),
        Err(_) => None,
    };

    match sql.insert_patch(
        &connection,
        &access_token,
        &patch.to_raw(),
        patch_description,
    ) {
        Ok(_) => {}
        Err(e) => return Err(TMResponseError::new(&e.to_string())),
    };

    Ok(status::Accepted(Some(json!({}))))
}
