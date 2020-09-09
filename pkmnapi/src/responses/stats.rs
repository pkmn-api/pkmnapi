use pkmnapi_db::types::{Stats, TypeName};
use serde::Serialize;

use crate::responses::links::Links;
use crate::responses::types::TypeResponse;
use crate::utils;

#[derive(Debug, Serialize)]
pub struct StatsResponse {
    pub id: String,
    #[serde(rename = "type")]
    pub _type: StatsResponseType,
    pub attributes: StatsResponseAttributes,
    pub links: Links,
}

impl StatsResponse {
    /// Create a new `StatsResponse`
    pub fn new(pokedex_id: &u8, stats: &Stats, type_names: Vec<TypeName>) -> StatsResponse {
        StatsResponse {
            id: pokedex_id.to_string(),
            _type: StatsResponseType::stats,
            attributes: StatsResponseAttributes {
                base_hp: stats.base_hp,
                base_attack: stats.base_attack,
                base_defence: stats.base_defence,
                base_speed: stats.base_speed,
                base_special: stats.base_special,
                types: stats
                    .type_ids
                    .iter()
                    .enumerate()
                    .map(|(i, type_id)| TypeResponse::new(&type_id, &type_names[i]))
                    .collect(),
                catch_rate: stats.catch_rate,
                base_exp_yield: stats.base_exp_yield,
            },
            links: Links {
                _self: utils::generate_url("stats", Some(&pokedex_id.to_string())),
            },
        }
    }
}

#[derive(Debug, Serialize)]
#[allow(non_camel_case_types)]
pub enum StatsResponseType {
    stats,
}

#[derive(Debug, Serialize)]
pub struct StatsResponseAttributes {
    pub base_hp: u8,
    pub base_attack: u8,
    pub base_defence: u8,
    pub base_speed: u8,
    pub base_special: u8,
    pub types: Vec<TypeResponse>,
    pub catch_rate: u8,
    pub base_exp_yield: u8,
}
