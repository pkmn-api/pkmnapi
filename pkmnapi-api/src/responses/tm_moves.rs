use pkmnapi_db::{MoveName, TM};
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
        tms: &HashMap<u8, TM>,
        move_names: &HashMap<u8, MoveName>,
    ) -> TMMoveResponseAll {
        TMMoveResponseAll {
            data: tm_ids
                .iter()
                .map(|tm_id| {
                    let tm = tms.get(tm_id).unwrap();

                    TMMoveResponseData::new(tm_id, tm, move_names.get(&tm.move_id).unwrap())
                })
                .collect(),
            links: Links {
                _self: utils::generate_url("tms/moves", None),
            },
        }
    }
}

impl TMMoveResponse {
    pub fn new(tm_id: &u8, tm: &TM, move_name: &MoveName) -> TMMoveResponse {
        TMMoveResponse {
            data: TMMoveResponseData::new(tm_id, tm, move_name),
            links: Links {
                _self: utils::generate_url("tms/moves", Some(&tm_id.to_string())),
            },
        }
    }
}

impl TMMoveResponseData {
    pub fn new(tm_id: &u8, tm: &TM, move_name: &MoveName) -> TMMoveResponseData {
        BaseResponseData {
            id: tm_id.to_string(),
            _type: BaseResponseType::tm_moves,
            attributes: TMMoveResponseAttributes {
                _move: MoveNameResponseData::new(&tm.move_id, move_name),
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
