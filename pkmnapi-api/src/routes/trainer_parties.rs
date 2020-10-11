use pkmnapi_sql::*;
use rocket::response::status;
use rocket::State;
use rocket_contrib::json::{Json, JsonError, JsonValue};

use crate::guards::*;
use crate::requests::trainer_parties::*;
use crate::responses::errors::*;
use crate::responses::trainer_parties::*;
use crate::utils;

#[get("/trainers/parties/<trainer_id>")]
pub fn get_trainer_parties(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
    trainer_id: u8,
) -> Result<Json<TrainerPartiesResponse>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let trainer_parties = match db.get_trainer_parties(&trainer_id) {
        Ok(trainer_parties) => trainer_parties,
        Err(e) => {
            return Err(NotFoundError::new(
                BaseErrorResponseId::error_trainer_parties,
                Some(e.to_string()),
            ))
        }
    };

    let pokedex_ids = trainer_parties
        .iter()
        .map(|trainer_party| {
            trainer_party
                .pokemon
                .iter()
                .map(|party_pokemon| party_pokemon.pokedex_id)
        })
        .flatten()
        .collect();
    let pokemon_names = utils::get_pokemon_names(
        &db,
        &pokedex_ids,
        BaseErrorResponseId::error_trainer_parties,
    )?;

    let response = TrainerPartiesResponse::new(&trainer_id, &trainer_parties, pokemon_names);

    Ok(Json(response))
}

#[post(
    "/trainers/parties/<trainer_id>",
    format = "application/json",
    data = "<data>"
)]
pub fn post_trainer_parties(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
    patch_description: Result<PatchDescription, PatchDescriptionError>,
    data: Result<Json<TrainerPartiesRequest>, JsonError>,
    trainer_id: u8,
) -> Result<status::Accepted<JsonValue>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let data = utils::get_data(data, BaseErrorResponseId::error_trainer_parties_invalid)?;
    let (db, connection) = utils::get_db(&sql, &access_token)?;

    let trainer_parties = data.get_parties();

    let patch = match db.set_trainer_parties(&trainer_id, &trainer_parties) {
        Ok(patch) => patch,
        Err(e) => {
            return Err(NotFoundError::new(
                BaseErrorResponseId::error_trainer_parties,
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
        BaseErrorResponseId::error_trainer_parties,
    )?;

    Ok(status::Accepted(Some(json!({}))))
}
