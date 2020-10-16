use pkmnapi_db::PokemonIcon;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::responses::base::{BaseResponse, BaseResponseAll, BaseResponseData, BaseResponseType};
use crate::responses::links::Links;
use crate::utils;

pub type PokemonIconResponse = BaseResponse<PokemonIconResponseAttributes>;
pub type PokemonIconResponseData = BaseResponseData<PokemonIconResponseAttributes>;
pub type PokemonIconResponseAll = BaseResponseAll<PokemonIconResponseData>;

impl PokemonIconResponseAll {
    pub fn new(
        pokedex_ids: &Vec<u8>,
        pokemon_icons: &HashMap<u8, PokemonIcon>,
    ) -> PokemonIconResponseAll {
        PokemonIconResponseAll {
            data: pokedex_ids
                .iter()
                .map(|pokedex_id| {
                    PokemonIconResponseData::new(pokedex_id, pokemon_icons.get(pokedex_id).unwrap())
                })
                .collect(),
            links: Links {
                _self: utils::generate_url("pokemon/icons", None),
            },
        }
    }
}

impl PokemonIconResponse {
    pub fn new(pokedex_id: &u8, pokemon_icon: &PokemonIcon) -> PokemonIconResponse {
        PokemonIconResponse {
            data: PokemonIconResponseData::new(pokedex_id, pokemon_icon),
            links: Links {
                _self: utils::generate_url("pokemon/icons", Some(&pokedex_id.to_string())),
            },
        }
    }
}

impl PokemonIconResponseData {
    pub fn new(pokedex_id: &u8, pokemon_icon: &PokemonIcon) -> PokemonIconResponseData {
        BaseResponseData {
            id: pokedex_id.to_string(),
            _type: BaseResponseType::pokemon_icons,
            attributes: PokemonIconResponseAttributes {
                icon: PokemonIconResponseAttributesIcon {
                    id: pokemon_icon.icon_id.to_string(),
                    _type: BaseResponseType::icons,
                    attributes: PokemonIconResponseAttributesIconAttributes {},
                    links: Links {
                        _self: utils::generate_url(
                            "icons",
                            Some(&pokemon_icon.icon_id.to_string()),
                        ),
                    },
                },
            },
            links: Links {
                _self: utils::generate_url("pokemon/icons", Some(&pokedex_id.to_string())),
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PokemonIconResponseAttributes {
    icon: PokemonIconResponseAttributesIcon,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PokemonIconResponseAttributesIcon {
    id: String,

    #[serde(rename = "type")]
    _type: BaseResponseType,

    attributes: PokemonIconResponseAttributesIconAttributes,
    links: Links,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PokemonIconResponseAttributesIconAttributes {}
