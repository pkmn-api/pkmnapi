use pkmnapi_db::PokemonIcon;
use serde::Serialize;

use crate::responses::base::{BaseResponse, BaseResponseData, BaseResponseType};
use crate::responses::links::Links;
use crate::utils;

pub type PokemonIconResponse = BaseResponse<PokemonIconResponseAttributes>;

impl PokemonIconResponse {
    /// Create a new `PokemonIconResponse`
    pub fn new(pokedex_id: &u8, pokemon_icon: &PokemonIcon) -> PokemonIconResponse {
        PokemonIconResponse {
            data: BaseResponseData {
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
            },
            links: Links {
                _self: utils::generate_url("pokemon/icons", Some(&pokedex_id.to_string())),
            },
        }
    }
}

#[derive(Debug, Serialize)]
pub struct PokemonIconResponseAttributes {
    icon: PokemonIconResponseAttributesIcon,
}

#[derive(Debug, Serialize)]
pub struct PokemonIconResponseAttributesIcon {
    id: String,

    #[serde(rename = "type")]
    _type: BaseResponseType,

    attributes: PokemonIconResponseAttributesIconAttributes,
    links: Links,
}

#[derive(Debug, Serialize)]
pub struct PokemonIconResponseAttributesIconAttributes {}
