use pkmnapi_db::types::{
    PokemonEvolution, PokemonEvolutionItem, PokemonEvolutionLevel, PokemonEvolutionTrade,
};
use serde::de::{self, Deserializer};
use serde::Deserialize;
use std::fmt::Display;
use std::str::FromStr;

use crate::requests::base::BaseRequest;

pub type PokemonEvolutionsRequest =
    BaseRequest<PokemonEvolutionsRequestType, PokemonEvolutionsRequestAttributes>;

impl PokemonEvolutionsRequest {
    pub fn get_evolutions(&self) -> Vec<PokemonEvolution> {
        self.data
            .attributes
            .evolutions
            .iter()
            .map(|pokemon_evolution| match pokemon_evolution {
                evolution::LEVEL(evolution) => {
                    PokemonEvolutionLevel::new(evolution.pokemon.id, evolution.level)
                }
                evolution::ITEM(evolution) => {
                    PokemonEvolutionItem::new(evolution.pokemon.id, evolution.item.id)
                }
                evolution::TRADE(evolution) => PokemonEvolutionTrade::new(evolution.pokemon.id),
            })
            .collect()
    }
}

#[derive(Debug, Deserialize)]
#[allow(non_camel_case_types)]
pub enum PokemonEvolutionsRequestType {
    pokemon_evolutions,
}

#[derive(Debug, Deserialize)]
pub struct PokemonEvolutionsRequestAttributes {
    pub evolutions: Vec<evolution>,
}

#[derive(Debug, Deserialize)]
#[allow(non_camel_case_types)]
#[serde(untagged)]
pub enum evolution {
    LEVEL(PokemonEvolutionsRequestAttributesEvolutionLevel),
    ITEM(PokemonEvolutionsRequestAttributesEvolutionItem),
    TRADE(PokemonEvolutionsRequestAttributesEvolutionTrade),
}

#[derive(Debug, Deserialize)]
pub struct PokemonEvolutionsRequestAttributesEvolutionLevel {
    pub evolution_type: PokemonEvolutionsRequestAttributesEvolutionLevelType,
    pub level: u8,
    pub pokemon: PokemonEvolutionsRequestAttributesEvolutionPokemon,
}

#[derive(Debug, Deserialize)]
#[allow(non_camel_case_types)]
pub enum PokemonEvolutionsRequestAttributesEvolutionLevelType {
    level,
}

#[derive(Debug, Deserialize)]
pub struct PokemonEvolutionsRequestAttributesEvolutionItem {
    pub evolution_type: PokemonEvolutionsRequestAttributesEvolutionItemType,
    pub item: PokemonEvolutionsRequestAttributesEvolutionItemAttributes,
    pub pokemon: PokemonEvolutionsRequestAttributesEvolutionPokemon,
}

#[derive(Debug, Deserialize)]
#[allow(non_camel_case_types)]
pub enum PokemonEvolutionsRequestAttributesEvolutionItemType {
    item,
}

#[derive(Debug, Deserialize)]
pub struct PokemonEvolutionsRequestAttributesEvolutionTrade {
    pub evolution_type: PokemonEvolutionsRequestAttributesEvolutionTradeType,
    pub pokemon: PokemonEvolutionsRequestAttributesEvolutionPokemon,
}

#[derive(Debug, Deserialize)]
#[allow(non_camel_case_types)]
pub enum PokemonEvolutionsRequestAttributesEvolutionTradeType {
    trade,
}

#[derive(Debug, Deserialize)]
pub struct PokemonEvolutionsRequestAttributesEvolutionItemAttributes {
    #[serde(deserialize_with = "from_str")]
    id: u8,
}

#[derive(Debug, Deserialize)]
pub struct PokemonEvolutionsRequestAttributesEvolutionPokemon {
    #[serde(deserialize_with = "from_str")]
    id: u8,
}

fn from_str<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: FromStr,
    T::Err: Display,
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    T::from_str(&s).map_err(de::Error::custom)
}
