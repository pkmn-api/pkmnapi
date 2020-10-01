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
    let access_token = match access_token {
        Ok(access_token) => access_token.into_inner(),
        Err(_) => return Err(AccessTokenErrorUnauthorized::new()),
    };

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
    let access_token = match access_token {
        Ok(access_token) => access_token.into_inner(),
        Err(_) => return Err(AccessTokenErrorUnauthorized::new()),
    };

    let data = match data {
        Ok(data) => data.into_inner(),
        Err(JsonError::Parse(_, e)) => {
            return Err(BadRequestError::new(
                BaseErrorResponseId::error_pokemon_names_invalid,
                Some(e.to_string()),
            ));
        }
        _ => {
            return Err(BadRequestError::new(
                BaseErrorResponseId::error_pokemon_names_invalid,
                Some("An unknown error occurred".to_owned()),
            ));
        }
    };

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

    let patch_description = match patch_description {
        Ok(patch_description) => patch_description.into_inner(),
        Err(_) => None,
    };

    if let Err(e) = sql.insert_rom_patch(
        &connection,
        &access_token,
        &patch.to_raw(),
        patch_description,
    ) {
        return Err(NotFoundError::new(
            BaseErrorResponseId::error_pokemon_names,
            Some(e.to_string()),
        ));
    }

    Ok(status::Accepted(Some(json!({}))))
}
