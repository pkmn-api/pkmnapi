use pkmnapi_sql::*;
use rocket::http::{ContentType, Header};
use rocket::response::Response;
use rocket::State;
use std::io::Cursor;

use crate::guards::*;
use crate::responses::errors::*;
use crate::utils;

#[get("/maps/pics/<map_id>", format = "image/png", rank = 1)]
pub fn get_map_pic_png<'a>(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
    map_id: u8,
) -> Result<Response<'a>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let map = db.get_map_pic(&map_id)?;
    let img = map.to_png()?;

    let response = Response::build()
        .header(ContentType::PNG)
        .header(Header::new(
            "Content-Disposition",
            format!(r#"attachment; filename="map-{}.png""#, map_id),
        ))
        .sized_body(Cursor::new(img))
        .finalize();

    Ok(response)
}

#[get("/maps/pics/<map_id>", format = "image/jpeg", rank = 2)]
pub fn get_map_pic_jpeg<'a>(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
    map_id: u8,
) -> Result<Response<'a>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let map = db.get_map_pic(&map_id)?;
    let img = map.to_jpeg()?;

    let response = Response::build()
        .header(ContentType::JPEG)
        .header(Header::new(
            "Content-Disposition",
            format!(r#"attachment; filename="map-{}.jpg""#, map_id),
        ))
        .sized_body(Cursor::new(img))
        .finalize();

    Ok(response)
}
