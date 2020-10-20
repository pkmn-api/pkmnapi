use pkmnapi_db::*;
use pkmnapi_sql::*;
use rocket::response::status;
use rocket::State;
use rocket_contrib::json::{Json, JsonError, JsonValue};

use crate::guards::*;
use crate::requests::trades::*;
use crate::responses::errors::*;
use crate::responses::trades::*;
use crate::utils;

#[get("/trades")]
pub fn get_trade_all(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
) -> Result<Json<TradeResponseAll>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let (min_trade_id, max_trade_id) = db.trade_id_bounds();
    let trade_ids: Vec<u8> = (min_trade_id..=max_trade_id)
        .map(|trade_id| trade_id as u8)
        .collect();
    let trades = db.get_trade_all(&trade_ids)?;
    let pokedex_ids = trades
        .iter()
        .map(|(_, trade)| vec![trade.give_pokedex_id, trade.get_pokedex_id])
        .flatten()
        .collect();
    let pokemon_names = db.get_pokemon_name_all(&pokedex_ids)?;

    let response = TradeResponseAll::new(&trade_ids, &trades, &pokemon_names);

    Ok(Json(response))
}

#[get("/trades/<trade_id>")]
pub fn get_trade(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
    trade_id: u8,
) -> Result<Json<TradeResponse>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let trade = db.get_trade(&trade_id)?;
    let pokedex_ids = vec![trade.give_pokedex_id, trade.get_pokedex_id];
    let pokemon_names = db.get_pokemon_name_all(&pokedex_ids)?;

    let response = TradeResponse::new(&trade_id, &trade, &pokemon_names);

    Ok(Json(response))
}

#[post("/trades/<trade_id>", format = "application/json", data = "<data>")]
pub fn post_trade(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
    patch_description: Result<PatchDescription, PatchDescriptionError>,
    data: Result<Json<TradeRequest>, JsonError>,
    trade_id: u8,
) -> Result<status::Accepted<JsonValue>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let data = utils::get_data(data, BaseErrorResponseId::error_trades_invalid)?;
    let (db, connection) = utils::get_db(&sql, &access_token)?;

    let trade = Trade::new(
        data.get_give_pokedex_id(),
        data.get_get_pokedex_id(),
        data.get_nickname(),
    );

    let patch = db.set_trade(&trade_id, &trade)?;

    utils::insert_rom_patch(
        sql,
        connection,
        access_token,
        patch,
        patch_description,
        BaseErrorResponseId::error_trades,
    )?;

    Ok(status::Accepted(Some(json!({}))))
}
