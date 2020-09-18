use pkmnapi_db::types::SavePlayerName;
use serde::Serialize;

use crate::responses::base::{BaseResponse, BaseResponseData, BaseResponseType};
use crate::responses::links::Links;
use crate::utils;

pub type SavPlayerNameResponse = BaseResponse<SavPlayerNameResponseAttributes>;

impl SavPlayerNameResponse {
    /// Create a new `SavPlayerNameResponse`
    pub fn new(player_id: &u16, player_name: &SavePlayerName) -> SavPlayerNameResponse {
        SavPlayerNameResponse {
            data: BaseResponseData {
                id: player_id.to_string(),
                _type: BaseResponseType::sav_player_names,
                attributes: SavPlayerNameResponseAttributes {
                    name: player_name.name.to_string(),
                },
                links: Links {
                    _self: utils::generate_url("savs/player_names", None),
                },
            },
            links: Links {
                _self: utils::generate_url("savs/player_names", None),
            },
        }
    }
}

#[derive(Debug, Serialize)]
pub struct SavPlayerNameResponseAttributes {
    name: String,
}
