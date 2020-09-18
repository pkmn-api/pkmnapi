use pkmnapi_sql::*;
use rocket::http::{ContentType, Header};
use rocket::response::Response;
use rocket::State;
use std::io::Cursor;

use crate::guards::*;
use crate::responses::errors::*;
use crate::utils;

#[get("/map/pics/<map_id>", format = "image/png", rank = 1)]
pub fn get_map_pic_png<'a>(
    sql: State<PkmnapiSQL>,
    access_token: Result<AccessToken, AccessTokenError>,
    map_id: u8,
) -> Result<Response<'a>, ResponseError> {
    let access_token = match access_token {
        Ok(access_token) => access_token.into_inner(),
        Err(_) => return Err(AccessTokenErrorUnauthorized::new()),
    };

    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let map = match db.get_map_pic(&map_id) {
        Ok(map) => map,
        Err(e) => return Err(MapPicResponseError::new(&e.to_string())),
    };

    let img = match map.to_png() {
        Ok(img) => img,
        Err(e) => return Err(MapPicResponseError::new(&e.to_string())),
    };

    let response = Response::build()
        .header(ContentType::new("image", "png"))
        .header(Header::new(
            "Content-Disposition",
            format!(r#"attachment; filename="map-{:03}.png""#, map_id),
        ))
        .sized_body(Cursor::new(img))
        .finalize();

    Ok(response)
}

#[get("/map/pics/<map_id>", format = "image/jpeg", rank = 2)]
pub fn get_map_pic_jpeg<'a>(
    sql: State<PkmnapiSQL>,
    access_token: Result<AccessToken, AccessTokenError>,
    map_id: u8,
) -> Result<Response<'a>, ResponseError> {
    let access_token = match access_token {
        Ok(access_token) => access_token.into_inner(),
        Err(_) => return Err(AccessTokenErrorUnauthorized::new()),
    };

    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let map = match db.get_map_pic(&map_id) {
        Ok(map) => map,
        Err(e) => return Err(MapPicResponseError::new(&e.to_string())),
    };

    let img = match map.to_jpeg() {
        Ok(img) => img,
        Err(e) => return Err(MapPicResponseError::new(&e.to_string())),
    };

    let response = Response::build()
        .header(ContentType::new("image", "jpeg"))
        .header(Header::new(
            "Content-Disposition",
            format!(r#"attachment; filename="map-{:03}.jpg""#, map_id),
        ))
        .sized_body(Cursor::new(img))
        .finalize();

    Ok(response)
}
