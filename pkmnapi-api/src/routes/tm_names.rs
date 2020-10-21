use pkmnapi_sql::*;
use rocket::State;
use rocket_contrib::json::Json;

use crate::guards::*;
use crate::responses::errors::*;
use crate::responses::tm_names::*;
use crate::utils;

#[get("/tms/names")]
pub fn get_tm_name_all(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
) -> Result<Json<TMNameResponseAll>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let (min_tm_id, max_tm_id) = db.tm_id_bounds();
    let tm_ids: Vec<u8> = (min_tm_id..=max_tm_id).map(|tm_id| tm_id as u8).collect();
    let tm_names = db.get_tm_name_all(&tm_ids)?;

    let response = TMNameResponseAll::new(&tm_ids, &tm_names);

    Ok(Json(response))
}

#[get("/tms/names/<tm_id>")]
pub fn get_tm_name(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
    tm_id: u8,
) -> Result<Json<TMNameResponse>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let tm_name = db.get_tm_name(&tm_id)?;

    let response = TMNameResponse::new(&tm_id, &tm_name);

    Ok(Json(response))
}
