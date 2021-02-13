use pkmnapi_db::{MoveStats, TypeName};
use rocket_okapi::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::responses::base::{BaseResponse, BaseResponseAll, BaseResponseData, BaseResponseType};
use crate::responses::links::Links;
use crate::responses::type_names::TypeNameResponseData;
use crate::utils;

pub type MoveStatsResponse = BaseResponse<MoveStatsResponseAttributes>;
pub type MoveStatsResponseData = BaseResponseData<MoveStatsResponseAttributes>;
pub type MoveStatsResponseAll = BaseResponseAll<MoveStatsResponseData>;

impl MoveStatsResponseAll {
    pub fn new(
        move_ids: &Vec<u8>,
        move_stats: &HashMap<u8, MoveStats>,
        type_names: &HashMap<u8, TypeName>,
    ) -> MoveStatsResponseAll {
        MoveStatsResponseAll {
            data: move_ids
                .iter()
                .map(|move_id| {
                    let stats = move_stats.get(move_id).unwrap();

                    MoveStatsResponseData::new(
                        move_id,
                        stats,
                        type_names.get(&stats.type_id).unwrap(),
                    )
                })
                .collect(),
            links: Links {
                _self: utils::generate_url("moves/stats", None),
            },
        }
    }
}

impl MoveStatsResponse {
    pub fn new(move_id: &u8, move_stats: &MoveStats, type_name: &TypeName) -> MoveStatsResponse {
        MoveStatsResponse {
            data: MoveStatsResponseData::new(move_id, move_stats, type_name),
            links: Links {
                _self: utils::generate_url("moves/stats", Some(&move_id.to_string())),
            },
        }
    }
}

impl MoveStatsResponseData {
    pub fn new(
        move_id: &u8,
        move_stats: &MoveStats,
        type_name: &TypeName,
    ) -> MoveStatsResponseData {
        BaseResponseData {
            id: move_id.to_string(),
            _type: BaseResponseType::move_stats,
            attributes: MoveStatsResponseAttributes {
                effect: move_stats.effect,
                power: move_stats.power,
                _type: TypeNameResponseData::new(&move_stats.type_id, type_name),
                accuracy: move_stats.accuracy,
                pp: move_stats.pp,
            },
            links: Links {
                _self: utils::generate_url("moves/stats", Some(&move_id.to_string())),
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct MoveStatsResponseAttributes {
    pub effect: u8,
    pub power: u8,

    #[serde(rename = "type")]
    pub _type: TypeNameResponseData,
    pub accuracy: f32,
    pub pp: u8,
}
