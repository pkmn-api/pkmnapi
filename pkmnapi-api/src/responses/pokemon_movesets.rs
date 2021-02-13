use pkmnapi_db::MoveName;
use rocket_okapi::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::responses::base::{BaseResponse, BaseResponseAll, BaseResponseData, BaseResponseType};
use crate::responses::links::Links;
use crate::responses::move_names::MoveNameResponseData;
use crate::utils;

pub type PokemonMovesetResponse = BaseResponse<PokemonMovesetResponseAttributes>;
pub type PokemonMovesetResponseData = BaseResponseData<PokemonMovesetResponseAttributes>;
pub type PokemonMovesetResponseAll = BaseResponseAll<PokemonMovesetResponseData>;

impl PokemonMovesetResponseAll {
    pub fn new(
        pokedex_ids: &Vec<u8>,
        pokemon_movesets: &HashMap<u8, Vec<u8>>,
        move_names: &HashMap<u8, MoveName>,
    ) -> PokemonMovesetResponseAll {
        PokemonMovesetResponseAll {
            data: pokedex_ids
                .iter()
                .map(|pokedex_id| {
                    PokemonMovesetResponseData::new(
                        pokedex_id,
                        pokemon_movesets.get(pokedex_id).unwrap(),
                        move_names,
                    )
                })
                .collect(),
            links: Links {
                _self: utils::generate_url("pokemon/movesets", None),
            },
        }
    }
}

impl PokemonMovesetResponse {
    pub fn new(
        pokedex_id: &u8,
        pokemon_moveset: &Vec<u8>,
        move_names: &HashMap<u8, MoveName>,
    ) -> PokemonMovesetResponse {
        PokemonMovesetResponse {
            data: PokemonMovesetResponseData::new(pokedex_id, pokemon_moveset, move_names),
            links: Links {
                _self: utils::generate_url("pokemon/movesets", Some(&pokedex_id.to_string())),
            },
        }
    }
}

impl PokemonMovesetResponseData {
    pub fn new(
        pokedex_id: &u8,
        pokemon_moveset: &Vec<u8>,
        move_names: &HashMap<u8, MoveName>,
    ) -> PokemonMovesetResponseData {
        BaseResponseData {
            id: pokedex_id.to_string(),
            _type: BaseResponseType::pokemon_movesets,
            attributes: PokemonMovesetResponseAttributes {
                moveset: pokemon_moveset
                    .iter()
                    .map(|move_id| PokemonMovesetResponseAttributesMoveset {
                        _move: MoveNameResponseData::new(
                            &move_id,
                            &move_names.get(&move_id).unwrap(),
                        ),
                    })
                    .collect(),
            },
            links: Links {
                _self: utils::generate_url("pokemon/movesets", Some(&pokedex_id.to_string())),
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PokemonMovesetResponseAttributes {
    pub moveset: Vec<PokemonMovesetResponseAttributesMoveset>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PokemonMovesetResponseAttributesMoveset {
    #[serde(rename = "move")]
    pub _move: MoveNameResponseData,
}
