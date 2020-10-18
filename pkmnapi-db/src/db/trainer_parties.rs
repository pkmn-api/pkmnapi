use crate::error::{self, Result};
use crate::patch::*;
use crate::PkmnapiDB;
use std::collections::HashMap;

impl PkmnapiDB {
    pub fn get_trainer_parties_all(
        &self,
        trainer_ids: &Vec<u8>,
    ) -> Result<HashMap<u8, Vec<Party>>> {
        let trainer_parties_all: HashMap<u8, Vec<Party>> = trainer_ids
            .iter()
            .map(|trainer_id| {
                let trainer_parties = self.get_trainer_parties(trainer_id)?;

                Ok((*trainer_id, trainer_parties))
            })
            .collect::<Result<HashMap<u8, Vec<Party>>>>()?;

        Ok(trainer_parties_all)
    }

    pub fn get_trainer_parties(&self, trainer_id: &u8) -> Result<Vec<Party>> {
        let (_min_id, max_id) = self.trainer_id_validate(trainer_id)?;

        let offset_base = PkmnapiDB::ROM_PAGE * 0x1C;
        let offset = offset_base + 0x1D3B;

        let pointer_min_offset = offset + ((*trainer_id as usize) - 1) * 0x02;
        let pointer_min =
            (offset_base - (PkmnapiDB::ROM_PAGE * 2)) + self.get_pointer(pointer_min_offset);

        let pointer_max_offset = offset + (*trainer_id as usize) * 0x02;
        let pointer_max =
            (offset_base - (PkmnapiDB::ROM_PAGE * 2)) + self.get_pointer(pointer_max_offset);

        let data_size = if trainer_id == &(max_id as u8) {
            self.rom[pointer_min..]
                .iter()
                .position(|r| r == &0x00)
                .unwrap()
                + 0x01
        } else {
            pointer_max - pointer_min
        };

        let trainer_party_offsets: Vec<usize> = [
            vec![0x00],
            self.rom[pointer_min..(pointer_min + data_size)]
                .iter()
                .enumerate()
                .filter_map(|(i, x)| {
                    let offset = i + 1;

                    if offset == data_size {
                        return None;
                    }

                    if x == &0x00 {
                        Some(i + 1)
                    } else {
                        None
                    }
                })
                .collect::<Vec<usize>>(),
        ]
        .concat();

        if data_size == 0x00 {
            return Ok(vec![]);
        }

        let trainer_parties: Vec<Party> = trainer_party_offsets
            .iter()
            .map(|trainer_party_offset| {
                let mut party = Party::from(
                    &self.rom[(pointer_min + trainer_party_offset)..(pointer_min + data_size)],
                );

                party.pokemon = party
                    .pokemon
                    .iter()
                    .map(|party_pokemon| {
                        PartyPokemon::new(
                            party_pokemon.level,
                            self.internal_id_to_pokedex_id(&party_pokemon.internal_id)
                                .unwrap(),
                        )
                    })
                    .collect();

                party
            })
            .collect();

        Ok(trainer_parties)
    }

    pub fn set_trainer_parties(
        &self,
        trainer_id: &u8,
        trainer_parties: &Vec<Party>,
    ) -> Result<Patch> {
        let old_trainer_parties = self.get_trainer_parties(trainer_id)?;
        let old_trainer_parties_len = old_trainer_parties.len();
        let old_trainer_parties_data: Vec<u8> = old_trainer_parties
            .iter()
            .map(|old_trainer_party| old_trainer_party.to_raw())
            .flatten()
            .collect();
        let old_trainer_parties_data_len = old_trainer_parties_data.len();
        let trainer_parties_len = trainer_parties.len();
        let trainer_parties_data: Vec<u8> = trainer_parties
            .iter()
            .map(|trainer_party| {
                let new_trainer_party = Party {
                    level_type: trainer_party.level_type,
                    pokemon: trainer_party
                        .pokemon
                        .iter()
                        .map(|pokemon| PartyPokemon {
                            level: pokemon.level,
                            pokedex_id: pokemon.pokedex_id,
                            internal_id: self
                                .pokedex_id_to_internal_id(&pokemon.pokedex_id)
                                .unwrap()
                                + 1,
                        })
                        .collect(),
                };

                new_trainer_party.to_raw()
            })
            .flatten()
            .collect();
        let trainer_parties_data_len = trainer_parties_data.len();

        if old_trainer_parties_len != trainer_parties_len {
            return Err(error::Error::TrainerPartiesWrongSize(
                old_trainer_parties_len,
                trainer_parties_len,
            ));
        }

        if old_trainer_parties_data_len != trainer_parties_data_len {
            return Err(error::Error::TrainerPartiesWrongDataSize(
                old_trainer_parties_data_len,
                trainer_parties_data_len,
            ));
        }

        let offset_base = PkmnapiDB::ROM_PAGE * 0x1C;
        let offset = offset_base + 0x1D3B;

        let pointer_offset = offset + ((*trainer_id as usize) - 1) * 0x02;
        let pointer = (offset_base - (PkmnapiDB::ROM_PAGE * 2)) + self.get_pointer(pointer_offset);

        Ok(Patch::new(&pointer, &trainer_parties_data))
    }
}

#[derive(Debug, PartialEq)]
pub struct Party {
    pub level_type: PartyLevelType,
    pub pokemon: Vec<PartyPokemon>,
}

impl From<&[u8]> for Party {
    fn from(party: &[u8]) -> Self {
        let level_type = match party.iter().next() {
            Some(level) if *level != 0xFF => PartyLevelType::SAME(*level),
            _ => PartyLevelType::DIFFERENT,
        };

        let pokemon: Vec<PartyPokemon> = match level_type {
            PartyLevelType::SAME(level) => party[1..(party.len() - 1)]
                .iter()
                .take_while(|&x| x != &0x00)
                .map(|internal_id| PartyPokemon {
                    level,
                    internal_id: *internal_id - 1,
                    pokedex_id: 0,
                })
                .collect(),
            PartyLevelType::DIFFERENT => party[1..(party.len() - 1)]
                .chunks(2)
                .take_while(|&chunk| chunk[0] != 0x00)
                .map(|chunk| PartyPokemon {
                    level: chunk[0],
                    internal_id: chunk[1] - 1,
                    pokedex_id: 0,
                })
                .collect(),
        };

        Party {
            level_type,
            pokemon,
        }
    }
}

impl Party {
    pub fn new(party_pokemon: &Vec<PartyPokemon>) -> Self {
        let levels: Vec<u8> = party_pokemon.iter().map(|pokemon| pokemon.level).collect();
        let level_min = levels.iter().min().unwrap();
        let level_max = levels.iter().max().unwrap();
        let level_type = if level_min == level_max {
            PartyLevelType::SAME(*level_min)
        } else {
            PartyLevelType::DIFFERENT
        };

        Party {
            level_type,
            pokemon: party_pokemon.to_vec(),
        }
    }

    pub fn to_raw(&self) -> Vec<u8> {
        match self.level_type {
            PartyLevelType::SAME(level) => [
                vec![level],
                self.pokemon
                    .iter()
                    .map(|pokemon| pokemon.internal_id)
                    .collect(),
                vec![0x00],
            ]
            .concat(),
            PartyLevelType::DIFFERENT => [
                vec![0xFF],
                self.pokemon
                    .iter()
                    .map(|pokemon| vec![pokemon.internal_id, pokemon.level])
                    .flatten()
                    .collect(),
                vec![0x00],
            ]
            .concat(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PartyPokemon {
    pub level: u8,
    pub pokedex_id: u8,
    pub(crate) internal_id: u8,
}

impl PartyPokemon {
    pub fn new(level: u8, pokedex_id: u8) -> Self {
        PartyPokemon {
            level,
            pokedex_id,
            internal_id: 0,
        }
    }
}

/// Level type of all party Pokémon
///
/// ```
/// use pkmnapi_db::*;
///
/// let party_level_type_same = PartyLevelType::SAME(42);
/// let party_level_type_different = PartyLevelType::DIFFERENT;
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PartyLevelType {
    SAME(u8),
    DIFFERENT,
}
