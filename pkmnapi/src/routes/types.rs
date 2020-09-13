use pkmnapi_db::string::*;
use pkmnapi_db::types::*;
use pkmnapi_db::*;
use pkmnapi_sql::*;
use rocket::response::status;
use rocket::State;
use rocket_contrib::json::{Json, JsonError, JsonValue};

use crate::guards::*;
use crate::requests::types::*;
use crate::responses::errors::*;
use crate::responses::types::*;
use crate::utils;

#[get("/types/<type_id>")]
pub fn get_type(
    sql: State<PkmnapiSQL>,
    access_token: Result<AccessToken, AccessTokenError>,
    type_id: u8,
) -> Result<Json<TypeResponse>, ResponseError> {
    let access_token = match access_token {
        Ok(access_token) => access_token.into_inner(),
        Err(_) => return Err(AccessTokenErrorUnauthorized::new()),
    };

    let db = utils::get_db_with_applied_patches(sql, &access_token)?;

    let type_name = match db.get_type_name(&type_id) {
        Ok(type_name) => type_name,
        Err(e) => return Err(TypeResponseError::new(&e.to_string())),
    };
    let response = TypeResponse::new(&type_id, &type_name);

    Ok(Json(response))
}

#[post("/types/<type_id>", format = "application/json", data = "<data>")]
pub fn post_type(
    sql: State<PkmnapiSQL>,
    access_token: Result<AccessToken, AccessTokenError>,
    patch_description: Result<PatchDescription, PatchDescriptionError>,
    data: Result<Json<TypeRequest>, JsonError>,
    type_id: u8,
) -> Result<status::Accepted<JsonValue>, ResponseError> {
    let access_token = match access_token {
        Ok(access_token) => access_token.into_inner(),
        Err(_) => return Err(AccessTokenErrorUnauthorized::new()),
    };

    let data = match data {
        Ok(data) => data.into_inner(),
        Err(JsonError::Parse(_, e)) => {
            return Err(TypeResponseErrorInvalid::new(&e.to_string()));
        }
        _ => {
            return Err(TypeResponseErrorInvalid::new(
                &"An unknown error occurred".to_owned(),
            ));
        }
    };

    let connection = sql.get_connection().unwrap();
    let rom_data_sql = match sql.select_user_rom_data_by_access_token(&connection, &access_token) {
        Ok(Some(rom_sql)) => rom_sql,
        _ => return Err(RomResponseErrorNoRom::new()),
    };

    let db = match PkmnapiDB::new(&rom_data_sql.data, None) {
        Ok(db) => db,
        Err(_) => return Err(RomResponseErrorInvalidRom::new()),
    };

    let type_name = TypeName {
        name: ROMString::from(data.get_name()),
    };

    let patch = match db.set_type_name(&type_id, &type_name) {
        Ok(patch) => patch,
        Err(e) => return Err(TypeResponseError::new(&e.to_string())),
    };

    let patch_description = match patch_description {
        Ok(patch_description) => patch_description.into_inner(),
        Err(_) => None,
    };

    match sql.insert_patch(
        &connection,
        &access_token,
        &patch.to_raw(),
        patch_description,
    ) {
        Ok(_) => {}
        Err(e) => return Err(TypeResponseError::new(&e.to_string())),
    };

    Ok(status::Accepted(Some(json!({}))))
}
