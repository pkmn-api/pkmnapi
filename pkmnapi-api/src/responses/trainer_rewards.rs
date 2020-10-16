use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::responses::base::{BaseResponse, BaseResponseAll, BaseResponseData, BaseResponseType};
use crate::responses::links::Links;
use crate::utils;

pub type TrainerRewardResponse = BaseResponse<TrainerRewardResponseAttributes>;
pub type TrainerRewardResponseData = BaseResponseData<TrainerRewardResponseAttributes>;
pub type TrainerRewardResponseAll = BaseResponseAll<TrainerRewardResponseData>;

impl TrainerRewardResponseAll {
    pub fn new(
        trainer_ids: &Vec<u8>,
        trainer_rewards: &HashMap<u8, u32>,
    ) -> TrainerRewardResponseAll {
        TrainerRewardResponseAll {
            data: trainer_ids
                .iter()
                .map(|trainer_id| {
                    TrainerRewardResponseData::new(
                        trainer_id,
                        trainer_rewards.get(trainer_id).unwrap(),
                    )
                })
                .collect(),
            links: Links {
                _self: utils::generate_url("trainers/rewards", None),
            },
        }
    }
}

impl TrainerRewardResponse {
    pub fn new(trainer_id: &u8, trainer_reward: &u32) -> TrainerRewardResponse {
        TrainerRewardResponse {
            data: TrainerRewardResponseData::new(trainer_id, trainer_reward),
            links: Links {
                _self: utils::generate_url("trainers/rewards", Some(&trainer_id.to_string())),
            },
        }
    }
}

impl TrainerRewardResponseData {
    pub fn new(trainer_id: &u8, trainer_reward: &u32) -> TrainerRewardResponseData {
        BaseResponseData {
            id: trainer_id.to_string(),
            _type: BaseResponseType::trainer_rewards,
            attributes: TrainerRewardResponseAttributes {
                reward: *trainer_reward,
            },
            links: Links {
                _self: utils::generate_url("trainers/rewards", Some(&trainer_id.to_string())),
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrainerRewardResponseAttributes {
    pub reward: u32,
}
