use pkmnapi_db::types::{MoveName, TM};
use serde::Serialize;

use crate::responses::base::{BaseResponse, BaseResponseData, BaseResponseType};
use crate::responses::links::Links;
use crate::responses::moves::MoveResponseData;
use crate::utils;

pub type TMResponse = BaseResponse<TMResponseAttributes>;

impl TMResponse {
    /// Create a new `TMResponse`
    pub fn new(tm_id: &u8, tm: &TM, move_name: &MoveName) -> TMResponse {
        TMResponse {
            data: BaseResponseData {
                id: tm_id.to_string(),
                _type: BaseResponseType::tms,
                attributes: TMResponseAttributes {
                    _move: MoveResponseData::new(&tm.move_id, move_name),
                },
                links: Links {
                    _self: utils::generate_url("tms", Some(&tm_id.to_string())),
                },
            },
            links: Links {
                _self: utils::generate_url("tms", Some(&tm_id.to_string())),
            },
        }
    }
}

#[derive(Debug, Serialize)]
pub struct TMResponseAttributes {
    #[serde(rename = "move")]
    pub _move: MoveResponseData,
}
