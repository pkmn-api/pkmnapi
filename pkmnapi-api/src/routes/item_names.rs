use pkmnapi_db::string::*;
use pkmnapi_db::*;
use pkmnapi_sql::*;
use rocket::response::status;
use rocket::State;
use rocket_contrib::json::{Json, JsonError, JsonValue};

use crate::guards::*;
use crate::requests::item_names::*;
use crate::responses::errors::*;
use crate::responses::item_names::*;
use crate::utils;

#[get("/items/names")]
pub fn get_item_name_all(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
) -> Result<Json<ItemNameResponseAll>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let (min_item_id, max_item_id) = db.item_id_bounds();
    let item_ids: Vec<u8> = (min_item_id..=max_item_id)
        .map(|item_id| item_id as u8)
        .collect();
    let item_names = db.get_item_name_all(&item_ids)?;

    let response = ItemNameResponseAll::new(&item_ids, &item_names);

    Ok(Json(response))
}

#[get("/items/names/<item_id>")]
pub fn get_item_name(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
    item_id: u8,
) -> Result<Json<ItemNameResponse>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let item_name = db.get_item_name(&item_id)?;

    let response = ItemNameResponse::new(&item_id, &item_name);

    Ok(Json(response))
}

#[post("/items/names/<item_id>", format = "application/json", data = "<data>")]
pub fn post_item_name(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
    patch_description: Result<PatchDescription, PatchDescriptionError>,
    data: Result<Json<ItemNameRequest>, JsonError>,
    item_id: u8,
) -> Result<status::Accepted<JsonValue>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let data = utils::get_data(data, BaseErrorResponseId::error_item_names_invalid)?;
    let (db, connection) = utils::get_db(&sql, &access_token)?;

    let item_name = ItemName {
        name: ROMString::from(data.get_name()),
    };

    let patch = db.set_item_name(&item_id, &item_name)?;

    utils::insert_rom_patch(
        sql,
        connection,
        access_token,
        patch,
        patch_description,
        BaseErrorResponseId::error_item_names,
    )?;

    Ok(status::Accepted(Some(json!({}))))
}
