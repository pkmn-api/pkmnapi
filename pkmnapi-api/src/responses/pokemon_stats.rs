use pkmnapi_db::{PokemonStats, TypeName};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::responses::base::{BaseResponse, BaseResponseAll, BaseResponseData, BaseResponseType};
use crate::responses::links::Links;
use crate::responses::type_names::TypeNameResponseData;
use crate::utils;

pub type PokemonStatsResponse = BaseResponse<PokemonStatsResponseAttributes>;
pub type PokemonStatsResponseData = BaseResponseData<PokemonStatsResponseAttributes>;
pub type PokemonStatsResponseAll = BaseResponseAll<PokemonStatsResponseData>;

impl PokemonStatsResponseAll {
    pub fn new(
        pokedex_ids: &Vec<u8>,
        pokemon_stats: &HashMap<u8, PokemonStats>,
        type_names: &HashMap<u8, TypeName>,
    ) -> PokemonStatsResponseAll {
        PokemonStatsResponseAll {
            data: pokedex_ids
                .iter()
                .map(|pokedex_id| {
                    PokemonStatsResponseData::new(
                        pokedex_id,
                        pokemon_stats.get(pokedex_id).unwrap(),
                        type_names,
                    )
                })
                .collect(),
            links: Links {
                _self: utils::generate_url("pokemon/stats", None),
            },
        }
    }
}

impl PokemonStatsResponse {
    pub fn new(
        pokedex_id: &u8,
        pokemon_stats: &PokemonStats,
        type_names: &HashMap<u8, TypeName>,
    ) -> PokemonStatsResponse {
        PokemonStatsResponse {
            data: PokemonStatsResponseData::new(pokedex_id, pokemon_stats, type_names),
            links: Links {
                _self: utils::generate_url("pokemon/stats", Some(&pokedex_id.to_string())),
            },
        }
    }
}

impl PokemonStatsResponseData {
    pub fn new(
        pokedex_id: &u8,
        pokemon_stats: &PokemonStats,
        type_names: &HashMap<u8, TypeName>,
    ) -> PokemonStatsResponseData {
        BaseResponseData {
            id: pokedex_id.to_string(),
            _type: BaseResponseType::pokemon_stats,
            attributes: PokemonStatsResponseAttributes {
                base_hp: pokemon_stats.base_hp,
                base_attack: pokemon_stats.base_attack,
                base_defence: pokemon_stats.base_defence,
                base_speed: pokemon_stats.base_speed,
                base_special: pokemon_stats.base_special,
                types: pokemon_stats
                    .type_ids
                    .iter()
                    .map(|type_id| {
                        TypeNameResponseData::new(&type_id, &type_names.get(type_id).unwrap())
                    })
                    .collect(),
                catch_rate: pokemon_stats.catch_rate,
                base_exp_yield: pokemon_stats.base_exp_yield,
                growth_rate: pokemon_stats.growth_rate,
            },
            links: Links {
                _self: utils::generate_url("pokemon/stats", Some(&pokedex_id.to_string())),
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PokemonStatsResponseAttributes {
    pub base_hp: u8,
    pub base_attack: u8,
    pub base_defence: u8,
    pub base_speed: u8,
    pub base_special: u8,
    pub types: Vec<TypeNameResponseData>,
    pub catch_rate: u8,
    pub base_exp_yield: u8,
    pub growth_rate: u8,
}
