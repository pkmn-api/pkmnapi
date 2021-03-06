use pkmnapi_db::{HMMove, MoveName, PokemonMachine, TMMove};
use rocket_okapi::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::responses::base::{BaseResponse, BaseResponseAll, BaseResponseData, BaseResponseType};
use crate::responses::hm_moves::HMMoveResponseData;
use crate::responses::links::Links;
use crate::responses::tm_moves::TMMoveResponseData;
use crate::utils;

pub type PokemonMachinesResponse = BaseResponse<PokemonMachinesResponseAttributes>;
pub type PokemonMachinesResponseData = BaseResponseData<PokemonMachinesResponseAttributes>;
pub type PokemonMachinesResponseAll = BaseResponseAll<PokemonMachinesResponseData>;

impl PokemonMachinesResponseAll {
    pub fn new(
        pokedex_ids: &Vec<u8>,
        pokemon_machines: &HashMap<u8, Vec<PokemonMachine>>,
        tm_moves: &HashMap<u8, TMMove>,
        hm_moves: &HashMap<u8, HMMove>,
        move_names: &HashMap<u8, MoveName>,
    ) -> PokemonMachinesResponseAll {
        PokemonMachinesResponseAll {
            data: pokedex_ids
                .iter()
                .map(|pokedex_id| {
                    PokemonMachinesResponseData::new(
                        pokedex_id,
                        pokemon_machines.get(pokedex_id).unwrap(),
                        tm_moves,
                        hm_moves,
                        move_names,
                    )
                })
                .collect(),
            links: Links {
                _self: utils::generate_url("pokemon/machines", None),
            },
        }
    }
}

impl PokemonMachinesResponse {
    pub fn new(
        pokedex_id: &u8,
        pokemon_machines: &Vec<PokemonMachine>,
        tm_moves: &HashMap<u8, TMMove>,
        hm_moves: &HashMap<u8, HMMove>,
        move_names: &HashMap<u8, MoveName>,
    ) -> PokemonMachinesResponse {
        PokemonMachinesResponse {
            data: PokemonMachinesResponseData::new(
                pokedex_id,
                pokemon_machines,
                tm_moves,
                hm_moves,
                move_names,
            ),
            links: Links {
                _self: utils::generate_url("pokemon/machines", Some(&pokedex_id.to_string())),
            },
        }
    }
}

impl PokemonMachinesResponseData {
    pub fn new(
        pokedex_id: &u8,
        pokemon_machines: &Vec<PokemonMachine>,
        tm_moves: &HashMap<u8, TMMove>,
        hm_moves: &HashMap<u8, HMMove>,
        move_names: &HashMap<u8, MoveName>,
    ) -> PokemonMachinesResponseData {
        BaseResponseData {
            id: pokedex_id.to_string(),
            _type: BaseResponseType::pokemon_machines,
            attributes: PokemonMachinesResponseAttributes {
                machines: pokemon_machines
                    .iter()
                    .map(|machine| match machine {
                        PokemonMachine::TM(tm_id) => {
                            let tm_move = tm_moves.get(&tm_id).unwrap();
                            let move_name = move_names.get(&tm_move.move_id).unwrap();

                            PokemonMachinesResponseAttributesMachine::TM(TMMoveResponseData::new(
                                &tm_id, &tm_move, &move_name,
                            ))
                        }
                        PokemonMachine::HM(hm_id) => {
                            let hm_move = hm_moves.get(&hm_id).unwrap();
                            let move_name = move_names.get(&hm_move.move_id).unwrap();

                            PokemonMachinesResponseAttributesMachine::HM(HMMoveResponseData::new(
                                &hm_id, &hm_move, &move_name,
                            ))
                        }
                    })
                    .collect(),
            },
            links: Links {
                _self: utils::generate_url("pokemon/machines", Some(&pokedex_id.to_string())),
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PokemonMachinesResponseAttributes {
    pub machines: Vec<PokemonMachinesResponseAttributesMachine>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[allow(non_camel_case_types)]
#[serde(untagged)]
pub enum PokemonMachinesResponseAttributesMachine {
    TM(TMMoveResponseData),
    HM(HMMoveResponseData),
}
