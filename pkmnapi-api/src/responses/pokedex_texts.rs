use pkmnapi_db::PokedexText;
use rocket_okapi::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::responses::base::{BaseResponse, BaseResponseAll, BaseResponseData, BaseResponseType};
use crate::responses::links::Links;
use crate::utils;

pub type PokedexTextResponse = BaseResponse<PokedexTextResponseAttributes>;
pub type PokedexTextResponseData = BaseResponseData<PokedexTextResponseAttributes>;
pub type PokedexTextResponseAll = BaseResponseAll<PokedexTextResponseData>;

impl PokedexTextResponseAll {
    pub fn new(
        pokedex_ids: &Vec<u8>,
        pokedex_texts: &HashMap<u8, PokedexText>,
    ) -> PokedexTextResponseAll {
        PokedexTextResponseAll {
            data: pokedex_ids
                .iter()
                .map(|pokedex_id| {
                    PokedexTextResponseData::new(pokedex_id, pokedex_texts.get(pokedex_id).unwrap())
                })
                .collect(),
            links: Links {
                _self: utils::generate_url("pokedex/texts", None),
            },
        }
    }
}

impl PokedexTextResponse {
    pub fn new(pokedex_id: &u8, pokedex_text: &PokedexText) -> PokedexTextResponse {
        PokedexTextResponse {
            data: PokedexTextResponseData::new(pokedex_id, pokedex_text),
            links: Links {
                _self: utils::generate_url("pokedex/texts", Some(&pokedex_id.to_string())),
            },
        }
    }
}

impl PokedexTextResponseData {
    pub fn new(pokedex_id: &u8, pokedex_text: &PokedexText) -> PokedexTextResponseData {
        BaseResponseData {
            id: pokedex_id.to_string(),
            _type: BaseResponseType::pokedex_texts,
            attributes: PokedexTextResponseAttributes {
                text: pokedex_text.text.to_string(),
            },
            links: Links {
                _self: utils::generate_url("pokedex/texts", Some(&pokedex_id.to_string())),
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PokedexTextResponseAttributes {
    pub text: String,
}
