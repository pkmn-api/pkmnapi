use pkmnapi_db::cry::*;
use pkmnapi_sql::*;
use rocket::http::{ContentType, Header};
use rocket::response::status;
use rocket::response::Response;
use rocket::State;
use rocket_contrib::json::{Json, JsonError, JsonValue};
use std::io::Cursor;

use crate::guards::*;
use crate::requests::pokemon_cries::*;
use crate::responses::errors::*;
use crate::responses::pokemon_cries::*;
use crate::utils;

#[get("/pokemon/cries/<pokedex_id>", format = "application/json", rank = 1)]
pub fn get_pokemon_cry(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
    pokedex_id: u8,
) -> Result<Json<PokemonCryResponse>, ResponseError> {
    let access_token = match access_token {
        Ok(access_token) => access_token.into_inner(),
        Err(_) => return Err(AccessTokenErrorUnauthorized::new()),
    };

    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let pokemon_cry = match db.get_pokemon_cry(&pokedex_id) {
        Ok(pokemon_cry) => pokemon_cry,
        Err(e) => {
            return Err(NotFoundError::new(
                BaseErrorResponseId::error_pokemon_cries,
                Some(e.to_string()),
            ))
        }
    };
    let response = PokemonCryResponse::new(&pokedex_id, &pokemon_cry);

    Ok(Json(response))
}

#[get("/pokemon/cries/<pokedex_id>", format = "audio/wav", rank = 2)]
pub fn get_pokemon_cry_wav<'a>(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
    pokedex_id: u8,
) -> Result<Response<'a>, ResponseError> {
    let access_token = match access_token {
        Ok(access_token) => access_token.into_inner(),
        Err(_) => return Err(AccessTokenErrorUnauthorized::new()),
    };

    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let pokemon_cry = match db.get_pokemon_cry(&pokedex_id) {
        Ok(pokemon_cry) => pokemon_cry,
        Err(e) => {
            return Err(NotFoundError::new(
                BaseErrorResponseId::error_pokemon_cries,
                Some(e.to_string()),
            ))
        }
    };

    let pokemon_name = match db.get_pokemon_name(&pokedex_id) {
        Ok(pokemon_name) => pokemon_name,
        Err(e) => {
            return Err(NotFoundError::new(
                BaseErrorResponseId::error_pokemon_cries,
                Some(e.to_string()),
            ))
        }
    };

    let wav = match pokemon_cry.to_wav(48000) {
        Ok(wav) => wav,
        Err(e) => {
            return Err(NotFoundError::new(
                BaseErrorResponseId::error_pokemon_cries,
                Some(e.to_string()),
            ))
        }
    };

    let response = Response::build()
        .header(ContentType::WAV)
        .header(Header::new(
            "Content-Disposition",
            format!(r#"attachment; filename="{}.wav""#, pokemon_name.name),
        ))
        .sized_body(Cursor::new(wav))
        .finalize();

    Ok(response)
}

#[post(
    "/pokemon/cries/<pokedex_id>",
    format = "application/json",
    data = "<data>"
)]
pub fn post_pokemon_cry(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
    patch_description: Result<PatchDescription, PatchDescriptionError>,
    data: Result<Json<PokemonCryRequest>, JsonError>,
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
                BaseErrorResponseId::error_pokemon_cries_invalid,
                Some(e.to_string()),
            ));
        }
        _ => {
            return Err(BadRequestError::new(
                BaseErrorResponseId::error_pokemon_cries_invalid,
                Some("An unknown error occurred".to_owned()),
            ));
        }
    };

    let (db, connection) = utils::get_db(&sql, &access_token)?;

    let pokemon_cry = Cry {
        base: data.get_base(),
        pitch: data.get_pitch(),
        length: data.get_length(),
        ..Default::default()
    };

    let patch = match db.set_pokemon_cry(&pokedex_id, &pokemon_cry) {
        Ok(patch) => patch,
        Err(e) => {
            return Err(NotFoundError::new(
                BaseErrorResponseId::error_pokemon_cries,
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
            BaseErrorResponseId::error_pokemon_cries,
            Some(e.to_string()),
        ));
    }

    Ok(status::Accepted(Some(json!({}))))
}
