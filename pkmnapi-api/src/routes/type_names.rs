use pkmnapi_db::string::*;
use pkmnapi_db::*;
use pkmnapi_sql::*;
use rocket::response::status;
use rocket::State;
use rocket_contrib::json::{Json, JsonError, JsonValue};

use crate::guards::*;
use crate::requests::type_names::*;
use crate::responses::errors::*;
use crate::responses::type_names::*;
use crate::utils;

#[get("/types/names")]
pub fn get_type_name_all(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
) -> Result<Json<TypeNameResponseAll>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let (min_type_id, max_type_id) = db.type_id_bounds();
    let type_ids: Vec<u8> = (min_type_id..=max_type_id)
        .map(|type_id| type_id as u8)
        .collect();
    let type_names = db.get_type_name_all(&type_ids)?;

    let response = TypeNameResponseAll::new(&type_ids, &type_names);

    Ok(Json(response))
}

#[get("/types/names/<type_id>")]
pub fn get_type_name(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
    type_id: u8,
) -> Result<Json<TypeNameResponse>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let type_name = db.get_type_name(&type_id)?;

    let response = TypeNameResponse::new(&type_id, &type_name);

    Ok(Json(response))
}

#[post("/types/names/<type_id>", format = "application/json", data = "<data>")]
pub fn post_type_name(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
    patch_description: Result<PatchDescription, PatchDescriptionError>,
    data: Result<Json<TypeNameRequest>, JsonError>,
    type_id: u8,
) -> Result<status::Accepted<JsonValue>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let data = utils::get_data(data, BaseErrorResponseId::error_type_names_invalid)?;
    let (db, connection) = utils::get_db(&sql, &access_token)?;

    let type_name = TypeName {
        name: ROMString::from(data.get_name()),
    };

    let patch = db.set_type_name(&type_id, &type_name)?;

    utils::insert_rom_patch(
        sql,
        connection,
        access_token,
        patch,
        patch_description,
        BaseErrorResponseId::error_type_names,
    )?;

    Ok(status::Accepted(Some(json!({}))))
}
