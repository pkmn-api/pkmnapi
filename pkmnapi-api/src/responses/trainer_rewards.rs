use serde::Serialize;

use crate::responses::base::{BaseResponse, BaseResponseData, BaseResponseType};
use crate::responses::links::Links;
use crate::utils;

pub type TrainerRewardResponse = BaseResponse<TrainerRewardResponseAttributes>;

impl TrainerRewardResponse {
    /// Create a new `TrainerRewardResponse`
    pub fn new(trainer_id: &u8, trainer_reward: &u32) -> TrainerRewardResponse {
        TrainerRewardResponse {
            data: BaseResponseData {
                id: trainer_id.to_string(),
                _type: BaseResponseType::trainer_rewards,
                attributes: TrainerRewardResponseAttributes {
                    reward: *trainer_reward,
                },
                links: Links {
                    _self: utils::generate_url("trainers/rewards", Some(&trainer_id.to_string())),
                },
            },
            links: Links {
                _self: utils::generate_url("trainers/rewards", Some(&trainer_id.to_string())),
            },
        }
    }
}

#[derive(Debug, Serialize)]
pub struct TrainerRewardResponseAttributes {
    pub reward: u32,
}
