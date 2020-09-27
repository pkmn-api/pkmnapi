use pkmnapi_db::string::*;
use pkmnapi_db::types::*;
use pkmnapi_sql::*;
use rocket::response::status;
use rocket::State;
use rocket_contrib::json::{Json, JsonError, JsonValue};

use crate::guards::*;
use crate::requests::trainer_names::*;
use crate::responses::errors::*;
use crate::responses::trainer_names::*;
use crate::utils;

#[get("/trainers/names/<trainer_id>")]
pub fn get_trainer_name(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
    trainer_id: u8,
) -> Result<Json<TrainerNameResponse>, ResponseError> {
    let access_token = match access_token {
        Ok(access_token) => access_token.into_inner(),
        Err(_) => return Err(AccessTokenErrorUnauthorized::new()),
    };

    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let trainer_name = match db.get_trainer_name(&trainer_id) {
        Ok(trainer_name) => trainer_name,
        Err(e) => return Err(TrainerNameResponseError::new(&e.to_string())),
    };
    let response = TrainerNameResponse::new(&trainer_id, &trainer_name);

    Ok(Json(response))
}

#[post(
    "/trainers/names/<trainer_id>",
    format = "application/json",
    data = "<data>"
)]
pub fn post_trainer_name(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
    patch_description: Result<PatchDescription, PatchDescriptionError>,
    data: Result<Json<TrainerNameRequest>, JsonError>,
    trainer_id: u8,
) -> Result<status::Accepted<JsonValue>, ResponseError> {
    let access_token = match access_token {
        Ok(access_token) => access_token.into_inner(),
        Err(_) => return Err(AccessTokenErrorUnauthorized::new()),
    };

    let data = match data {
        Ok(data) => data.into_inner(),
        Err(JsonError::Parse(_, e)) => {
            return Err(TrainerNameResponseErrorInvalid::new(&e.to_string()));
        }
        _ => {
            return Err(TrainerNameResponseErrorInvalid::new(
                &"An unknown error occurred".to_owned(),
            ));
        }
    };

    let (db, connection) = utils::get_db(&sql, &access_token)?;

    let trainer_name = TrainerName {
        name: ROMString::from(data.get_name()),
    };

    let patch = match db.set_trainer_name(&trainer_id, &trainer_name) {
        Ok(patch) => patch,
        Err(e) => return Err(TrainerNameResponseError::new(&e.to_string())),
    };

    let patch_description = match patch_description {
        Ok(patch_description) => patch_description.into_inner(),
        Err(_) => None,
    };

    match sql.insert_rom_patch(
        &connection,
        &access_token,
        &patch.to_raw(),
        patch_description,
    ) {
        Ok(_) => {}
        Err(e) => return Err(TrainerNameResponseError::new(&e.to_string())),
    };

    Ok(status::Accepted(Some(json!({}))))
}
