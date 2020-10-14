use pkmnapi_db::{PokemonStats, TypeName};
use serde::Serialize;

use crate::responses::base::{BaseResponse, BaseResponseData, BaseResponseType};
use crate::responses::links::Links;
use crate::responses::type_names::TypeNameResponseData;
use crate::utils;

pub type PokemonStatsResponse = BaseResponse<PokemonStatsResponseAttributes>;

impl PokemonStatsResponse {
    /// Create a new `PokemonStatsResponse`
    pub fn new(
        pokedex_id: &u8,
        pokemon_stats: &PokemonStats,
        type_names: Vec<TypeName>,
    ) -> PokemonStatsResponse {
        PokemonStatsResponse {
            data: BaseResponseData {
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
                        .enumerate()
                        .map(|(i, type_id)| TypeNameResponseData::new(&type_id, &type_names[i]))
                        .collect(),
                    catch_rate: pokemon_stats.catch_rate,
                    base_exp_yield: pokemon_stats.base_exp_yield,
                },
                links: Links {
                    _self: utils::generate_url("pokemon/stats", Some(&pokedex_id.to_string())),
                },
            },
            links: Links {
                _self: utils::generate_url("pokemon/stats", Some(&pokedex_id.to_string())),
            },
        }
    }
}

#[derive(Debug, Serialize)]
pub struct PokemonStatsResponseAttributes {
    pub base_hp: u8,
    pub base_attack: u8,
    pub base_defence: u8,
    pub base_speed: u8,
    pub base_special: u8,
    pub types: Vec<TypeNameResponseData>,
    pub catch_rate: u8,
    pub base_exp_yield: u8,
}
