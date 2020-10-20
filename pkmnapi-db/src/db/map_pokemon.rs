use crate::error::{self, Result};
use crate::patch::*;
use crate::PkmnapiDB;
use std::collections::HashMap;

impl PkmnapiDB {
    pub fn get_map_pokemon_all(&self, map_ids: &Vec<u8>) -> Result<HashMap<u8, MapPokemon>> {
        let map_pokemon_all: HashMap<u8, MapPokemon> = map_ids
            .iter()
            .map(|map_id| {
                let map_pokemon = self.get_map_pokemon(map_id)?;

                Ok((*map_id, map_pokemon))
            })
            .collect::<Result<HashMap<u8, MapPokemon>>>()?;

        Ok(map_pokemon_all)
    }

    pub fn get_map_pokemon(&self, map_id: &u8) -> Result<MapPokemon> {
        self.map_id_validate(map_id)?;

        let offset_base = PkmnapiDB::ROM_PAGE * 0x03;
        let offset = offset_base + 0x0EEB;
        let pointer_offset = offset + ((*map_id as usize) * 0x02);
        let pointer = offset_base - PkmnapiDB::ROM_PAGE + self.get_pointer(pointer_offset);

        let map_pokemon = MapPokemon::from(&self.rom[pointer..]);
        let map_pokemon = MapPokemon {
            grass: MapPokemonArea {
                pokemon: map_pokemon
                    .grass
                    .pokemon
                    .iter()
                    .map(|map_pokemon_info| MapPokemonInfo {
                        pokedex_id: self
                            .internal_id_to_pokedex_id(&map_pokemon_info.internal_id)
                            .unwrap(),
                        internal_id: 0,
                        ..*map_pokemon_info
                    })
                    .collect(),
                ..map_pokemon.grass
            },
            water: MapPokemonArea {
                pokemon: map_pokemon
                    .water
                    .pokemon
                    .iter()
                    .map(|map_pokemon_info| MapPokemonInfo {
                        pokedex_id: self
                            .internal_id_to_pokedex_id(&map_pokemon_info.internal_id)
                            .unwrap(),
                        internal_id: 0,
                        ..*map_pokemon_info
                    })
                    .collect(),
                ..map_pokemon.water
            },
        };

        Ok(map_pokemon)
    }

    pub fn set_map_pokemon(&self, map_id: &u8, map_pokemon: &MapPokemon) -> Result<Patch> {
        let old_map_pokemon = self.get_map_pokemon(map_id)?;
        let old_map_pokemon_data = old_map_pokemon.to_raw();
        let old_map_pokemon_data_len = old_map_pokemon_data.len();
        let map_pokemon_data = {
            MapPokemon {
                grass: MapPokemonArea {
                    pokemon: map_pokemon
                        .grass
                        .pokemon
                        .iter()
                        .map(|pokemon| MapPokemonInfo {
                            internal_id: self
                                .pokedex_id_to_internal_id(&pokemon.pokedex_id)
                                .unwrap(),
                            ..*pokemon
                        })
                        .collect(),
                    ..map_pokemon.grass
                },
                water: MapPokemonArea {
                    pokemon: map_pokemon
                        .water
                        .pokemon
                        .iter()
                        .map(|pokemon| MapPokemonInfo {
                            internal_id: self
                                .pokedex_id_to_internal_id(&pokemon.pokedex_id)
                                .unwrap(),
                            ..*pokemon
                        })
                        .collect(),
                    ..map_pokemon.water
                },
            }
        }
        .to_raw();
        let map_pokemon_data_len = map_pokemon_data.len();

        if old_map_pokemon_data_len != map_pokemon_data_len {
            return Err(error::Error::MapPokemonWrongSize(
                old_map_pokemon_data_len,
                map_pokemon_data_len,
            ));
        }

        let offset_base = PkmnapiDB::ROM_PAGE * 0x03;
        let offset = offset_base + 0x0EEB;
        let pointer_offset = offset + ((*map_id as usize) * 0x02);
        let pointer = offset_base - PkmnapiDB::ROM_PAGE + self.get_pointer(pointer_offset);

        Ok(Patch::new(&pointer, &map_pokemon_data))
    }
}

#[derive(Debug, PartialEq)]
pub struct MapPokemon {
    pub grass: MapPokemonArea,
    pub water: MapPokemonArea,
}

impl From<&[u8]> for MapPokemon {
    /// Convert &[u8] to MapPokemon
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::*;
    ///
    /// let rom = vec![
    ///     0x0A, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01,
    ///     0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x00,
    /// ];
    /// let map_pokemon = MapPokemon::from(&rom[..]);
    ///
    /// assert_eq!(
    ///     map_pokemon,
    ///     MapPokemon {
    ///         grass: MapPokemonArea {
    ///             encounter_rate: 10,
    ///             pokemon: vec![MapPokemonInfo::new(10, 0); 10],
    ///         },
    ///         water: MapPokemonArea {
    ///             encounter_rate: 0,
    ///             pokemon: vec![]
    ///         }
    ///     }
    /// );
    /// ```
    fn from(rom: &[u8]) -> Self {
        let grass_offset = 0;
        let grass_encounter_rate = rom[grass_offset];
        let grass_pokemon = {
            let count = if grass_encounter_rate != 0x00 { 10 } else { 0 };

            rom[(grass_offset + 1)..]
                .chunks(2)
                .take(count)
                .map(|chunk| {
                    let level = chunk[0];
                    let internal_id = chunk[1] - 1;

                    MapPokemonInfo {
                        level,
                        internal_id,
                        pokedex_id: 0,
                    }
                })
                .collect()
        };
        let water_offset = if grass_encounter_rate == 0x00 {
            grass_offset + 1
        } else {
            grass_offset + 21
        };
        let water_encounter_rate = rom[water_offset];
        let water_pokemon = {
            let count = if water_encounter_rate != 0x00 { 10 } else { 0 };

            rom[(water_offset + 1)..]
                .chunks(2)
                .take(count)
                .map(|chunk| {
                    let level = chunk[0];
                    let internal_id = chunk[1] - 1;

                    MapPokemonInfo {
                        level,
                        internal_id,
                        pokedex_id: 0,
                    }
                })
                .collect()
        };

        MapPokemon {
            grass: MapPokemonArea {
                encounter_rate: grass_encounter_rate,
                pokemon: grass_pokemon,
            },
            water: MapPokemonArea {
                encounter_rate: water_encounter_rate,
                pokemon: water_pokemon,
            },
        }
    }
}

impl MapPokemon {
    pub fn to_raw(&self) -> Vec<u8> {
        let grass_encounter_rate = {
            if self.grass.pokemon.len() == 0 {
                vec![0]
            } else {
                vec![self.grass.encounter_rate]
            }
        };
        let grass_pokemon = self
            .grass
            .pokemon
            .iter()
            .map(|pokemon| vec![pokemon.level, pokemon.internal_id + 1])
            .flatten()
            .collect();
        let water_encounter_rate = {
            if self.water.pokemon.len() == 0 {
                vec![0]
            } else {
                vec![self.water.encounter_rate]
            }
        };
        let water_pokemon = self
            .water
            .pokemon
            .iter()
            .map(|pokemon| vec![pokemon.level, pokemon.internal_id + 1])
            .flatten()
            .collect();

        [
            grass_encounter_rate,
            grass_pokemon,
            water_encounter_rate,
            water_pokemon,
        ]
        .concat()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct MapPokemonArea {
    pub encounter_rate: u8,
    pub pokemon: Vec<MapPokemonInfo>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MapPokemonInfo {
    pub level: u8,
    pub pokedex_id: u8,
    pub(crate) internal_id: u8,
}

impl MapPokemonInfo {
    pub fn new(level: u8, pokedex_id: u8) -> Self {
        MapPokemonInfo {
            level,
            pokedex_id,
            internal_id: 0,
        }
    }
}
