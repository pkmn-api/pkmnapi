use rocket_okapi::JsonSchema;
use serde::Deserialize;

use crate::requests::base::BaseRequest;

pub type ItemNameRequest = BaseRequest<ItemNameRequestType, ItemNameRequestAttributes>;

impl ItemNameRequest {
    pub fn get_name(&self) -> &String {
        &self.data.attributes.name
    }
}

#[derive(Debug, Deserialize, JsonSchema)]
#[allow(non_camel_case_types)]
pub enum ItemNameRequestType {
    item_names,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ItemNameRequestAttributes {
    pub name: String,
}
