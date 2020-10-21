use pkmnapi_db::{HMMove, MoveName};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::responses::base::{BaseResponse, BaseResponseAll, BaseResponseData, BaseResponseType};
use crate::responses::links::Links;
use crate::responses::move_names::MoveNameResponseData;
use crate::utils;

pub type HMMoveResponse = BaseResponse<HMMoveResponseAttributes>;
pub type HMMoveResponseData = BaseResponseData<HMMoveResponseAttributes>;
pub type HMMoveResponseAll = BaseResponseAll<HMMoveResponseData>;

impl HMMoveResponseAll {
    pub fn new(
        hm_ids: &Vec<u8>,
        hm_moves: &HashMap<u8, HMMove>,
        move_names: &HashMap<u8, MoveName>,
    ) -> HMMoveResponseAll {
        HMMoveResponseAll {
            data: hm_ids
                .iter()
                .map(|hm_id| {
                    let hm_move = hm_moves.get(hm_id).unwrap();

                    HMMoveResponseData::new(
                        hm_id,
                        hm_move,
                        move_names.get(&hm_move.move_id).unwrap(),
                    )
                })
                .collect(),
            links: Links {
                _self: utils::generate_url("hms/moves", None),
            },
        }
    }
}

impl HMMoveResponse {
    pub fn new(hm_id: &u8, hm_move: &HMMove, move_name: &MoveName) -> HMMoveResponse {
        HMMoveResponse {
            data: HMMoveResponseData::new(hm_id, hm_move, move_name),
            links: Links {
                _self: utils::generate_url("hms/moves", Some(&hm_id.to_string())),
            },
        }
    }
}

impl HMMoveResponseData {
    pub fn new(hm_id: &u8, hm_move: &HMMove, move_name: &MoveName) -> HMMoveResponseData {
        BaseResponseData {
            id: hm_id.to_string(),
            _type: BaseResponseType::hm_moves,
            attributes: HMMoveResponseAttributes {
                _move: MoveNameResponseData::new(&hm_move.move_id, move_name),
            },
            links: Links {
                _self: utils::generate_url("hms/moves", Some(&hm_id.to_string())),
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HMMoveResponseAttributes {
    #[serde(rename = "move")]
    pub _move: MoveNameResponseData,
}
