use pkmnapi_db::sav::Sav;
use pkmnapi_sql::*;
use rocket::response::status;
use rocket::{Data, State};
use rocket_contrib::json::Json;

use crate::guards::*;
use crate::responses::errors::*;
use crate::responses::savs::*;
use crate::utils;

#[post("/savs", data = "<data>")]
pub fn post_sav(
    sql: State<PkmnapiSQL>,
    data: Data,
    access_token: Result<AccessToken, AccessTokenError>,
) -> Result<status::Created<Json<SavResponse>>, ResponseError> {
    let access_token = match access_token {
        Ok(access_token) => access_token.into_inner(),
        Err(_) => return Err(AccessTokenErrorUnauthorized::new()),
    };

    let sav = {
        let mut sav = Vec::new();

        data.stream_to(&mut sav).unwrap();

        sav
    };

    match Sav::new(&sav) {
        Ok(_) => {}
        Err(_) => return Err(SavResponseErrorInvalidSav::new()),
    };

    let connection = sql.get_connection().unwrap();
    let sav_sql = match sql.update_user_sav_by_access_token(&connection, &access_token, &sav) {
        Ok(sav_sql) => sav_sql,
        Err(_) => return Err(SavResponseErrorSavExists::new()),
    };

    let response = SavResponse::new(&sav_sql);

    Ok(status::Created(
        utils::generate_url("savs", None),
        Some(Json(response)),
    ))
}

#[get("/savs")]
pub fn get_sav(
    sql: State<PkmnapiSQL>,
    access_token: Result<AccessToken, AccessTokenError>,
) -> Result<Json<SavResponse>, ResponseError> {
    let access_token = match access_token {
        Ok(access_token) => access_token.into_inner(),
        Err(_) => return Err(AccessTokenErrorUnauthorized::new()),
    };

    let connection = sql.get_connection().unwrap();
    let sav_sql = match sql.select_user_sav_by_access_token(&connection, &access_token) {
        Ok(Some(sav_sql)) => sav_sql,
        _ => return Err(SavResponseErrorNoSav::new()),
    };

    let response = SavResponse::new(&sav_sql);

    Ok(Json(response))
}

#[delete("/savs")]
pub fn delete_sav(
    sql: State<PkmnapiSQL>,
    access_token: Result<AccessToken, AccessTokenError>,
) -> Result<status::NoContent, ResponseError> {
    let access_token = match access_token {
        Ok(access_token) => access_token.into_inner(),
        Err(_) => return Err(AccessTokenErrorUnauthorized::new()),
    };

    let connection = sql.get_connection().unwrap();
    match sql.delete_user_sav_by_access_token(&connection, &access_token) {
        Ok(_) => {}
        Err(_) => return Err(SavResponseErrorNoSav::new()),
    }

    Ok(status::NoContent)
}
