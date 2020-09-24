use pkmnapi_db::types::TrainerName;
use serde::Serialize;

use crate::responses::base::{BaseResponse, BaseResponseData, BaseResponseType};
use crate::responses::links::Links;
use crate::utils;

pub type TrainerNameResponse = BaseResponse<TrainerNameResponseAttributes>;

impl TrainerNameResponse {
    /// Create a new `TrainerNameResponse`
    pub fn new(trainer_id: &u8, trainer_name: &TrainerName) -> TrainerNameResponse {
        TrainerNameResponse {
            data: BaseResponseData {
                id: trainer_id.to_string(),
                _type: BaseResponseType::trainer_names,
                attributes: TrainerNameResponseAttributes {
                    name: trainer_name.name.to_string(),
                },
                links: Links {
                    _self: utils::generate_url("trainers/names", Some(&trainer_id.to_string())),
                },
            },
            links: Links {
                _self: utils::generate_url("trainers/names", Some(&trainer_id.to_string())),
            },
        }
    }
}

#[derive(Debug, Serialize)]
pub struct TrainerNameResponseAttributes {
    pub name: String,
}
