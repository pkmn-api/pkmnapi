use pkmnapi_db::*;
use pkmnapi_sql::*;
use rocket::response::status;
use rocket::State;
use rocket_contrib::json::{Json, JsonError, JsonValue};
use rocket_okapi::openapi;

use crate::guards::*;
use crate::requests::type_effects::*;
use crate::responses::errors::*;
use crate::responses::type_effects::*;
use crate::utils;

#[openapi]
#[get("/types/effects")]
pub fn get_type_effect_all(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
) -> Result<Json<TypeEffectResponseAll>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let (min_type_effect_id, max_type_effect_id) = db.type_effect_id_bounds();
    let type_effect_ids: Vec<u8> = (min_type_effect_id..=max_type_effect_id)
        .map(|type_effect_id| type_effect_id as u8)
        .collect();
    let type_effects = db.get_type_effect_all(&type_effect_ids)?;
    let type_ids = type_effects
        .iter()
        .map(|(_, type_effect)| vec![type_effect.attacking_type_id, type_effect.defending_type_id])
        .flatten()
        .collect();
    let type_names = db.get_type_name_all(&type_ids)?;

    let response = TypeEffectResponseAll::new(&type_effect_ids, &type_effects, &type_names);

    Ok(Json(response))
}

#[openapi]
#[get("/types/effects/<type_effect_id>")]
pub fn get_type_effect(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
    type_effect_id: u8,
) -> Result<Json<TypeEffectResponse>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let type_effect = db.get_type_effect(&type_effect_id)?;
    let type_ids = vec![type_effect.attacking_type_id, type_effect.defending_type_id];
    let type_names = db.get_type_name_all(&type_ids)?;

    let response = TypeEffectResponse::new(&type_effect_id, &type_effect, &type_names);

    Ok(Json(response))
}

#[openapi]
#[post(
    "/types/effects/<type_effect_id>",
    format = "application/json",
    data = "<data>"
)]
pub fn post_type_effect(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
    patch_description: Result<PatchDescription, PatchDescriptionError>,
    data: Result<Json<TypeEffectRequest>, JsonError>,
    type_effect_id: u8,
) -> Result<status::Accepted<JsonValue>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let data = utils::get_data(data, BaseErrorResponseId::error_type_effects_invalid)?;
    let (db, connection) = utils::get_db(&sql, &access_token)?;

    let type_effect = TypeEffect {
        attacking_type_id: data.get_attacking_type_id(),
        defending_type_id: data.get_defending_type_id(),
        multiplier: data.get_multiplier(),
    };

    let patch = db.set_type_effect(&type_effect_id, &type_effect)?;

    utils::insert_rom_patch(
        sql,
        connection,
        access_token,
        patch,
        patch_description,
        BaseErrorResponseId::error_type_effects,
    )?;

    Ok(status::Accepted(Some(json!({}))))
}
