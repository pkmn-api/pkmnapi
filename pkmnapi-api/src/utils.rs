use pkmnapi_db::error;
use pkmnapi_db::patch::*;
use pkmnapi_db::types::{ItemName, MoveName, PokemonName, HM, TM};
use pkmnapi_db::*;
use pkmnapi_sql::*;
use rocket::{Data, State};
use rocket_contrib::json::{Json, JsonError};
use serde::de::{self, Deserializer};
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::fmt::Display;
use std::str::FromStr;

use crate::guards::*;
use crate::responses::errors::*;

pub fn get_access_token(
    access_token: Result<AccessToken, AccessTokenError>,
) -> Result<String, ResponseError> {
    match access_token {
        Ok(access_token) => Ok(access_token.into_inner()),
        Err(_) => Err(AccessTokenErrorUnauthorized::new()),
    }
}

pub fn get_etag(if_match: Result<IfMatch, IfMatchError>) -> Result<String, ResponseError> {
    match if_match {
        Ok(if_match) => Ok(if_match.into_inner()),
        Err(_) => Err(ETagErrorMissing::new()),
    }
}

pub fn get_data<T>(
    data: Result<Json<T>, JsonError>,
    error_id: BaseErrorResponseId,
) -> Result<T, ResponseError> {
    match data {
        Ok(data) => Ok(data.into_inner()),
        Err(JsonError::Parse(_, e)) => {
            return Err(BadRequestError::new(error_id, Some(e.to_string())));
        }
        _ => {
            return Err(BadRequestError::new(
                error_id,
                Some("An unknown error occurred".to_owned()),
            ));
        }
    }
}

pub fn get_data_raw(data: Data) -> Vec<u8> {
    let mut raw = Vec::new();

    data.stream_to(&mut raw).unwrap();

    raw
}

pub fn get_db(
    sql: &State<PkmnapiSQL>,
    access_token: &String,
) -> Result<(PkmnapiDB, PgPooledConnection), ResponseError> {
    let connection = sql.get_connection().unwrap();
    let rom_data = match sql.select_user_rom_data_by_access_token(&connection, &access_token) {
        Ok(Some(rom_data)) => rom_data,
        _ => return Err(RomErrorNoRom::new()),
    };
    let sav = match sql.select_user_sav_by_access_token(&connection, &access_token) {
        Ok(Some(sav)) => Some(sav.data),
        _ => None,
    };

    let db = match PkmnapiDB::new(&rom_data.data, sav) {
        Ok(db) => db,
        Err(_) => return Err(RomErrorInvalidRom::new()),
    };

    Ok((db, connection))
}

pub fn get_db_with_applied_patches(
    sql: &State<PkmnapiSQL>,
    access_token: &String,
) -> Result<(PkmnapiDB, PgPooledConnection), ResponseError> {
    let (mut db, connection) = get_db(sql, access_token)?;

    let rom_patches = match sql.select_rom_patches_by_access_token(&connection, &access_token) {
        Ok(patches) => patches,
        Err(_) => vec![],
    };

    let sav_patches = match sql.select_sav_patches_by_access_token(&connection, &access_token) {
        Ok(patches) => patches,
        Err(_) => vec![],
    };

    for patch in rom_patches {
        db.apply_patch(patch.data);
    }

    if let Some(ref mut sav) = db.sav {
        for patch in sav_patches {
            sav.apply_patch(patch.data);
        }
    }

    Ok((db, connection))
}

pub fn get_pokemon_names(
    db: &PkmnapiDB,
    pokedex_ids: &Vec<u8>,
    error_id: BaseErrorResponseId,
) -> Result<HashMap<u8, PokemonName>, ResponseError> {
    let mut pokemon_names: HashMap<u8, PokemonName> = HashMap::new();
    let result = pokedex_ids
        .iter()
        .map(|pokedex_id| {
            let pokemon_name = db.get_pokemon_name(pokedex_id)?;

            pokemon_names.insert(*pokedex_id, pokemon_name);

            Ok(())
        })
        .collect::<Result<Vec<_>, error::Error>>();

    match result {
        Ok(_) => Ok(pokemon_names),
        Err(e) => Err(NotFoundError::new(error_id, Some(e.to_string()))),
    }
}

pub fn get_item_names(
    db: &PkmnapiDB,
    item_ids: &Vec<u8>,
    error_id: BaseErrorResponseId,
) -> Result<HashMap<u8, ItemName>, ResponseError> {
    let mut item_names: HashMap<u8, ItemName> = HashMap::new();
    let result = item_ids
        .iter()
        .map(|item_id| {
            let item_name = db.get_item_name(item_id)?;

            item_names.insert(*item_id, item_name);

            Ok(())
        })
        .collect::<Result<Vec<_>, error::Error>>();

    match result {
        Ok(_) => Ok(item_names),
        Err(e) => Err(NotFoundError::new(error_id, Some(e.to_string()))),
    }
}

pub fn get_move_names(
    db: &PkmnapiDB,
    move_ids: &Vec<u8>,
    error_id: BaseErrorResponseId,
) -> Result<HashMap<u8, MoveName>, ResponseError> {
    let mut move_names: HashMap<u8, MoveName> = HashMap::new();
    let result = move_ids
        .iter()
        .map(|move_id| {
            let move_name = db.get_move_name(move_id)?;

            move_names.insert(*move_id, move_name);

            Ok(())
        })
        .collect::<Result<Vec<_>, error::Error>>();

    match result {
        Ok(_) => Ok(move_names),
        Err(e) => Err(NotFoundError::new(error_id, Some(e.to_string()))),
    }
}

pub fn get_tm_moves(
    db: &PkmnapiDB,
    tm_ids: &Vec<u8>,
    error_id: BaseErrorResponseId,
) -> Result<HashMap<u8, TM>, ResponseError> {
    let mut tm_moves: HashMap<u8, TM> = HashMap::new();
    let result = tm_ids
        .iter()
        .map(|tm_id| {
            let tm_move = db.get_tm(tm_id)?;

            tm_moves.insert(*tm_id, tm_move);

            Ok(())
        })
        .collect::<Result<Vec<_>, error::Error>>();

    match result {
        Ok(_) => Ok(tm_moves),
        Err(e) => Err(NotFoundError::new(error_id, Some(e.to_string()))),
    }
}

pub fn get_hm_moves(
    db: &PkmnapiDB,
    hm_ids: &Vec<u8>,
    error_id: BaseErrorResponseId,
) -> Result<HashMap<u8, HM>, ResponseError> {
    let mut hm_moves: HashMap<u8, HM> = HashMap::new();
    let result = hm_ids
        .iter()
        .map(|hm_id| {
            let hm_move = db.get_hm(hm_id)?;

            hm_moves.insert(*hm_id, hm_move);

            Ok(())
        })
        .collect::<Result<Vec<_>, error::Error>>();

    match result {
        Ok(_) => Ok(hm_moves),
        Err(e) => Err(NotFoundError::new(error_id, Some(e.to_string()))),
    }
}

pub fn get_patch_description(
    patch_description: Result<PatchDescription, PatchDescriptionError>,
) -> Option<String> {
    match patch_description {
        Ok(patch_description) => patch_description.into_inner(),
        Err(_) => None,
    }
}

pub fn insert_rom_patch(
    sql: State<PkmnapiSQL>,
    connection: PgPooledConnection,
    access_token: String,
    patch: Patch,
    patch_description: Result<PatchDescription, PatchDescriptionError>,
    error_id: BaseErrorResponseId,
) -> Result<(), ResponseError> {
    let patch_description = get_patch_description(patch_description);

    match sql.insert_rom_patch(
        &connection,
        &access_token,
        &patch.to_raw(),
        patch_description,
    ) {
        Ok(_) => Ok(()),
        Err(e) => return Err(NotFoundError::new(error_id, Some(e.to_string()))),
    }
}

pub fn generate_url(route: &str, resource: Option<&String>) -> String {
    let version = env::var("API_VERSION").unwrap_or("1".to_owned());
    let domain = env::var("API_DOMAIN").unwrap_or("localhost".to_owned());
    let resource = match &resource {
        Some(resource) => format!("/{}", resource),
        None => "".to_owned(),
    };

    format!("{}/v{}/{}{}", domain, version, route, resource)
}

pub fn from_numeric_str<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: FromStr,
    T::Err: Display,
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    T::from_str(&s).map_err(de::Error::custom)
}
