use pkmnapi_db::*;
use pkmnapi_sql::*;
use rocket::response::status;
use rocket::State;
use rocket_contrib::json::{Json, JsonError, JsonValue};

use crate::guards::*;
use crate::requests::pokemon_machines::*;
use crate::responses::errors::*;
use crate::responses::pokemon_machines::*;
use crate::utils;

#[get("/pokemon/machines/<pokedex_id>")]
pub fn get_pokemon_machines(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
    pokedex_id: u8,
) -> Result<Json<PokemonMachinesResponse>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let pokemon_machines = match db.get_pokemon_machines(&pokedex_id) {
        Ok(pokemon_machines) => pokemon_machines,
        Err(e) => {
            return Err(NotFoundError::new(
                BaseErrorResponseId::error_pokemon_machines,
                Some(e.to_string()),
            ))
        }
    };

    let tm_ids = pokemon_machines
        .iter()
        .filter_map(|machine| match machine {
            PokemonMachine::TM(tm_id) => Some(*tm_id),
            _ => None,
        })
        .collect();

    let tm_moves = utils::get_tm_moves(&db, &tm_ids, BaseErrorResponseId::error_pokemon_machines)?;

    let hm_ids = pokemon_machines
        .iter()
        .filter_map(|machine| match machine {
            PokemonMachine::HM(hm_id) => Some(*hm_id),
            _ => None,
        })
        .collect();

    let hm_moves = utils::get_hm_moves(&db, &hm_ids, BaseErrorResponseId::error_pokemon_machines)?;

    let move_ids = [
        tm_moves
            .iter()
            .map(|(_tm_id, tm_move)| tm_move.move_id)
            .collect::<Vec<u8>>(),
        hm_moves
            .iter()
            .map(|(_hm_id, hm_move)| hm_move.move_id)
            .collect::<Vec<u8>>(),
    ]
    .concat();

    let move_names =
        utils::get_move_names(&db, &move_ids, BaseErrorResponseId::error_pokemon_machines)?;

    let response = PokemonMachinesResponse::new(
        &pokedex_id,
        &pokemon_machines,
        &tm_moves,
        &hm_moves,
        &move_names,
    );

    Ok(Json(response))
}

#[post(
    "/pokemon/machines/<pokedex_id>",
    format = "application/json",
    data = "<data>"
)]
pub fn post_pokemon_machines(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
    patch_description: Result<PatchDescription, PatchDescriptionError>,
    data: Result<Json<PokemonMachinesRequest>, JsonError>,
    pokedex_id: u8,
) -> Result<status::Accepted<JsonValue>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let data = utils::get_data(data, BaseErrorResponseId::error_pokemon_machines_invalid)?;
    let (db, connection) = utils::get_db(&sql, &access_token)?;

    let pokemon_machines = data.get_machines();

    let patch = match db.set_pokemon_machines(&pokedex_id, &pokemon_machines) {
        Ok(patch) => patch,
        Err(e) => {
            return Err(NotFoundError::new(
                BaseErrorResponseId::error_pokemon_machines,
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
        BaseErrorResponseId::error_pokemon_machines,
    )?;

    Ok(status::Accepted(Some(json!({}))))
}
