use pkmnapi_db::types::{MoveName, TM};
use serde::Serialize;

use crate::responses::links::Links;
use crate::responses::moves::MoveResponse;
use crate::utils;

#[derive(Debug, Serialize)]
pub struct TMResponse {
    pub id: String,
    #[serde(rename = "type")]
    pub _type: TMResponseType,
    pub attributes: TMResponseAttributes,
    pub links: Links,
}

impl TMResponse {
    /// Create a new `TMResponse`
    pub fn new(tm_id: &u8, tm: &TM, move_name: &MoveName) -> TMResponse {
        TMResponse {
            id: tm_id.to_string(),
            _type: TMResponseType::tms,
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
#[allow(non_camel_case_types)]
pub enum TMResponseType {
    tms,
}

#[derive(Debug, Serialize)]
pub struct TMResponseAttributes {
    #[serde(rename = "move")]
    pub _move: MoveResponse,
}
