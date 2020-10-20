use pkmnapi_db::{PokemonName, Trade};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::responses::base::{BaseResponse, BaseResponseAll, BaseResponseData, BaseResponseType};
use crate::responses::links::Links;
use crate::responses::pokemon_names::PokemonNameResponseData;
use crate::utils;

pub type TradeResponse = BaseResponse<TradeResponseAttributes>;
pub type TradeResponseData = BaseResponseData<TradeResponseAttributes>;
pub type TradeResponseAll = BaseResponseAll<TradeResponseData>;

impl TradeResponseAll {
    pub fn new(
        trade_ids: &Vec<u8>,
        trades: &HashMap<u8, Trade>,
        pokemon_names: &HashMap<u8, PokemonName>,
    ) -> TradeResponseAll {
        TradeResponseAll {
            data: trade_ids
                .iter()
                .map(|trade_id| {
                    TradeResponseData::new(&trade_id, &trades.get(trade_id).unwrap(), pokemon_names)
                })
                .collect(),
            links: Links {
                _self: utils::generate_url("trades", None),
            },
        }
    }
}

impl TradeResponse {
    pub fn new(
        trade_id: &u8,
        trade: &Trade,
        pokemon_names: &HashMap<u8, PokemonName>,
    ) -> TradeResponse {
        TradeResponse {
            data: TradeResponseData::new(trade_id, trade, pokemon_names),
            links: Links {
                _self: utils::generate_url("trades", Some(&trade_id.to_string())),
            },
        }
    }
}

impl TradeResponseData {
    pub fn new(
        trade_id: &u8,
        trade: &Trade,
        pokemon_names: &HashMap<u8, PokemonName>,
    ) -> TradeResponseData {
        BaseResponseData {
            id: trade_id.to_string(),
            _type: BaseResponseType::trades,
            attributes: TradeResponseAttributes {
                give: PokemonNameResponseData::new(
                    &trade.give_pokedex_id,
                    pokemon_names.get(&trade.give_pokedex_id).unwrap(),
                ),
                get: PokemonNameResponseData::new(
                    &trade.get_pokedex_id,
                    pokemon_names.get(&trade.get_pokedex_id).unwrap(),
                ),
                nickname: trade.nickname.to_string(),
            },
            links: Links {
                _self: utils::generate_url("trades", Some(&trade_id.to_string())),
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TradeResponseAttributes {
    pub give: PokemonNameResponseData,
    pub get: PokemonNameResponseData,
    pub nickname: String,
}
