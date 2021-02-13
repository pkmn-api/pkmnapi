use rocket_okapi::JsonSchema;
use serde::Deserialize;

use crate::requests::base::BaseRequest;

pub type PlayerNamesRequest = BaseRequest<PlayerNamesRequestType, PlayerNamesRequestAttributes>;

impl PlayerNamesRequest {
    pub fn get_player_names(&self) -> &Vec<String> {
        &self.data.attributes.player
    }

    pub fn get_rival_names(&self) -> &Vec<String> {
        &self.data.attributes.rival
    }
}

#[derive(Debug, Deserialize, JsonSchema)]
#[allow(non_camel_case_types)]
pub enum PlayerNamesRequestType {
    player_names,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct PlayerNamesRequestAttributes {
    pub player: Vec<String>,
    pub rival: Vec<String>,
}
