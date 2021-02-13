use pkmnapi_db::{MoveName, PokemonLearnset};
use rocket_okapi::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::responses::base::{BaseResponse, BaseResponseAll, BaseResponseData, BaseResponseType};
use crate::responses::links::Links;
use crate::responses::move_names::MoveNameResponseData;
use crate::utils;

pub type PokemonLearnsetResponse = BaseResponse<PokemonLearnsetResponseAttributes>;
pub type PokemonLearnsetResponseData = BaseResponseData<PokemonLearnsetResponseAttributes>;
pub type PokemonLearnsetResponseAll = BaseResponseAll<PokemonLearnsetResponseData>;

impl PokemonLearnsetResponseAll {
    pub fn new(
        pokedex_ids: &Vec<u8>,
        pokemon_learnsets: &HashMap<u8, Vec<PokemonLearnset>>,
        move_names: &HashMap<u8, MoveName>,
    ) -> PokemonLearnsetResponseAll {
        PokemonLearnsetResponseAll {
            data: pokedex_ids
                .iter()
                .map(|pokedex_id| {
                    PokemonLearnsetResponseData::new(
                        pokedex_id,
                        pokemon_learnsets.get(pokedex_id).unwrap(),
                        move_names,
                    )
                })
                .collect(),
            links: Links {
                _self: utils::generate_url("pokemon/learnsets", None),
            },
        }
    }
}

impl PokemonLearnsetResponse {
    pub fn new(
        pokedex_id: &u8,
        pokemon_learnset: &Vec<PokemonLearnset>,
        move_names: &HashMap<u8, MoveName>,
    ) -> PokemonLearnsetResponse {
        PokemonLearnsetResponse {
            data: PokemonLearnsetResponseData::new(pokedex_id, pokemon_learnset, move_names),
            links: Links {
                _self: utils::generate_url("pokemon/learnsets", Some(&pokedex_id.to_string())),
            },
        }
    }
}

impl PokemonLearnsetResponseData {
    pub fn new(
        pokedex_id: &u8,
        pokemon_learnset: &Vec<PokemonLearnset>,
        move_names: &HashMap<u8, MoveName>,
    ) -> PokemonLearnsetResponseData {
        BaseResponseData {
            id: pokedex_id.to_string(),
            _type: BaseResponseType::pokemon_learnsets,
            attributes: PokemonLearnsetResponseAttributes {
                learnset: pokemon_learnset
                    .iter()
                    .map(|learnset| PokemonLearnsetResponseAttributesLearnset {
                        level: learnset.level,
                        _move: MoveNameResponseData::new(
                            &learnset.move_id,
                            &move_names.get(&learnset.move_id).unwrap(),
                        ),
                    })
                    .collect(),
            },
            links: Links {
                _self: utils::generate_url("pokemon/learnsets", Some(&pokedex_id.to_string())),
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PokemonLearnsetResponseAttributes {
    pub learnset: Vec<PokemonLearnsetResponseAttributesLearnset>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PokemonLearnsetResponseAttributesLearnset {
    pub level: u8,

    #[serde(rename = "move")]
    pub _move: MoveNameResponseData,
}
