use pkmnapi_db::types::PokemonName;
use pkmnapi_sql::*;
use rocket::response::status;
use rocket::State;
use rocket_contrib::json::{Json, JsonError, JsonValue};
use std::collections::HashMap;

use crate::guards::*;
use crate::requests::trainer_parties::*;
use crate::responses::errors::*;
use crate::responses::trainer_parties::*;
use crate::utils;

#[get("/trainer/parties/<trainer_id>")]
pub fn get_trainer_parties(
    sql: State<PkmnapiSQL>,
    access_token: Result<AccessToken, AccessTokenError>,
    trainer_id: u8,
) -> Result<Json<TrainerPartiesResponse>, ResponseError> {
    let access_token = match access_token {
        Ok(access_token) => access_token.into_inner(),
        Err(_) => return Err(AccessTokenErrorUnauthorized::new()),
    };

    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let trainer_parties = match db.get_trainer_parties(&trainer_id) {
        Ok(trainer_parties) => trainer_parties,
        Err(e) => return Err(TrainerPartiesResponseError::new(&e.to_string())),
    };

    let mut pokemon_names: HashMap<u8, PokemonName> = HashMap::new();

    match trainer_parties
        .iter()
        .map(|trainer_party| {
            trainer_party
                .pokemon
                .iter()
                .map(|party_pokemon| party_pokemon.pokedex_id)
        })
        .flatten()
        .map(|pokedex_id| {
            let pokemon_name = match db.get_pokemon_name(&pokedex_id) {
                Ok(pokemon_name) => pokemon_name,
                Err(e) => return Err(TrainerPartiesResponseError::new(&e.to_string())),
            };

            pokemon_names.insert(pokedex_id, pokemon_name);

            Ok(())
        })
        .collect::<Result<Vec<_>, ResponseError>>()
    {
        Ok(_) => {}
        Err(e) => return Err(e),
    };

    let response = TrainerPartiesResponse::new(&trainer_id, &trainer_parties, pokemon_names);

    Ok(Json(response))
}

#[post(
    "/trainer/parties/<trainer_id>",
    format = "application/json",
    data = "<data>"
)]
pub fn post_trainer_parties(
    sql: State<PkmnapiSQL>,
    access_token: Result<AccessToken, AccessTokenError>,
    patch_description: Result<PatchDescription, PatchDescriptionError>,
    data: Result<Json<TrainerPartiesRequest>, JsonError>,
    trainer_id: u8,
) -> Result<status::Accepted<JsonValue>, ResponseError> {
    let access_token = match access_token {
        Ok(access_token) => access_token.into_inner(),
        Err(_) => return Err(AccessTokenErrorUnauthorized::new()),
    };

    let data = match data {
        Ok(data) => data.into_inner(),
        Err(JsonError::Parse(_, e)) => {
            return Err(TrainerPartiesResponseErrorInvalid::new(&e.to_string()));
        }
        _ => {
            return Err(TrainerPartiesResponseErrorInvalid::new(
                &"An unknown error occurred".to_owned(),
            ));
        }
    };

    let (db, connection) = utils::get_db(&sql, &access_token)?;

    let trainer_parties = data.get_parties();

    let patch = match db.set_trainer_parties(&trainer_id, &trainer_parties) {
        Ok(patch) => patch,
        Err(e) => return Err(TrainerPartiesResponseError::new(&e.to_string())),
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
        Err(e) => return Err(TrainerPartiesResponseError::new(&e.to_string())),
    };

    Ok(status::Accepted(Some(json!({}))))
}
