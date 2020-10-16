use pkmnapi_db::PokedexEntry;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::responses::base::{BaseResponse, BaseResponseAll, BaseResponseData, BaseResponseType};
use crate::responses::links::Links;
use crate::utils;

pub type PokedexEntryResponse = BaseResponse<PokedexEntryResponseAttributes>;
pub type PokedexEntryResponseData = BaseResponseData<PokedexEntryResponseAttributes>;
pub type PokedexEntryResponseAll = BaseResponseAll<PokedexEntryResponseData>;

impl PokedexEntryResponseAll {
    pub fn new(
        pokedex_ids: &Vec<u8>,
        pokedex_entries: &HashMap<u8, PokedexEntry>,
    ) -> PokedexEntryResponseAll {
        PokedexEntryResponseAll {
            data: pokedex_ids
                .iter()
                .map(|pokedex_id| {
                    PokedexEntryResponseData::new(
                        pokedex_id,
                        pokedex_entries.get(pokedex_id).unwrap(),
                    )
                })
                .collect(),
            links: Links {
                _self: utils::generate_url("pokedex/entries", None),
            },
        }
    }
}

impl PokedexEntryResponse {
    pub fn new(pokedex_id: &u8, pokedex_entry: &PokedexEntry) -> PokedexEntryResponse {
        PokedexEntryResponse {
            data: PokedexEntryResponseData::new(pokedex_id, pokedex_entry),
            links: Links {
                _self: utils::generate_url("pokedex/entries", Some(&pokedex_id.to_string())),
            },
        }
    }
}

impl PokedexEntryResponseData {
    pub fn new(pokedex_id: &u8, pokedex_entry: &PokedexEntry) -> PokedexEntryResponseData {
        BaseResponseData {
            id: pokedex_id.to_string(),
            _type: BaseResponseType::pokedex_entries,
            attributes: PokedexEntryResponseAttributes {
                species: pokedex_entry.species.to_string(),
                height: pokedex_entry.height,
                weight: pokedex_entry.weight,
            },
            links: Links {
                _self: utils::generate_url("pokedex/entries", Some(&pokedex_id.to_string())),
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PokedexEntryResponseAttributes {
    pub species: String,
    pub height: u32,
    pub weight: u32,
}
