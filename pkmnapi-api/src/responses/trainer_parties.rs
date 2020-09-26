use pkmnapi_db::types::{Party, PokemonName};
use serde::Serialize;
use std::collections::HashMap;

use crate::responses::base::{BaseResponse, BaseResponseData, BaseResponseType};
use crate::responses::links::Links;
use crate::responses::pokemon_names::PokemonNameResponseData;
use crate::utils;

pub type TrainerPartiesResponse = BaseResponse<TrainerPartiesResponseAttributes>;

impl TrainerPartiesResponse {
    /// Create a new `TrainerPartiesResponse`
    pub fn new(
        trainer_id: &u8,
        trainer_parties: &Vec<Party>,
        pokemon_names: HashMap<u8, PokemonName>,
    ) -> TrainerPartiesResponse {
        TrainerPartiesResponse {
            data: BaseResponseData {
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
            },
            links: Links {
                _self: utils::generate_url("trainers/parties", Some(&trainer_id.to_string())),
            },
        }
    }
}

#[derive(Debug, Serialize)]
pub struct TrainerPartiesResponseAttributes {
    pub parties: Vec<TrainerPartiesResponseAttributesParty>,
}

#[derive(Debug, Serialize)]
pub struct TrainerPartiesResponseAttributesParty {
    pub party: Vec<TrainerPartiesResponseAttributesPartyPokemon>,
}

#[derive(Debug, Serialize)]
pub struct TrainerPartiesResponseAttributesPartyPokemon {
    pub level: u8,
    pub pokemon: PokemonNameResponseData,
}
