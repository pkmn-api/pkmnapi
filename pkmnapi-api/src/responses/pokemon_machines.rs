use pkmnapi_db::{MoveName, PokemonMachine, HM, TM};
use serde::Serialize;
use std::collections::HashMap;

use crate::responses::base::{BaseResponse, BaseResponseData, BaseResponseType};
use crate::responses::hm_moves::HMMoveResponseData;
use crate::responses::links::Links;
use crate::responses::tm_moves::TMMoveResponseData;
use crate::utils;

pub type PokemonMachinesResponse = BaseResponse<PokemonMachinesResponseAttributes>;

impl PokemonMachinesResponse {
    /// Create a new `PokemonMachinesResponse`
    pub fn new(
        pokedex_id: &u8,
        pokemon_machines: &Vec<PokemonMachine>,
        tm_moves: &HashMap<u8, TM>,
        hm_moves: &HashMap<u8, HM>,
        move_names: &HashMap<u8, MoveName>,
    ) -> PokemonMachinesResponse {
        PokemonMachinesResponse {
            data: BaseResponseData {
                id: pokedex_id.to_string(),
                _type: BaseResponseType::pokemon_machines,
                attributes: PokemonMachinesResponseAttributes {
                    machines: pokemon_machines
                        .iter()
                        .map(|machine| match machine {
                            PokemonMachine::TM(tm_id) => {
                                let tm_move = tm_moves.get(&tm_id).unwrap();
                                let move_name = move_names.get(&tm_move.move_id).unwrap();

                                PokemonMachinesResponseAttributesMachine::TM(
                                    TMMoveResponseData::new(&tm_id, &tm_move, &move_name),
                                )
                            }
                            PokemonMachine::HM(hm_id) => {
                                let hm_move = hm_moves.get(&hm_id).unwrap();
                                let move_name = move_names.get(&hm_move.move_id).unwrap();

                                PokemonMachinesResponseAttributesMachine::HM(
                                    HMMoveResponseData::new(&hm_id, &hm_move, &move_name),
                                )
                            }
                        })
                        .collect(),
                },
                links: Links {
                    _self: utils::generate_url("pokemon/machines", Some(&pokedex_id.to_string())),
                },
            },
            links: Links {
                _self: utils::generate_url("pokemon/machines", Some(&pokedex_id.to_string())),
            },
        }
    }
}

#[derive(Debug, Serialize)]
pub struct PokemonMachinesResponseAttributes {
    pub machines: Vec<PokemonMachinesResponseAttributesMachine>,
}

#[derive(Debug, Serialize)]
#[allow(non_camel_case_types)]
#[serde(untagged)]
pub enum PokemonMachinesResponseAttributesMachine {
    TM(TMMoveResponseData),
    HM(HMMoveResponseData),
}
