use pkmnapi_db::cry::Cry;
use rocket_okapi::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::responses::base::{BaseResponse, BaseResponseAll, BaseResponseData, BaseResponseType};
use crate::responses::links::Links;
use crate::utils;

pub type PokemonCryResponse = BaseResponse<PokemonCryResponseAttributes>;
pub type PokemonCryResponseData = BaseResponseData<PokemonCryResponseAttributes>;
pub type PokemonCryResponseAll = BaseResponseAll<PokemonCryResponseData>;

impl PokemonCryResponseAll {
    pub fn new(pokedex_ids: &Vec<u8>, pokemon_cries: &HashMap<u8, Cry>) -> PokemonCryResponseAll {
        PokemonCryResponseAll {
            data: pokedex_ids
                .iter()
                .map(|pokedex_id| {
                    PokemonCryResponseData::new(pokedex_id, pokemon_cries.get(pokedex_id).unwrap())
                })
                .collect(),
            links: Links {
                _self: utils::generate_url("pokemon/cries", None),
            },
        }
    }
}

impl PokemonCryResponse {
    pub fn new(pokedex_id: &u8, pokemon_cry: &Cry) -> PokemonCryResponse {
        PokemonCryResponse {
            data: PokemonCryResponseData::new(pokedex_id, pokemon_cry),
            links: Links {
                _self: utils::generate_url("pokemon/cries", Some(&pokedex_id.to_string())),
            },
        }
    }
}

impl PokemonCryResponseData {
    pub fn new(pokedex_id: &u8, pokemon_cry: &Cry) -> PokemonCryResponseData {
        BaseResponseData {
            id: pokedex_id.to_string(),
            _type: BaseResponseType::pokemon_cries,
            attributes: PokemonCryResponseAttributes {
                base: pokemon_cry.base,
                pitch: pokemon_cry.pitch,
                length: pokemon_cry.length,
            },
            links: Links {
                _self: utils::generate_url("pokemon/cries", Some(&pokedex_id.to_string())),
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PokemonCryResponseAttributes {
    pub base: u8,
    pub pitch: u8,
    pub length: u8,
}
