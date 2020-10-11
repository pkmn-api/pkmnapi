use pkmnapi_db::types::*;
use pkmnapi_sql::*;
use rocket::response::status;
use rocket::State;
use rocket_contrib::json::{Json, JsonError, JsonValue};

use crate::guards::*;
use crate::requests::type_effects::*;
use crate::responses::errors::*;
use crate::responses::type_effects::*;
use crate::utils;

#[get("/types/effects/<type_effect_id>")]
pub fn get_type_effect(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
    type_effect_id: u8,
) -> Result<Json<TypeEffectResponse>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let type_effect = match db.get_type_effect(&type_effect_id) {
        Ok(type_effect) => type_effect,
        Err(e) => {
            return Err(NotFoundError::new(
                BaseErrorResponseId::error_type_effects,
                Some(e.to_string()),
            ))
        }
    };

    let attacking_type_name = match db.get_type_name(&type_effect.attacking_type_id) {
        Ok(attacking_type_name) => attacking_type_name,
        Err(e) => {
            return Err(NotFoundError::new(
                BaseErrorResponseId::error_type_effects,
                Some(e.to_string()),
            ))
        }
    };

    let defending_type_name = match db.get_type_name(&type_effect.defending_type_id) {
        Ok(defending_type_name) => defending_type_name,
        Err(e) => {
            return Err(NotFoundError::new(
                BaseErrorResponseId::error_type_effects,
                Some(e.to_string()),
            ))
        }
    };

    let response = TypeEffectResponse::new(
        &type_effect_id,
        &type_effect,
        vec![&attacking_type_name, &defending_type_name],
    );

    Ok(Json(response))
}

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

    let patch = match db.set_type_effect(&type_effect_id, &type_effect) {
        Ok(patch) => patch,
        Err(e) => {
            return Err(NotFoundError::new(
                BaseErrorResponseId::error_type_effects,
                Some(e.to_string()),
            ))
        }
    };

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
