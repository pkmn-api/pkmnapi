use pkmnapi_db::{ItemName, MartItem, MoveName, TM};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::responses::base::{BaseResponse, BaseResponseAll, BaseResponseData, BaseResponseType};
use crate::responses::item_names::ItemNameResponseData;
use crate::responses::links::Links;
use crate::responses::tm_moves::TMMoveResponseData;
use crate::utils;

pub type MartItemsResponse = BaseResponse<MartItemsResponseAttributes>;
pub type MartItemsResponseData = BaseResponseData<MartItemsResponseAttributes>;
pub type MartItemsResponseAll = BaseResponseAll<MartItemsResponseData>;

impl MartItemsResponseAll {
    pub fn new(
        mart_ids: &Vec<u8>,
        mart_items: &HashMap<u8, Vec<MartItem>>,
        item_names: &HashMap<u8, ItemName>,
        tm_moves: &HashMap<u8, TM>,
        move_names: &HashMap<u8, MoveName>,
    ) -> MartItemsResponseAll {
        MartItemsResponseAll {
            data: mart_ids
                .iter()
                .map(|mart_id| {
                    MartItemsResponseData::new(
                        &mart_id,
                        &mart_items.get(mart_id).unwrap(),
                        item_names,
                        tm_moves,
                        move_names,
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
        tm_moves: &HashMap<u8, TM>,
        move_names: &HashMap<u8, MoveName>,
    ) -> MartItemsResponse {
        MartItemsResponse {
            data: MartItemsResponseData::new(mart_id, mart_items, item_names, tm_moves, move_names),
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
        tm_moves: &HashMap<u8, TM>,
        move_names: &HashMap<u8, MoveName>,
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
                            let tm = tm_moves.get(tm_id).unwrap();

                            mart_item::TM(TMMoveResponseData::new(
                                tm_id,
                                tm,
                                move_names.get(&tm.move_id).unwrap(),
                            ))
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

#[derive(Debug, Serialize, Deserialize)]
pub struct MartItemsResponseAttributes {
    pub mart_items: Vec<mart_item>,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[serde(untagged)]
pub enum mart_item {
    ITEM(ItemNameResponseData),
    TM(TMMoveResponseData),
}
