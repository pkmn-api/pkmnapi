use pkmnapi_db::types::ItemName;
use serde::Serialize;

use crate::responses::base::{BaseResponse, BaseResponseData, BaseResponseType};
use crate::responses::links::Links;
use crate::utils;

pub type ItemNameResponse = BaseResponse<ItemNameResponseAttributes>;
pub type ItemNameResponseData = BaseResponseData<ItemNameResponseAttributes>;

impl ItemNameResponse {
    /// Create a new `ItemNameResponse`
    pub fn new(pokedex_id: &u8, item_name: &ItemName) -> ItemNameResponse {
        ItemNameResponse {
            data: ItemNameResponseData::new(pokedex_id, item_name),
            links: Links {
                _self: utils::generate_url("items/names", Some(&pokedex_id.to_string())),
            },
        }
    }
}

impl ItemNameResponseData {
    pub fn new(pokedex_id: &u8, item_name: &ItemName) -> ItemNameResponseData {
        BaseResponseData {
            id: pokedex_id.to_string(),
            _type: BaseResponseType::item_names,
            attributes: ItemNameResponseAttributes {
                name: item_name.name.to_string(),
            },
            links: Links {
                _self: utils::generate_url("items/names", Some(&pokedex_id.to_string())),
            },
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ItemNameResponseAttributes {
    pub name: String,
}
