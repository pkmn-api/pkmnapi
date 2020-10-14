use pkmnapi_db::TMPrice;
use serde::Serialize;

use crate::responses::base::{BaseResponse, BaseResponseData, BaseResponseType};
use crate::responses::links::Links;
use crate::utils;

pub type TMPriceResponse = BaseResponse<TMPriceResponseAttributes>;

impl TMPriceResponse {
    /// Create a new `TMPriceResponse`
    pub fn new(tm_id: &u8, tm_price: &TMPrice) -> TMPriceResponse {
        TMPriceResponse {
            data: BaseResponseData {
                id: tm_id.to_string(),
                _type: BaseResponseType::tm_prices,
                attributes: TMPriceResponseAttributes {
                    price: tm_price.value,
                },
                links: Links {
                    _self: utils::generate_url("tms/prices", Some(&tm_id.to_string())),
                },
            },
            links: Links {
                _self: utils::generate_url("tms/prices", Some(&tm_id.to_string())),
            },
        }
    }
}

#[derive(Debug, Serialize)]
pub struct TMPriceResponseAttributes {
    pub price: u32,
}
