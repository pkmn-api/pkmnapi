use rocket_okapi::JsonSchema;
use serde::Deserialize;

use crate::requests::base::BaseRequest;

pub type TMPriceRequest = BaseRequest<TMPriceRequestType, TMPriceRequestAttributes>;

impl TMPriceRequest {
    pub fn get_price(&self) -> u32 {
        self.data.attributes.price
    }
}

#[derive(Debug, Deserialize, JsonSchema)]
#[allow(non_camel_case_types)]
pub enum TMPriceRequestType {
    tm_prices,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct TMPriceRequestAttributes {
    pub price: u32,
}
