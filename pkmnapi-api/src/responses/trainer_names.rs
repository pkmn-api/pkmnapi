use pkmnapi_db::TrainerName;
use rocket_okapi::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::responses::base::{BaseResponse, BaseResponseAll, BaseResponseData, BaseResponseType};
use crate::responses::links::Links;
use crate::utils;

pub type TrainerNameResponse = BaseResponse<TrainerNameResponseAttributes>;
pub type TrainerNameResponseData = BaseResponseData<TrainerNameResponseAttributes>;
pub type TrainerNameResponseAll = BaseResponseAll<TrainerNameResponseData>;

impl TrainerNameResponseAll {
    pub fn new(
        trainer_ids: &Vec<u8>,
        trainer_names: &HashMap<u8, TrainerName>,
    ) -> TrainerNameResponseAll {
        TrainerNameResponseAll {
            data: trainer_ids
                .iter()
                .map(|trainer_id| {
                    TrainerNameResponseData::new(trainer_id, trainer_names.get(trainer_id).unwrap())
                })
                .collect(),
            links: Links {
                _self: utils::generate_url("trainers/names", None),
            },
        }
    }
}

impl TrainerNameResponse {
    pub fn new(trainer_id: &u8, trainer_name: &TrainerName) -> TrainerNameResponse {
        TrainerNameResponse {
            data: TrainerNameResponseData::new(trainer_id, trainer_name),
            links: Links {
                _self: utils::generate_url("trainers/names", Some(&trainer_id.to_string())),
            },
        }
    }
}

impl TrainerNameResponseData {
    pub fn new(trainer_id: &u8, trainer_name: &TrainerName) -> TrainerNameResponseData {
        BaseResponseData {
            id: trainer_id.to_string(),
            _type: BaseResponseType::trainer_names,
            attributes: TrainerNameResponseAttributes {
                name: trainer_name.name.to_string(),
            },
            links: Links {
                _self: utils::generate_url("trainers/names", Some(&trainer_id.to_string())),
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TrainerNameResponseAttributes {
    pub name: String,
}
