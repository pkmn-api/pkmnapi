use pkmnapi_db::string::*;
use pkmnapi_db::types::*;
use pkmnapi_sql::*;
use rocket::response::status;
use rocket::State;
use rocket_contrib::json::{Json, JsonError, JsonValue};

use crate::guards::*;
use crate::requests::pokemon_names::*;
use crate::responses::errors::*;
use crate::responses::pokemon_names::*;
use crate::utils;

#[get("/pokemon/names/<pokedex_id>")]
pub fn get_pokemon_name(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
    pokedex_id: u8,
) -> Result<Json<PokemonNameResponse>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let pokemon_name = match db.get_pokemon_name(&pokedex_id) {
        Ok(pokemon_name) => pokemon_name,
        Err(e) => {
            return Err(NotFoundError::new(
                BaseErrorResponseId::error_pokemon_names,
                Some(e.to_string()),
            ))
        }
    };
    let response = PokemonNameResponse::new(&pokedex_id, &pokemon_name);

    Ok(Json(response))
}

#[post(
    "/pokemon/names/<pokedex_id>",
    format = "application/json",
    data = "<data>"
)]
pub fn post_pokemon_name(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
    patch_description: Result<PatchDescription, PatchDescriptionError>,
    data: Result<Json<PokemonNameRequest>, JsonError>,
    pokedex_id: u8,
) -> Result<status::Accepted<JsonValue>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let data = utils::get_data(data, BaseErrorResponseId::error_pokemon_names_invalid)?;
    let (db, connection) = utils::get_db(&sql, &access_token)?;

    let pokemon_name = PokemonName {
        name: ROMString::from(data.get_name()),
    };

    let patch = match db.set_pokemon_name(&pokedex_id, &pokemon_name) {
        Ok(patch) => patch,
        Err(e) => {
            return Err(NotFoundError::new(
                BaseErrorResponseId::error_pokemon_names,
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
        BaseErrorResponseId::error_pokemon_names,
    )?;

    Ok(status::Accepted(Some(json!({}))))
}
