use pkmnapi_db::types::*;
use pkmnapi_db::*;
use pkmnapi_sql::*;
use rocket::response::status;
use rocket::State;
use rocket_contrib::json::{Json, JsonError, JsonValue};

use crate::guards::*;
use crate::requests::type_effects::*;
use crate::responses::errors::*;
use crate::responses::type_effects::*;
use crate::utils;

#[get("/type_effects/<type_effect_id>")]
pub fn get_type_effect(
    sql: State<PkmnapiSQL>,
    access_token: Result<AccessToken, AccessTokenError>,
    type_effect_id: u8,
) -> Result<Json<TypeEffectResponse>, ResponseError> {
    let access_token = match access_token {
        Ok(access_token) => access_token.into_inner(),
        Err(_) => return Err(AccessTokenErrorUnauthorized::new()),
    };

    let db = utils::get_db_with_applied_patches(sql, &access_token)?;

    let type_effect = match db.get_type_effect(&type_effect_id) {
        Ok(type_effect) => type_effect,
        Err(e) => return Err(TypeEffectResponseError::new(&e.to_string())),
    };

    let attacking_type_name = match db.get_type_name(&type_effect.attacking_type_id) {
        Ok(attacking_type_name) => attacking_type_name,
        Err(e) => return Err(TypeEffectResponseError::new(&e.to_string())),
    };

    let defending_type_name = match db.get_type_name(&type_effect.defending_type_id) {
        Ok(defending_type_name) => defending_type_name,
        Err(e) => return Err(TypeEffectResponseError::new(&e.to_string())),
    };

    let response = TypeEffectResponse::new(
        &type_effect_id,
        &type_effect,
        vec![&attacking_type_name, &defending_type_name],
    );

    Ok(Json(response))
}

#[post(
    "/type_effects/<type_effect_id>",
    format = "application/json",
    data = "<data>"
)]
pub fn post_type_effect(
    sql: State<PkmnapiSQL>,
    access_token: Result<AccessToken, AccessTokenError>,
    patch_description: Result<PatchDescription, PatchDescriptionError>,
    data: Result<Json<TypeEffectRequest>, JsonError>,
    type_effect_id: u8,
) -> Result<status::Accepted<JsonValue>, ResponseError> {
    let access_token = match access_token {
        Ok(access_token) => access_token.into_inner(),
        Err(_) => return Err(AccessTokenErrorUnauthorized::new()),
    };

    let data = match data {
        Ok(data) => data.into_inner(),
        Err(JsonError::Parse(_, e)) => {
            return Err(TypeEffectResponseErrorInvalid::new(&e.to_string()));
        }
        _ => {
            return Err(TypeEffectResponseErrorInvalid::new(
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

    let type_effect = TypeEffect {
        attacking_type_id: data.get_attacking_type_id().parse::<u8>().unwrap(),
        defending_type_id: data.get_defending_type_id().parse::<u8>().unwrap(),
        multiplier: data.get_multiplier(),
    };

    let patch = match db.set_type_effect(&type_effect_id, &type_effect) {
        Ok(patch) => patch,
        Err(e) => return Err(TypeEffectResponseError::new(&e.to_string())),
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
        Err(e) => return Err(TypeEffectResponseError::new(&e.to_string())),
    };

    Ok(status::Accepted(Some(json!({}))))
}
