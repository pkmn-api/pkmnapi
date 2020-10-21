use pkmnapi_db::MartItem;
use serde::Deserialize;

use crate::requests::base::BaseRequest;

pub type MartItemsRequest = BaseRequest<MartItemsRequestType, MartItemsRequestAttributes>;

impl MartItemsRequest {
    pub fn get_mart_items(&self) -> Vec<MartItem> {
        self.data
            .attributes
            .mart_items
            .iter()
            .map(|mart_item| match mart_item._type {
                MartItemsRequestAttributesItemType::item_names => MartItem::ITEM(mart_item.id),
                MartItemsRequestAttributesItemType::tm_names => MartItem::TM(mart_item.id),
            })
            .collect()
    }
}

#[derive(Debug, Deserialize)]
#[allow(non_camel_case_types)]
pub enum MartItemsRequestType {
    mart_items,
}

#[derive(Debug, Deserialize)]
pub struct MartItemsRequestAttributes {
    mart_items: Vec<MartItemsRequestAttributesItem>,
}

#[derive(Debug, Deserialize)]
pub struct MartItemsRequestAttributesItem {
    #[serde(deserialize_with = "crate::utils::from_numeric_str")]
    pub id: u8,

    #[serde(rename = "type")]
    pub _type: MartItemsRequestAttributesItemType,
}

#[derive(Debug, Deserialize)]
#[allow(non_camel_case_types)]
pub enum MartItemsRequestAttributesItemType {
    item_names,
    tm_names,
}
