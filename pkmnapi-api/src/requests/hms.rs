use serde::Deserialize;

use crate::requests::base::BaseRequest;

pub type HMRequest = BaseRequest<HMRequestType, HMRequestAttributes>;

impl HMRequest {
    pub fn get_move_id(&self) -> u8 {
        self.data.attributes._move.id
    }
}

#[derive(Debug, Deserialize)]
#[allow(non_camel_case_types)]
pub enum HMRequestType {
    hms,
}

#[derive(Debug, Deserialize)]
pub struct HMRequestAttributes {
    #[serde(rename = "move")]
    pub _move: HMRequestAttributesMove,
}

#[derive(Debug, Deserialize)]
pub struct HMRequestAttributesMove {
    #[serde(deserialize_with = "crate::utils::from_numeric_str")]
    pub id: u8,
}
