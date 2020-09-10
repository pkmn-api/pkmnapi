use pkmnapi_db::types::{MoveName, TM};
use serde::Serialize;

use crate::responses::base::{BaseResponse, BaseResponseType};
use crate::responses::links::Links;
use crate::responses::moves::MoveResponse;
use crate::utils;

pub type TMResponse = BaseResponse<TMResponseAttributes>;

impl TMResponse {
    /// Create a new `TMResponse`
    pub fn new(tm_id: &u8, tm: &TM, move_name: &MoveName) -> TMResponse {
        TMResponse {
            id: tm_id.to_string(),
            _type: BaseResponseType::tms,
            attributes: TMResponseAttributes {
                _move: MoveResponse::new(&tm.move_id, move_name),
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
    pub _move: MoveResponse,
}
