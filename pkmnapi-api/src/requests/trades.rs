use serde::Deserialize;

use crate::requests::base::BaseRequest;

pub type TradeRequest = BaseRequest<TradeRequestType, TradeRequestAttributes>;

impl TradeRequest {
    pub fn get_give_pokedex_id(&self) -> u8 {
        self.data.attributes.give.id
    }

    pub fn get_get_pokedex_id(&self) -> u8 {
        self.data.attributes.get.id
    }
    pub fn get_nickname(&self) -> &String {
        &self.data.attributes.nickname
    }
}

#[derive(Debug, Deserialize)]
#[allow(non_camel_case_types)]
pub enum TradeRequestType {
    trades,
}

#[derive(Debug, Deserialize)]
pub struct TradeRequestAttributes {
    pub give: TradeRequestAttributesPokemon,
    pub get: TradeRequestAttributesPokemon,
    pub nickname: String,
}

#[derive(Debug, Deserialize)]
pub struct TradeRequestAttributesPokemon {
    #[serde(deserialize_with = "crate::utils::from_numeric_str")]
    pub id: u8,
}
