use serde::Deserialize;

use crate::requests::base::BaseRequest;

pub type SavPlayerNameRequest =
    BaseRequest<SavPlayerNameRequestType, SavPlayerNameRequestAttributes>;

impl SavPlayerNameRequest {
    pub fn get_name(&self) -> &String {
        &self.data.attributes.name
    }
}

#[derive(Debug, Deserialize)]
#[allow(non_camel_case_types)]
pub enum SavPlayerNameRequestType {
    sav_player_names,
}

#[derive(Debug, Deserialize)]
pub struct SavPlayerNameRequestAttributes {
    pub name: String,
}
