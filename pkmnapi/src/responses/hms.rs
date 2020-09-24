use pkmnapi_db::types::{MoveName, HM};
use serde::Serialize;

use crate::responses::base::{BaseResponse, BaseResponseData, BaseResponseType};
use crate::responses::links::Links;
use crate::responses::move_names::MoveNameResponseData;
use crate::utils;

pub type HMResponse = BaseResponse<HMResponseAttributes>;

impl HMResponse {
    /// Create a new `HMResponse`
    pub fn new(hm_id: &u8, hm: &HM, move_name: &MoveName) -> HMResponse {
        HMResponse {
            data: BaseResponseData {
                id: hm_id.to_string(),
                _type: BaseResponseType::hms,
                attributes: HMResponseAttributes {
                    _move: MoveNameResponseData::new(&hm.move_id, move_name),
                },
                links: Links {
                    _self: utils::generate_url("hms", Some(&hm_id.to_string())),
                },
            },
            links: Links {
                _self: utils::generate_url("hms", Some(&hm_id.to_string())),
            },
        }
    }
}

#[derive(Debug, Serialize)]
pub struct HMResponseAttributes {
    #[serde(rename = "move")]
    pub _move: MoveNameResponseData,
}
