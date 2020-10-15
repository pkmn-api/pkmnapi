use pkmnapi_sql::*;
use rocket::response::status;
use rocket::State;
use rocket_contrib::json::{Json, JsonError, JsonValue};

use crate::guards::*;
use crate::requests::trainer_rewards::*;
use crate::responses::errors::*;
use crate::responses::trainer_rewards::*;
use crate::utils;

#[get("/trainers/rewards/<trainer_id>")]
pub fn get_trainer_reward(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
    trainer_id: u8,
) -> Result<Json<TrainerRewardResponse>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let trainer_reward = match db.get_trainer_reward(&trainer_id) {
        Ok(trainer_reward) => trainer_reward,
        Err(e) => {
            return Err(NotFoundError::new(
                BaseErrorResponseId::error_trainer_rewards,
                Some(e.to_string()),
            ))
        }
    };
    let response = TrainerRewardResponse::new(&trainer_id, &trainer_reward);

    Ok(Json(response))
}

#[post(
    "/trainers/rewards/<trainer_id>",
    format = "application/json",
    data = "<data>"
)]
pub fn post_trainer_reward(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
    patch_description: Result<PatchDescription, PatchDescriptionError>,
    data: Result<Json<TrainerRewardRequest>, JsonError>,
    trainer_id: u8,
) -> Result<status::Accepted<JsonValue>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let data = utils::get_data(data, BaseErrorResponseId::error_trainer_rewards_invalid)?;
    let (db, connection) = utils::get_db(&sql, &access_token)?;

    let trainer_reward = data.get_reward();

    let patch = match db.set_trainer_reward(&trainer_id, &trainer_reward) {
        Ok(patch) => patch,
        Err(e) => {
            return Err(NotFoundError::new(
                BaseErrorResponseId::error_trainer_rewards,
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
        BaseErrorResponseId::error_trainer_rewards,
    )?;

    Ok(status::Accepted(Some(json!({}))))
}
