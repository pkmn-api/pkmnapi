use rocket_okapi::JsonSchema;
use serde::Deserialize;

use crate::requests::base::BaseRequest;

pub type TMMoveRequest = BaseRequest<TMMoveRequestType, TMMoveRequestAttributes>;

impl TMMoveRequest {
    pub fn get_move_id(&self) -> u8 {
        self.data.attributes._move.id
    }
}

#[derive(Debug, Deserialize, JsonSchema)]
#[allow(non_camel_case_types)]
pub enum TMMoveRequestType {
    tm_moves,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct TMMoveRequestAttributes {
    #[serde(rename = "move")]
    pub _move: TMMoveRequestAttributesMove,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct TMMoveRequestAttributesMove {
    #[serde(deserialize_with = "crate::utils::from_numeric_str")]
    pub id: u8,
}
