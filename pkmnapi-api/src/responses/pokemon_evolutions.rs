use pkmnapi_db::{ItemName, PokemonEvolution, PokemonName};
use serde::Serialize;
use std::collections::HashMap;

use crate::responses::base::{BaseResponse, BaseResponseData, BaseResponseType};
use crate::responses::item_names::ItemNameResponseData;
use crate::responses::links::Links;
use crate::responses::pokemon_names::PokemonNameResponseData;
use crate::utils;

pub type PokemonEvolutionsResponse = BaseResponse<PokemonEvolutionsResponseAttributes>;

impl PokemonEvolutionsResponse {
    /// Create a new `PokemonEvolutionsResponse`
    pub fn new(
        pokedex_id: &u8,
        pokemon_evolutions: &Vec<PokemonEvolution>,
        pokemon_names: HashMap<u8, PokemonName>,
        item_names: HashMap<u8, ItemName>,
    ) -> PokemonEvolutionsResponse {
        PokemonEvolutionsResponse {
            data: BaseResponseData {
                id: pokedex_id.to_string(),
                _type: BaseResponseType::pokemon_evolutions,
                attributes: PokemonEvolutionsResponseAttributes {
                    evolutions: pokemon_evolutions
                        .iter()
                        .map(|pokemon_evolution| {
                            PokemonEvolutionsResponseAttributesEvolution::new(
                                pokemon_evolution,
                                &pokemon_names,
                                &item_names,
                            )
                        })
                        .collect(),
                },
                links: Links {
                    _self: utils::generate_url("pokemon/evolutions", Some(&pokedex_id.to_string())),
                },
            },
            links: Links {
                _self: utils::generate_url("pokemon/evolutions", Some(&pokedex_id.to_string())),
            },
        }
    }
}

#[derive(Debug, Serialize)]
pub struct PokemonEvolutionsResponseAttributes {
    pub evolutions: Vec<PokemonEvolutionsResponseAttributesEvolution>,
}

#[derive(Debug, Serialize)]
pub struct PokemonEvolutionsResponseAttributesEvolution {
    pub evolution_type: PokemonEvolutionsResponseAttributesEvolutionType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<ItemNameResponseData>,
    pub pokemon: PokemonNameResponseData,
}

impl PokemonEvolutionsResponseAttributesEvolution {
    pub fn new(
        pokemon_evolution: &PokemonEvolution,
        pokemon_names: &HashMap<u8, PokemonName>,
        item_names: &HashMap<u8, ItemName>,
    ) -> Self {
        match pokemon_evolution {
            PokemonEvolution::LEVEL(evolution) => PokemonEvolutionsResponseAttributesEvolution {
                evolution_type: PokemonEvolutionsResponseAttributesEvolutionType::level,
                level: Some(evolution.level),
                item: None,
                pokemon: PokemonNameResponseData::new(
                    &evolution.pokedex_id,
                    &pokemon_names.get(&evolution.pokedex_id).unwrap(),
                ),
            },
            PokemonEvolution::ITEM(evolution) => PokemonEvolutionsResponseAttributesEvolution {
                evolution_type: PokemonEvolutionsResponseAttributesEvolutionType::item,
                level: None,
                item: Some(ItemNameResponseData::new(
                    &evolution.item_id,
                    &item_names.get(&evolution.item_id).unwrap(),
                )),
                pokemon: PokemonNameResponseData::new(
                    &evolution.pokedex_id,
                    &pokemon_names.get(&evolution.pokedex_id).unwrap(),
                ),
            },
            PokemonEvolution::TRADE(evolution) => PokemonEvolutionsResponseAttributesEvolution {
                evolution_type: PokemonEvolutionsResponseAttributesEvolutionType::trade,
                level: None,
                item: None,
                pokemon: PokemonNameResponseData::new(
                    &evolution.pokedex_id,
                    &pokemon_names.get(&evolution.pokedex_id).unwrap(),
                ),
            },
        }
    }
}

#[derive(Debug, Serialize)]
#[allow(non_camel_case_types)]
pub enum PokemonEvolutionsResponseAttributesEvolutionType {
    level,
    item,
    trade,
}
