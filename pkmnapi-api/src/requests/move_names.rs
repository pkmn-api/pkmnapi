use rocket_okapi::JsonSchema;
use serde::Deserialize;

use crate::requests::base::BaseRequest;

pub type MoveNameRequest = BaseRequest<MoveNameRequestType, MoveNameRequestAttributes>;

impl MoveNameRequest {
    pub fn get_name(&self) -> &String {
        &self.data.attributes.name
    }
}

#[derive(Debug, Deserialize, JsonSchema)]
#[allow(non_camel_case_types)]
pub enum MoveNameRequestType {
    move_names,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct MoveNameRequestAttributes {
    pub name: String,
}
