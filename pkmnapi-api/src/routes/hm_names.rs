use pkmnapi_sql::*;
use rocket::State;
use rocket_contrib::json::Json;
use rocket_okapi::openapi;

use crate::guards::*;
use crate::responses::errors::*;
use crate::responses::hm_names::*;
use crate::utils;

#[openapi]
#[get("/hms/names")]
pub fn get_hm_name_all(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
) -> Result<Json<HMNameResponseAll>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let (min_hm_id, max_hm_id) = db.hm_id_bounds();
    let hm_ids: Vec<u8> = (min_hm_id..=max_hm_id).map(|hm_id| hm_id as u8).collect();
    let hm_names = db.get_hm_name_all(&hm_ids)?;

    let response = HMNameResponseAll::new(&hm_ids, &hm_names);

    Ok(Json(response))
}

#[openapi]
#[get("/hms/names/<hm_id>")]
pub fn get_hm_name(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
    hm_id: u8,
) -> Result<Json<HMNameResponse>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let hm_name = db.get_hm_name(&hm_id)?;

    let response = HMNameResponse::new(&hm_id, &hm_name);

    Ok(Json(response))
}
