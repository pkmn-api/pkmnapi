use pkmnapi_db::*;
use pkmnapi_sql::*;
use rocket::response::status;
use rocket::State;
use rocket_contrib::json::{Json, JsonError, JsonValue};

use crate::guards::*;
use crate::requests::mart_items::*;
use crate::responses::errors::*;
use crate::responses::mart_items::*;
use crate::utils;

#[get("/marts/items")]
pub fn get_mart_items_all(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
) -> Result<Json<MartItemsResponseAll>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let (min_mart_id, max_mart_id) = db.mart_id_bounds();
    let mart_ids: Vec<u8> = (min_mart_id..=max_mart_id)
        .map(|mart_id| mart_id as u8)
        .collect();
    let mart_items = db.get_mart_items_all(&mart_ids)?;
    let item_ids = mart_items
        .iter()
        .map(|(_, mart_item)| mart_item)
        .flatten()
        .filter_map(|mart_item| match mart_item {
            MartItem::ITEM(item_id) => Some(*item_id),
            _ => None,
        })
        .collect();
    let item_names = db.get_item_name_all(&item_ids)?;
    let tm_ids = mart_items
        .iter()
        .map(|(_, mart_item)| mart_item)
        .flatten()
        .filter_map(|mart_item| match mart_item {
            MartItem::TM(tm_id) => Some(*tm_id),
            _ => None,
        })
        .collect();
    let tm_moves = db.get_tm_move_all(&tm_ids)?;
    let move_ids = tm_moves.iter().map(|(_, tm)| tm.move_id).collect();
    let move_names = db.get_move_name_all(&move_ids)?;

    let response =
        MartItemsResponseAll::new(&mart_ids, &mart_items, &item_names, &tm_moves, &move_names);

    Ok(Json(response))
}

#[get("/marts/items/<mart_id>")]
pub fn get_mart_items(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
    mart_id: u8,
) -> Result<Json<MartItemsResponse>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let mart_items = db.get_mart_items(&mart_id)?;
    let item_ids = mart_items
        .iter()
        .filter_map(|mart_item| match mart_item {
            MartItem::ITEM(item_id) => Some(*item_id),
            _ => None,
        })
        .collect();
    let item_names = db.get_item_name_all(&item_ids)?;
    let tm_ids = mart_items
        .iter()
        .filter_map(|mart_item| match mart_item {
            MartItem::TM(tm_id) => Some(*tm_id),
            _ => None,
        })
        .collect();
    let tm_moves = db.get_tm_move_all(&tm_ids)?;
    let move_ids = tm_moves.iter().map(|(_, tm)| tm.move_id).collect();
    let move_names = db.get_move_name_all(&move_ids)?;

    let response =
        MartItemsResponse::new(&mart_id, &mart_items, &item_names, &tm_moves, &move_names);

    Ok(Json(response))
}

#[post("/marts/items/<mart_id>", format = "application/json", data = "<data>")]
pub fn post_mart_items(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
    patch_description: Result<PatchDescription, PatchDescriptionError>,
    data: Result<Json<MartItemsRequest>, JsonError>,
    mart_id: u8,
) -> Result<status::Accepted<JsonValue>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let data = utils::get_data(data, BaseErrorResponseId::error_mart_items_invalid)?;
    let (db, connection) = utils::get_db(&sql, &access_token)?;

    let mart_items = data.get_mart_items();

    let patch = db.set_mart_items(&mart_id, &mart_items)?;

    utils::insert_rom_patch(
        sql,
        connection,
        access_token,
        patch,
        patch_description,
        BaseErrorResponseId::error_mart_items,
    )?;

    Ok(status::Accepted(Some(json!({}))))
}
