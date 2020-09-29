use pkmnapi_db::types::PokedexText;
use serde::Serialize;

use crate::responses::base::{BaseResponse, BaseResponseData, BaseResponseType};
use crate::responses::links::Links;
use crate::utils;

pub type PokedexTextResponse = BaseResponse<PokedexTextResponseAttributes>;

impl PokedexTextResponse {
    /// Create a new `PokedexTextResponse`
    pub fn new(pokedex_id: &u8, pokedex_text: &PokedexText) -> PokedexTextResponse {
        PokedexTextResponse {
            data: BaseResponseData {
                id: pokedex_id.to_string(),
                _type: BaseResponseType::pokedex_texts,
                attributes: PokedexTextResponseAttributes {
                    text: pokedex_text.text.to_string(),
                },
                links: Links {
                    _self: utils::generate_url("pokedex/texts", Some(&pokedex_id.to_string())),
                },
            },
            links: Links {
                _self: utils::generate_url("pokedex/texts", Some(&pokedex_id.to_string())),
            },
        }
    }
}

#[derive(Debug, Serialize)]
pub struct PokedexTextResponseAttributes {
    pub text: String,
}
