use pkmnapi_db::{MapPokemon, PokemonName};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::responses::base::{BaseResponse, BaseResponseAll, BaseResponseData, BaseResponseType};
use crate::responses::links::Links;
use crate::responses::pokemon_names::PokemonNameResponseData;
use crate::utils;

pub type MapPokemonResponse = BaseResponse<MapPokemonResponseAttributes>;
pub type MapPokemonResponseData = BaseResponseData<MapPokemonResponseAttributes>;
pub type MapPokemonResponseAll = BaseResponseAll<MapPokemonResponseData>;

impl MapPokemonResponseAll {
    pub fn new(
        map_ids: &Vec<u8>,
        map_pokemon: &HashMap<u8, MapPokemon>,
        pokemon_names: &HashMap<u8, PokemonName>,
    ) -> MapPokemonResponseAll {
        MapPokemonResponseAll {
            data: map_ids
                .iter()
                .map(|map_id| {
                    MapPokemonResponseData::new(
                        map_id,
                        map_pokemon.get(&map_id).unwrap(),
                        pokemon_names,
                    )
                })
                .collect(),
            links: Links {
                _self: utils::generate_url("maps", None),
            },
        }
    }
}

impl MapPokemonResponse {
    pub fn new(
        map_id: &u8,
        map_pokemon: &MapPokemon,
        pokemon_names: &HashMap<u8, PokemonName>,
    ) -> MapPokemonResponse {
        MapPokemonResponse {
            data: MapPokemonResponseData::new(map_id, map_pokemon, pokemon_names),
            links: Links {
                _self: utils::generate_url("maps/pokemon", Some(&map_id.to_string())),
            },
        }
    }
}

impl MapPokemonResponseData {
    pub fn new(
        map_id: &u8,
        map_pokemon: &MapPokemon,
        pokemon_names: &HashMap<u8, PokemonName>,
    ) -> MapPokemonResponseData {
        BaseResponseData {
            id: map_id.to_string(),
            _type: BaseResponseType::map_pokemon,
            attributes: MapPokemonResponseAttributes {
                grass: MapPokemonResponseAttributesArea {
                    encounter_rate: map_pokemon.grass.encounter_rate,
                    pokemon: map_pokemon
                        .grass
                        .pokemon
                        .iter()
                        .map(|pokemon| MapPokemonResponseAttributesPokemon {
                            level: pokemon.level,
                            pokemon: PokemonNameResponseData::new(
                                &pokemon.pokedex_id,
                                &pokemon_names.get(&pokemon.pokedex_id).unwrap(),
                            ),
                        })
                        .collect(),
                },
                water: MapPokemonResponseAttributesArea {
                    encounter_rate: map_pokemon.water.encounter_rate,
                    pokemon: map_pokemon
                        .water
                        .pokemon
                        .iter()
                        .map(|pokemon| MapPokemonResponseAttributesPokemon {
                            level: pokemon.level,
                            pokemon: PokemonNameResponseData::new(
                                &pokemon.pokedex_id,
                                &pokemon_names.get(&pokemon.pokedex_id).unwrap(),
                            ),
                        })
                        .collect(),
                },
            },
            links: Links {
                _self: utils::generate_url("maps/pokemon", Some(&map_id.to_string())),
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapPokemonResponseAttributes {
    pub grass: MapPokemonResponseAttributesArea,
    pub water: MapPokemonResponseAttributesArea,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapPokemonResponseAttributesArea {
    pub encounter_rate: u8,
    pub pokemon: Vec<MapPokemonResponseAttributesPokemon>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapPokemonResponseAttributesPokemon {
    pub level: u8,
    pub pokemon: PokemonNameResponseData,
}
