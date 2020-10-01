use pkmnapi_db::sav::Sav;
use pkmnapi_sql::*;
use rocket::http::{ContentType, Header, Status};
use rocket::response::status;
use rocket::response::Response;
use rocket::{Data, State};
use std::io::Cursor;

use crate::guards::*;
use crate::responses::errors::*;
use crate::responses::savs::*;
use crate::utils;

#[post("/savs", data = "<data>")]
pub fn post_sav<'a>(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    data: Data,
    access_token: Result<AccessToken, AccessTokenError>,
) -> Result<Response<'a>, ResponseError> {
    let access_token = match access_token {
        Ok(access_token) => access_token.into_inner(),
        Err(_) => return Err(AccessTokenErrorUnauthorized::new()),
    };

    let sav = {
        let mut sav = Vec::new();

        data.stream_to(&mut sav).unwrap();

        sav
    };

    if let Err(_) = Sav::new(&sav) {
        return Err(SavErrorInvalidSav::new());
    }

    let connection = sql.get_connection().unwrap();
    let sav_sql = match sql.update_user_sav_by_access_token(&connection, &access_token, &sav) {
        Ok(sav_sql) => sav_sql,
        Err(_) => return Err(SavErrorSavExists::new()),
    };

    let response = SavResponse::new(&sav_sql);
    let body = serde_json::to_string(&response).unwrap();

    let response = Response::build()
        .status(Status::Created)
        .header(ContentType::JSON)
        .header(Header::new("Location", utils::generate_url("savs", None)))
        .header(Header::new("ETag", sav_sql.etag))
        .sized_body(Cursor::new(body))
        .finalize();

    Ok(response)
}

#[get("/savs")]
pub fn get_sav<'a>(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
) -> Result<Response<'a>, ResponseError> {
    let access_token = match access_token {
        Ok(access_token) => access_token.into_inner(),
        Err(_) => return Err(AccessTokenErrorUnauthorized::new()),
    };

    let connection = sql.get_connection().unwrap();
    let sav_sql = match sql.select_user_sav_by_access_token(&connection, &access_token) {
        Ok(Some(sav_sql)) => sav_sql,
        _ => return Err(SavErrorNoSav::new()),
    };

    let response = SavResponse::new(&sav_sql);
    let body = serde_json::to_string(&response).unwrap();

    let response = Response::build()
        .header(ContentType::JSON)
        .header(Header::new("ETag", sav_sql.etag))
        .sized_body(Cursor::new(body))
        .finalize();

    Ok(response)
}

#[delete("/savs")]
pub fn delete_sav(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
    if_match: Result<IfMatch, IfMatchError>,
) -> Result<status::NoContent, ResponseError> {
    let access_token = match access_token {
        Ok(access_token) => access_token.into_inner(),
        Err(_) => return Err(AccessTokenErrorUnauthorized::new()),
    };

    let etag = match if_match {
        Ok(if_match) => if_match.into_inner(),
        Err(_) => return Err(ETagErrorMissing::new()),
    };

    let connection = sql.get_connection().unwrap();

    match sql.delete_user_sav_by_access_token(&connection, &access_token, &etag) {
        Ok(_) => {}
        Err(pkmnapi_sql::error::Error::ETagError) => return Err(ETagErrorMismatch::new()),
        Err(_) => return Err(SavErrorNoSav::new()),
    }

    Ok(status::NoContent)
}
