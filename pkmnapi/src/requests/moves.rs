use serde::Deserialize;

use crate::requests::base::BaseRequest;

pub type MoveRequest = BaseRequest<MoveRequestType, MoveRequestAttributes>;

impl MoveRequest {
    pub fn get_name(&self) -> &String {
        &self.data.attributes.name
    }
}

#[derive(Debug, Deserialize)]
#[allow(non_camel_case_types)]
pub enum MoveRequestType {
    moves,
}

#[derive(Debug, Deserialize)]
pub struct MoveRequestAttributes {
    pub name: String,
}
