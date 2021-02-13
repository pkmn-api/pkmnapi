use rocket_okapi::JsonSchema;
use serde::Deserialize;

use crate::requests::base::BaseRequest;

pub type TrainerNameRequest = BaseRequest<TrainerNameRequestType, TrainerNameRequestAttributes>;

impl TrainerNameRequest {
    pub fn get_name(&self) -> &String {
        &self.data.attributes.name
    }
}

#[derive(Debug, Deserialize, JsonSchema)]
#[allow(non_camel_case_types)]
pub enum TrainerNameRequestType {
    trainer_names,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct TrainerNameRequestAttributes {
    pub name: String,
}
