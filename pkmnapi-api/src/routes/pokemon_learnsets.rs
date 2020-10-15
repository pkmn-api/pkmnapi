use pkmnapi_sql::*;
use rocket::response::status;
use rocket::State;
use rocket_contrib::json::{Json, JsonError, JsonValue};

use crate::guards::*;
use crate::requests::pokemon_learnsets::*;
use crate::responses::errors::*;
use crate::responses::pokemon_learnsets::*;
use crate::utils;

#[get("/pokemon/learnsets/<pokedex_id>")]
pub fn get_pokemon_learnset(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
    pokedex_id: u8,
) -> Result<Json<PokemonLearnsetResponse>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let pokemon_learnset = match db.get_pokemon_learnset(&pokedex_id) {
        Ok(pokemon_learnset) => pokemon_learnset,
        Err(e) => {
            return Err(NotFoundError::new(
                BaseErrorResponseId::error_pokemon_learnsets,
                Some(e.to_string()),
            ))
        }
    };

    let move_ids = pokemon_learnset
        .iter()
        .map(|learnset| learnset.move_id)
        .collect();
    let move_names =
        utils::get_move_names(&db, &move_ids, BaseErrorResponseId::error_pokemon_learnsets)?;

    let response = PokemonLearnsetResponse::new(&pokedex_id, &pokemon_learnset, move_names);

    Ok(Json(response))
}

#[post(
    "/pokemon/learnsets/<pokedex_id>",
    format = "application/json",
    data = "<data>"
)]
pub fn post_pokemon_learnset(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
    patch_description: Result<PatchDescription, PatchDescriptionError>,
    data: Result<Json<PokemonLearnsetRequest>, JsonError>,
    pokedex_id: u8,
) -> Result<status::Accepted<JsonValue>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let data = utils::get_data(data, BaseErrorResponseId::error_pokemon_learnsets_invalid)?;
    let (db, connection) = utils::get_db(&sql, &access_token)?;

    let pokemon_learnset = data.get_learnset();

    let patch = match db.set_pokemon_learnset(&pokedex_id, &pokemon_learnset) {
        Ok(patch) => patch,
        Err(e) => {
            return Err(NotFoundError::new(
                BaseErrorResponseId::error_pokemon_learnsets,
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
        BaseErrorResponseId::error_pokemon_learnsets,
    )?;

    Ok(status::Accepted(Some(json!({}))))
}
