use pkmnapi_db::types::{MoveName, HM};
use serde::Serialize;

use crate::responses::base::{BaseResponse, BaseResponseData, BaseResponseType};
use crate::responses::links::Links;
use crate::responses::move_names::MoveNameResponseData;
use crate::utils;

pub type HMMoveResponse = BaseResponse<HMMoveResponseAttributes>;
pub type HMMoveResponseData = BaseResponseData<HMMoveResponseAttributes>;

impl HMMoveResponse {
    /// Create a new `HMMoveResponse`
    pub fn new(hm_id: &u8, hm: &HM, move_name: &MoveName) -> HMMoveResponse {
        HMMoveResponse {
            data: HMMoveResponseData::new(hm_id, hm, move_name),
            links: Links {
                _self: utils::generate_url("hms/moves", Some(&hm_id.to_string())),
            },
        }
    }
}

impl HMMoveResponseData {
    pub fn new(hm_id: &u8, hm: &HM, move_name: &MoveName) -> HMMoveResponseData {
        BaseResponseData {
            id: hm_id.to_string(),
            _type: BaseResponseType::hm_moves,
            attributes: HMMoveResponseAttributes {
                _move: MoveNameResponseData::new(&hm.move_id, move_name),
            },
            links: Links {
                _self: utils::generate_url("hms/moves", Some(&hm_id.to_string())),
            },
        }
    }
}

#[derive(Debug, Serialize)]
pub struct HMMoveResponseAttributes {
    #[serde(rename = "move")]
    pub _move: MoveNameResponseData,
}
