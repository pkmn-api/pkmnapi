use pkmnapi_db::string::*;
use pkmnapi_db::types::*;
use pkmnapi_sql::*;
use rocket::response::status;
use rocket::State;
use rocket_contrib::json::{Json, JsonError, JsonValue};

use crate::guards::*;
use crate::requests::item_names::*;
use crate::responses::errors::*;
use crate::responses::item_names::*;
use crate::utils;

#[get("/items/names/<item_id>")]
pub fn get_item_name(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
    item_id: u8,
) -> Result<Json<ItemNameResponse>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let item_name = match db.get_item_name(&item_id) {
        Ok(item_name) => item_name,
        Err(e) => {
            return Err(NotFoundError::new(
                BaseErrorResponseId::error_item_names,
                Some(e.to_string()),
            ))
        }
    };
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

    let patch = match db.set_item_name(&item_id, &item_name) {
        Ok(patch) => patch,
        Err(e) => {
            return Err(NotFoundError::new(
                BaseErrorResponseId::error_item_names,
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
        BaseErrorResponseId::error_item_names,
    )?;

    Ok(status::Accepted(Some(json!({}))))
}
