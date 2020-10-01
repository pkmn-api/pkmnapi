use pkmnapi_db::types::{MoveName, TM};
use serde::Serialize;

use crate::responses::base::{BaseResponse, BaseResponseData, BaseResponseType};
use crate::responses::links::Links;
use crate::responses::move_names::MoveNameResponseData;
use crate::utils;

pub type TMMoveResponse = BaseResponse<TMMoveResponseAttributes>;

impl TMMoveResponse {
    /// Create a new `TMMoveResponse`
    pub fn new(tm_id: &u8, tm: &TM, move_name: &MoveName) -> TMMoveResponse {
        TMMoveResponse {
            data: BaseResponseData {
                id: tm_id.to_string(),
                _type: BaseResponseType::tm_moves,
                attributes: TMMoveResponseAttributes {
                    _move: MoveNameResponseData::new(&tm.move_id, move_name),
                },
                links: Links {
                    _self: utils::generate_url("tms/moves", Some(&tm_id.to_string())),
                },
            },
            links: Links {
                _self: utils::generate_url("tms/moves", Some(&tm_id.to_string())),
            },
        }
    }
}

#[derive(Debug, Serialize)]
pub struct TMMoveResponseAttributes {
    #[serde(rename = "move")]
    pub _move: MoveNameResponseData,
}
