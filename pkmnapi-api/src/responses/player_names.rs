use pkmnapi_db::PlayerNames;
use rocket_okapi::JsonSchema;
use serde::Serialize;

use crate::responses::base::{BaseResponse, BaseResponseData, BaseResponseType};
use crate::responses::links::Links;
use crate::utils;

pub type PlayerNamesResponse = BaseResponse<PlayerNamesResponseAttributes>;

impl PlayerNamesResponse {
    pub fn new(player_names: &PlayerNames) -> PlayerNamesResponse {
        PlayerNamesResponse {
            data: BaseResponseData {
                id: "0".to_owned(),
                _type: BaseResponseType::player_names,
                attributes: PlayerNamesResponseAttributes {
                    player: player_names
                        .player
                        .iter()
                        .map(|name| name.to_string())
                        .collect(),
                    rival: player_names
                        .rival
                        .iter()
                        .map(|name| name.to_string())
                        .collect(),
                },
                links: Links {
                    _self: utils::generate_url("player_names", None),
                },
            },
            links: Links {
                _self: utils::generate_url("player_names", None),
            },
        }
    }
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct PlayerNamesResponseAttributes {
    player: Vec<String>,
    rival: Vec<String>,
}
