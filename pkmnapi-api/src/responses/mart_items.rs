use pkmnapi_db::{ItemName, MartItem, TMName};
use rocket_okapi::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::responses::base::{BaseResponse, BaseResponseAll, BaseResponseData, BaseResponseType};
use crate::responses::item_names::ItemNameResponseData;
use crate::responses::links::Links;
use crate::responses::tm_names::TMNameResponseData;
use crate::utils;

pub type MartItemsResponse = BaseResponse<MartItemsResponseAttributes>;
pub type MartItemsResponseData = BaseResponseData<MartItemsResponseAttributes>;
pub type MartItemsResponseAll = BaseResponseAll<MartItemsResponseData>;

impl MartItemsResponseAll {
    pub fn new(
        mart_ids: &Vec<u8>,
        mart_items: &HashMap<u8, Vec<MartItem>>,
        item_names: &HashMap<u8, ItemName>,
        tm_names: &HashMap<u8, TMName>,
    ) -> MartItemsResponseAll {
        MartItemsResponseAll {
            data: mart_ids
                .iter()
                .map(|mart_id| {
                    MartItemsResponseData::new(
                        &mart_id,
                        &mart_items.get(mart_id).unwrap(),
                        item_names,
                        tm_names,
                    )
                })
                .collect(),
            links: Links {
                _self: utils::generate_url("marts/items", None),
            },
        }
    }
}

impl MartItemsResponse {
    pub fn new(
        mart_id: &u8,
        mart_items: &Vec<MartItem>,
        item_names: &HashMap<u8, ItemName>,
        tm_names: &HashMap<u8, TMName>,
    ) -> MartItemsResponse {
        MartItemsResponse {
            data: MartItemsResponseData::new(mart_id, mart_items, item_names, tm_names),
            links: Links {
                _self: utils::generate_url("marts/items", Some(&mart_id.to_string())),
            },
        }
    }
}

impl MartItemsResponseData {
    pub fn new(
        mart_id: &u8,
        mart_items: &Vec<MartItem>,
        item_names: &HashMap<u8, ItemName>,
        tm_names: &HashMap<u8, TMName>,
    ) -> MartItemsResponseData {
        BaseResponseData {
            id: mart_id.to_string(),
            _type: BaseResponseType::mart_items,
            attributes: MartItemsResponseAttributes {
                mart_items: mart_items
                    .iter()
                    .map(|_mart_item| match _mart_item {
                        MartItem::ITEM(item_id) => mart_item::ITEM(ItemNameResponseData::new(
                            item_id,
                            item_names.get(item_id).unwrap(),
                        )),
                        MartItem::TM(tm_id) => {
                            let tm_name = tm_names.get(tm_id).unwrap();

                            mart_item::TM(TMNameResponseData::new(tm_id, tm_name))
                        }
                    })
                    .collect(),
            },
            links: Links {
                _self: utils::generate_url("marts/items", Some(&mart_id.to_string())),
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct MartItemsResponseAttributes {
    pub mart_items: Vec<mart_item>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[allow(non_camel_case_types)]
#[serde(untagged)]
pub enum mart_item {
    ITEM(ItemNameResponseData),
    TM(TMNameResponseData),
}
