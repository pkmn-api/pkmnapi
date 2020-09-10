use pkmnapi_db::types::MoveName;
use serde::Serialize;

use crate::responses::base::{BaseResponse, BaseResponseType};
use crate::responses::links::Links;
use crate::utils;

pub type MoveResponse = BaseResponse<MoveResponseAttributes>;

impl MoveResponse {
    /// Create a new `MoveResponse`
    pub fn new(move_id: &u8, move_name: &MoveName) -> MoveResponse {
        MoveResponse {
            id: move_id.to_string(),
            _type: BaseResponseType::moves,
            attributes: MoveResponseAttributes {
                name: move_name.name.to_string(),
            },
            links: Links {
                _self: utils::generate_url("moves", Some(&move_id.to_string())),
            },
        }
    }
}

#[derive(Debug, Serialize)]
pub struct MoveResponseAttributes {
    pub name: String,
}
