use pkmnapi_db::ItemName;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::responses::base::{BaseResponse, BaseResponseAll, BaseResponseData, BaseResponseType};
use crate::responses::links::Links;
use crate::utils;

pub type ItemNameResponse = BaseResponse<ItemNameResponseAttributes>;
pub type ItemNameResponseData = BaseResponseData<ItemNameResponseAttributes>;
pub type ItemNameResponseAll = BaseResponseAll<ItemNameResponseData>;

impl ItemNameResponseAll {
    pub fn new(item_ids: &Vec<u8>, item_names: &HashMap<u8, ItemName>) -> ItemNameResponseAll {
        ItemNameResponseAll {
            data: item_ids
                .iter()
                .map(|item_id| ItemNameResponseData::new(item_id, item_names.get(item_id).unwrap()))
                .collect(),
            links: Links {
                _self: utils::generate_url("items/names", None),
            },
        }
    }
}

impl ItemNameResponse {
    pub fn new(item_id: &u8, item_name: &ItemName) -> ItemNameResponse {
        ItemNameResponse {
            data: ItemNameResponseData::new(item_id, item_name),
            links: Links {
                _self: utils::generate_url("items/names", Some(&item_id.to_string())),
            },
        }
    }
}

impl ItemNameResponseData {
    pub fn new(item_id: &u8, item_name: &ItemName) -> ItemNameResponseData {
        BaseResponseData {
            id: item_id.to_string(),
            _type: BaseResponseType::item_names,
            attributes: ItemNameResponseAttributes {
                name: item_name.name.to_string(),
            },
            links: Links {
                _self: utils::generate_url("items/names", Some(&item_id.to_string())),
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemNameResponseAttributes {
    pub name: String,
}
