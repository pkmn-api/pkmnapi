use pkmnapi_db::PokemonName;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::responses::base::{BaseResponse, BaseResponseAll, BaseResponseData, BaseResponseType};
use crate::responses::links::Links;
use crate::utils;

pub type PokemonNameResponse = BaseResponse<PokemonNameResponseAttributes>;
pub type PokemonNameResponseData = BaseResponseData<PokemonNameResponseAttributes>;
pub type PokemonNameResponseAll = BaseResponseAll<PokemonNameResponseData>;

impl PokemonNameResponseAll {
    pub fn new(
        pokedex_ids: &Vec<u8>,
        pokemon_names: &HashMap<u8, PokemonName>,
    ) -> PokemonNameResponseAll {
        PokemonNameResponseAll {
            data: pokedex_ids
                .iter()
                .map(|pokedex_id| {
                    PokemonNameResponseData::new(
                        &pokedex_id,
                        &pokemon_names.get(pokedex_id).unwrap(),
                    )
                })
                .collect(),
            links: Links {
                _self: utils::generate_url("pokemon/names", None),
            },
        }
    }
}

impl PokemonNameResponse {
    pub fn new(pokedex_id: &u8, pokemon_name: &PokemonName) -> PokemonNameResponse {
        PokemonNameResponse {
            data: PokemonNameResponseData::new(pokedex_id, pokemon_name),
            links: Links {
                _self: utils::generate_url("pokemon/names", Some(&pokedex_id.to_string())),
            },
        }
    }
}

impl PokemonNameResponseData {
    pub fn new(pokedex_id: &u8, pokemon_name: &PokemonName) -> PokemonNameResponseData {
        BaseResponseData {
            id: pokedex_id.to_string(),
            _type: BaseResponseType::pokemon_names,
            attributes: PokemonNameResponseAttributes {
                name: pokemon_name.name.to_string(),
            },
            links: Links {
                _self: utils::generate_url("pokemon/names", Some(&pokedex_id.to_string())),
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PokemonNameResponseAttributes {
    pub name: String,
}
