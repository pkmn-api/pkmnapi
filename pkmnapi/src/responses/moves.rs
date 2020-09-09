use pkmnapi_db::types::MoveName;
use serde::Serialize;

use crate::responses::links::Links;
use crate::utils;

#[derive(Debug, Serialize)]
pub struct MoveResponse {
    pub id: String,
    #[serde(rename = "type")]
    pub _type: MoveResponseMove,
    pub attributes: MoveResponseAttributes,
    pub links: Links,
}

impl MoveResponse {
    /// Create a new `MoveResponse`
    pub fn new(move_id: &u8, move_name: &MoveName) -> MoveResponse {
        MoveResponse {
            id: move_id.to_string(),
            _type: MoveResponseMove::moves,
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
#[allow(non_camel_case_types)]
pub enum MoveResponseMove {
    moves,
}

#[derive(Debug, Serialize)]
pub struct MoveResponseAttributes {
    pub name: String,
}
