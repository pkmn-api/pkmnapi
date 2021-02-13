use pkmnapi_db::TMPrice;
use rocket_okapi::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::responses::base::{BaseResponse, BaseResponseAll, BaseResponseData, BaseResponseType};
use crate::responses::links::Links;
use crate::utils;

pub type TMPriceResponse = BaseResponse<TMPriceResponseAttributes>;
pub type TMPriceResponseData = BaseResponseData<TMPriceResponseAttributes>;
pub type TMPriceResponseAll = BaseResponseAll<TMPriceResponseData>;

impl TMPriceResponseAll {
    pub fn new(tm_ids: &Vec<u8>, tm_prices: &HashMap<u8, TMPrice>) -> TMPriceResponseAll {
        TMPriceResponseAll {
            data: tm_ids
                .iter()
                .map(|tm_id| TMPriceResponseData::new(tm_id, tm_prices.get(tm_id).unwrap()))
                .collect(),
            links: Links {
                _self: utils::generate_url("tms/prices", None),
            },
        }
    }
}

impl TMPriceResponse {
    pub fn new(tm_id: &u8, tm_price: &TMPrice) -> TMPriceResponse {
        TMPriceResponse {
            data: TMPriceResponseData::new(tm_id, tm_price),
            links: Links {
                _self: utils::generate_url("tms/prices", Some(&tm_id.to_string())),
            },
        }
    }
}

impl TMPriceResponseData {
    pub fn new(tm_id: &u8, tm_price: &TMPrice) -> TMPriceResponseData {
        BaseResponseData {
            id: tm_id.to_string(),
            _type: BaseResponseType::tm_prices,
            attributes: TMPriceResponseAttributes {
                price: tm_price.value,
            },
            links: Links {
                _self: utils::generate_url("tms/prices", Some(&tm_id.to_string())),
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TMPriceResponseAttributes {
    pub price: u32,
}
