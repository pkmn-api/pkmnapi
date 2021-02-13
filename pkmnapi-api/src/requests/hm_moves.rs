use rocket_okapi::JsonSchema;
use serde::Deserialize;

use crate::requests::base::BaseRequest;

pub type HMMoveRequest = BaseRequest<HMMoveRequestType, HMMoveRequestAttributes>;

impl HMMoveRequest {
    pub fn get_move_id(&self) -> u8 {
        self.data.attributes._move.id
    }
}

#[derive(Debug, Deserialize, JsonSchema)]
#[allow(non_camel_case_types)]
pub enum HMMoveRequestType {
    hm_moves,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct HMMoveRequestAttributes {
    #[serde(rename = "move")]
    pub _move: HMMoveRequestAttributesMove,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct HMMoveRequestAttributesMove {
    #[serde(deserialize_with = "crate::utils::from_numeric_str")]
    pub id: u8,
}
