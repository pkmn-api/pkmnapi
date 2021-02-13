use rocket_okapi::JsonSchema;
use serde::Deserialize;

use crate::requests::base::BaseRequest;

pub type SavPlayerNameRequest =
    BaseRequest<SavPlayerNameRequestType, SavPlayerNameRequestAttributes>;

impl SavPlayerNameRequest {
    pub fn get_name(&self) -> &String {
        &self.data.attributes.name
    }
}

#[derive(Debug, Deserialize, JsonSchema)]
#[allow(non_camel_case_types)]
pub enum SavPlayerNameRequestType {
    sav_player_names,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SavPlayerNameRequestAttributes {
    pub name: String,
}
