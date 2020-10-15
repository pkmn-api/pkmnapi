use serde::Deserialize;

use crate::requests::base::BaseRequest;

pub type TrainerRewardRequest =
    BaseRequest<TrainerRewardRequestType, TrainerRewardRequestAttributes>;

impl TrainerRewardRequest {
    pub fn get_reward(&self) -> u32 {
        self.data.attributes.reward
    }
}

#[derive(Debug, Deserialize)]
#[allow(non_camel_case_types)]
pub enum TrainerRewardRequestType {
    trainer_rewards,
}

#[derive(Debug, Deserialize)]
pub struct TrainerRewardRequestAttributes {
    pub reward: u32,
}
