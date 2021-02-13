use pkmnapi_db::{Party, PokemonName};
use rocket_okapi::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::responses::base::{BaseResponse, BaseResponseAll, BaseResponseData, BaseResponseType};
use crate::responses::links::Links;
use crate::responses::pokemon_names::PokemonNameResponseData;
use crate::utils;

pub type TrainerPartiesResponse = BaseResponse<TrainerPartiesResponseAttributes>;
pub type TrainerPartiesResponseData = BaseResponseData<TrainerPartiesResponseAttributes>;
pub type TrainerPartiesResponseAll = BaseResponseAll<TrainerPartiesResponseData>;

impl TrainerPartiesResponseAll {
    pub fn new(
        trainer_ids: &Vec<u8>,
        trainer_parties: &HashMap<u8, Vec<Party>>,
        pokemon_names: &HashMap<u8, PokemonName>,
    ) -> TrainerPartiesResponseAll {
        TrainerPartiesResponseAll {
            data: trainer_ids
                .iter()
                .map(|trainer_id| {
                    TrainerPartiesResponseData::new(
                        trainer_id,
                        trainer_parties.get(trainer_id).unwrap(),
                        pokemon_names,
                    )
                })
                .collect(),
            links: Links {
                _self: utils::generate_url("trainers/parties", None),
            },
        }
    }
}

impl TrainerPartiesResponse {
    pub fn new(
        trainer_id: &u8,
        trainer_parties: &Vec<Party>,
        pokemon_names: &HashMap<u8, PokemonName>,
    ) -> TrainerPartiesResponse {
        TrainerPartiesResponse {
            data: TrainerPartiesResponseData::new(trainer_id, trainer_parties, pokemon_names),
            links: Links {
                _self: utils::generate_url("trainers/parties", Some(&trainer_id.to_string())),
            },
        }
    }
}

impl TrainerPartiesResponseData {
    pub fn new(
        trainer_id: &u8,
        trainer_parties: &Vec<Party>,
        pokemon_names: &HashMap<u8, PokemonName>,
    ) -> TrainerPartiesResponseData {
        BaseResponseData {
            id: trainer_id.to_string(),
            _type: BaseResponseType::trainer_parties,
            attributes: TrainerPartiesResponseAttributes {
                parties: trainer_parties
                    .iter()
                    .map(|trainer_party| TrainerPartiesResponseAttributesParty {
                        party: trainer_party
                            .pokemon
                            .iter()
                            .map(
                                |party_pokemon| TrainerPartiesResponseAttributesPartyPokemon {
                                    level: party_pokemon.level,
                                    pokemon: PokemonNameResponseData::new(
                                        &party_pokemon.pokedex_id,
                                        &pokemon_names.get(&party_pokemon.pokedex_id).unwrap(),
                                    ),
                                },
                            )
                            .collect(),
                    })
                    .collect(),
            },
            links: Links {
                _self: utils::generate_url("trainers/parties", Some(&trainer_id.to_string())),
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TrainerPartiesResponseAttributes {
    pub parties: Vec<TrainerPartiesResponseAttributesParty>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TrainerPartiesResponseAttributesParty {
    pub party: Vec<TrainerPartiesResponseAttributesPartyPokemon>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TrainerPartiesResponseAttributesPartyPokemon {
    pub level: u8,
    pub pokemon: PokemonNameResponseData,
}
