use pkmnapi_db::{MoveName, PokemonLearnset};
use serde::Serialize;
use std::collections::HashMap;

use crate::responses::base::{BaseResponse, BaseResponseData, BaseResponseType};
use crate::responses::links::Links;
use crate::responses::move_names::MoveNameResponseData;
use crate::utils;

pub type PokemonLearnsetResponse = BaseResponse<PokemonLearnsetResponseAttributes>;

impl PokemonLearnsetResponse {
    /// Create a new `PokemonLearnsetResponse`
    pub fn new(
        pokedex_id: &u8,
        pokemon_learnset: &Vec<PokemonLearnset>,
        move_names: HashMap<u8, MoveName>,
    ) -> PokemonLearnsetResponse {
        PokemonLearnsetResponse {
            data: BaseResponseData {
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
            },
            links: Links {
                _self: utils::generate_url("pokemon/learnsets", Some(&pokedex_id.to_string())),
            },
        }
    }
}

#[derive(Debug, Serialize)]
pub struct PokemonLearnsetResponseAttributes {
    pub learnset: Vec<PokemonLearnsetResponseAttributesLearnset>,
}

#[derive(Debug, Serialize)]
pub struct PokemonLearnsetResponseAttributesLearnset {
    pub level: u8,

    #[serde(rename = "move")]
    pub _move: MoveNameResponseData,
}
