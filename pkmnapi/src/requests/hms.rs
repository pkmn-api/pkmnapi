use serde::Deserialize;

use crate::requests::base::BaseRequest;

pub type HMRequest = BaseRequest<HMRequestType, HMRequestAttributes>;

impl HMRequest {
    pub fn get_move_id(&self) -> &String {
        &self.data.attributes._move.id
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
    pub id: String,
}
