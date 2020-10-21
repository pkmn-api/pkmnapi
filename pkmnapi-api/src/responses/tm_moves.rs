use pkmnapi_db::{MoveName, TMMove};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::responses::base::{BaseResponse, BaseResponseAll, BaseResponseData, BaseResponseType};
use crate::responses::links::Links;
use crate::responses::move_names::MoveNameResponseData;
use crate::utils;

pub type TMMoveResponse = BaseResponse<TMMoveResponseAttributes>;
pub type TMMoveResponseData = BaseResponseData<TMMoveResponseAttributes>;
pub type TMMoveResponseAll = BaseResponseAll<TMMoveResponseData>;

impl TMMoveResponseAll {
    pub fn new(
        tm_ids: &Vec<u8>,
        tm_moves: &HashMap<u8, TMMove>,
        move_names: &HashMap<u8, MoveName>,
    ) -> TMMoveResponseAll {
        TMMoveResponseAll {
            data: tm_ids
                .iter()
                .map(|tm_id| {
                    let tm_move = tm_moves.get(tm_id).unwrap();

                    TMMoveResponseData::new(
                        tm_id,
                        tm_move,
                        move_names.get(&tm_move.move_id).unwrap(),
                    )
                })
                .collect(),
            links: Links {
                _self: utils::generate_url("tms/moves", None),
            },
        }
    }
}

impl TMMoveResponse {
    pub fn new(tm_id: &u8, tm_move: &TMMove, move_name: &MoveName) -> TMMoveResponse {
        TMMoveResponse {
            data: TMMoveResponseData::new(tm_id, tm_move, move_name),
            links: Links {
                _self: utils::generate_url("tms/moves", Some(&tm_id.to_string())),
            },
        }
    }
}

impl TMMoveResponseData {
    pub fn new(tm_id: &u8, tm_move: &TMMove, move_name: &MoveName) -> TMMoveResponseData {
        BaseResponseData {
            id: tm_id.to_string(),
            _type: BaseResponseType::tm_moves,
            attributes: TMMoveResponseAttributes {
                _move: MoveNameResponseData::new(&tm_move.move_id, move_name),
            },
            links: Links {
                _self: utils::generate_url("tms/moves", Some(&tm_id.to_string())),
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TMMoveResponseAttributes {
    #[serde(rename = "move")]
    pub _move: MoveNameResponseData,
}
