use pkmnapi_sql::*;
use rocket::http::{ContentType, Header};
use rocket::response::Response;
use rocket::State;
use rocket_contrib::json::Json;
use std::io::Cursor;

use crate::guards::*;
use crate::responses::errors::*;
use crate::responses::patches::*;

#[get("/patches", format = "application/json", rank = 1)]
pub fn get_patches(
    sql: State<PkmnapiSQL>,
    access_token: Result<AccessToken, AccessTokenError>,
) -> Result<Json<PatchesResponse>, ResponseError> {
    let access_token = match access_token {
        Ok(access_token) => access_token.into_inner(),
        Err(_) => return Err(AccessTokenErrorUnauthorized::new()),
    };

    let connection = sql.get_connection().unwrap();
    let patches = match sql.select_patches_by_access_token(&connection, &access_token) {
        Ok(patches) => patches,
        Err(_) => return Err(RomResponseErrorNoRom::new()),
    };

    let response = PatchesResponse::new(&patches);

    Ok(Json(response))
}

#[get("/patches", format = "application/patch", rank = 2)]
pub fn get_patches_raw<'a>(
    sql: State<PkmnapiSQL>,
    access_token: Result<AccessToken, AccessTokenError>,
) -> Result<Response<'a>, ResponseError> {
    let access_token = match access_token {
        Ok(access_token) => access_token.into_inner(),
        Err(_) => return Err(AccessTokenErrorUnauthorized::new()),
    };

    let connection = sql.get_connection().unwrap();
    let patches = match sql.select_patches_by_access_token(&connection, &access_token) {
        Ok(patches) => patches,
        Err(_) => return Err(RomResponseErrorNoRom::new()),
    };

    let patch: Vec<u8> = [
        "PATCH".chars().map(|c| c as u8).collect::<Vec<u8>>(),
        patches
            .iter()
            .map(|patch| patch.data.to_vec())
            .flatten()
            .collect(),
        "EOF".chars().map(|c| c as u8).collect::<Vec<u8>>(),
    ]
    .concat();

    let response = Response::build()
        .header(ContentType::new("application", "patch"))
        .header(Header::new(
            "Content-Disposition",
            r#"attachment; filename="patch.ips""#,
        ))
        .sized_body(Cursor::new(patch))
        .finalize();

    Ok(response)
}

#[get("/patches/<patch_id>")]
pub fn get_patch(
    sql: State<PkmnapiSQL>,
    access_token: Result<AccessToken, AccessTokenError>,
    patch_id: String,
) -> Result<Json<PatchResponse>, ResponseError> {
    let access_token = match access_token {
        Ok(access_token) => access_token.into_inner(),
        Err(_) => return Err(AccessTokenErrorUnauthorized::new()),
    };

    let connection = sql.get_connection().unwrap();
    let patch = match sql.select_patch_by_id(&connection, &access_token, &patch_id) {
        Ok(Some(patch)) => patch,
        Ok(None) => return Err(PatchResponseError::new()),
        Err(_) => return Err(RomResponseErrorNoRom::new()),
    };

    let response = PatchResponse::new(&patch);

    Ok(Json(response))
}
