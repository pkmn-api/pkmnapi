use crate::error::{self, Result};
use crate::patch::*;
use crate::PkmnapiDB;
use std::collections::HashMap;

impl PkmnapiDB {
    pub fn get_pokemon_evolutions_all(
        &self,
        pokedex_ids: &Vec<u8>,
    ) -> Result<HashMap<u8, Vec<PokemonEvolution>>> {
        let pokemon_evolutions_all: HashMap<u8, Vec<PokemonEvolution>> = pokedex_ids
            .iter()
            .map(|pokedex_id| {
                let pokemon_evolutions = self.get_pokemon_evolutions(pokedex_id)?;

                Ok((*pokedex_id, pokemon_evolutions))
            })
            .collect::<Result<HashMap<u8, Vec<PokemonEvolution>>>>()?;

        Ok(pokemon_evolutions_all)
    }

    /// Get Pokémon evolutions by Pokédex ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::*;
    /// use std::fs;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    ///
    /// let pokemon_evolutions = db.get_pokemon_evolutions(&1).unwrap();
    ///
    /// assert_eq!(
    ///     pokemon_evolutions,
    ///     vec![
    ///         PokemonEvolutionLevel::new(2, 16)
    ///     ]
    /// );
    /// ```
    pub fn get_pokemon_evolutions(&self, pokedex_id: &u8) -> Result<Vec<PokemonEvolution>> {
        let offset_base = PkmnapiDB::ROM_PAGE * 0x1D;
        let offset = offset_base + 0x105C;

        let internal_id = self.pokedex_id_to_internal_id(pokedex_id)?;

        let pointer_offset = offset + ((internal_id as usize) * 0x02);
        let pointer =
            (offset_base - (PkmnapiDB::ROM_PAGE * 0x03)) + self.get_pointer(pointer_offset);

        let evolution_data = self.rom[pointer..(pointer + 0x0D)].to_vec();
        let mut pokemon_evolutions = vec![];
        let mut i = 0;

        while i < evolution_data.len() {
            let id = evolution_data[i];

            if id == 0x00 {
                break;
            }

            pokemon_evolutions.push(PokemonEvolution::from(&evolution_data[i..(i + 4)]));

            i = match id {
                0x01 => i + 3,
                0x02 => i + 4,
                0x03 => i + 3,
                _ => unreachable!(),
            };
        }

        let pokemon_evolutions = pokemon_evolutions
            .iter()
            .map(|pokemon_evolution| match pokemon_evolution {
                PokemonEvolution::LEVEL(evolution) => {
                    PokemonEvolution::LEVEL(PokemonEvolutionLevel {
                        pokedex_id: self
                            .internal_id_to_pokedex_id(&evolution.internal_id)
                            .unwrap(),
                        internal_id: 0,
                        ..*evolution
                    })
                }
                PokemonEvolution::ITEM(evolution) => PokemonEvolution::ITEM(PokemonEvolutionItem {
                    pokedex_id: self
                        .internal_id_to_pokedex_id(&evolution.internal_id)
                        .unwrap(),
                    internal_id: 0,
                    ..*evolution
                }),
                PokemonEvolution::TRADE(evolution) => {
                    PokemonEvolution::TRADE(PokemonEvolutionTrade {
                        pokedex_id: self
                            .internal_id_to_pokedex_id(&evolution.internal_id)
                            .unwrap(),
                        internal_id: 0,
                        ..*evolution
                    })
                }
            })
            .collect();

        Ok(pokemon_evolutions)
    }

    /// Set Pokémon evolutions by Pokédex ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::patch::*;
    /// use pkmnapi_db::*;
    /// use std::fs;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    ///
    /// let patch = db.set_pokemon_evolutions(&1, &vec![
    ///     PokemonEvolutionLevel::new(2, 16)
    /// ]).unwrap();
    ///
    /// assert_eq!(
    ///     patch,
    ///     Patch {
    ///         offset: 0x3B844,
    ///         length: 0x03,
    ///         data: vec![0x01, 0x10, 0x09]
    ///     }
    /// );
    /// ```
    pub fn set_pokemon_evolutions(
        &self,
        pokedex_id: &u8,
        pokemon_evolutions: &Vec<PokemonEvolution>,
    ) -> Result<Patch> {
        let old_pokemon_evolutions = self.get_pokemon_evolutions(pokedex_id)?;
        let old_pokemon_evolutions_data: Vec<u8> = old_pokemon_evolutions
            .iter()
            .map(|pokemon_evolution| pokemon_evolution.to_raw())
            .flatten()
            .collect();
        let old_pokemon_evolutions_data_len = old_pokemon_evolutions_data.len();

        let pokemon_evolutions_data: Vec<u8> = pokemon_evolutions
            .iter()
            .map(|pokemon_evolution| {
                let pokemon_evolution = match pokemon_evolution {
                    PokemonEvolution::LEVEL(evolution) => {
                        PokemonEvolution::LEVEL(PokemonEvolutionLevel {
                            internal_id: self
                                .pokedex_id_to_internal_id(&evolution.pokedex_id)
                                .unwrap(),
                            ..*evolution
                        })
                    }
                    PokemonEvolution::ITEM(evolution) => {
                        PokemonEvolution::ITEM(PokemonEvolutionItem {
                            internal_id: self
                                .pokedex_id_to_internal_id(&evolution.pokedex_id)
                                .unwrap(),
                            ..*evolution
                        })
                    }
                    PokemonEvolution::TRADE(evolution) => {
                        PokemonEvolution::TRADE(PokemonEvolutionTrade {
                            internal_id: self
                                .pokedex_id_to_internal_id(&evolution.pokedex_id)
                                .unwrap(),
                            ..*evolution
                        })
                    }
                };

                pokemon_evolution.to_raw()
            })
            .flatten()
            .collect();
        let pokemon_evolutions_data_len = pokemon_evolutions_data.len();

        if old_pokemon_evolutions_data_len != pokemon_evolutions_data_len {
            return Err(error::Error::PokemonEvolutionWrongSize(
                old_pokemon_evolutions_data_len,
                pokemon_evolutions_data_len,
            ));
        }

        let offset_base = PkmnapiDB::ROM_PAGE * 0x1D;
        let offset = offset_base + 0x105C;

        let internal_id = self.pokedex_id_to_internal_id(pokedex_id)?;

        let pointer_offset = offset + ((internal_id as usize) * 0x02);
        let pointer =
            (offset_base - (PkmnapiDB::ROM_PAGE * 0x03)) + self.get_pointer(pointer_offset);

        Ok(Patch::new(&pointer, &pokemon_evolutions_data))
    }
}

/// Pokémon evolution type
///
/// ```
/// use pkmnapi_db::*;
///
/// let pokemon_evolution_level = PokemonEvolutionLevel::new(0, 0);
///
/// assert_eq!(
///     pokemon_evolution_level,
///     PokemonEvolution::LEVEL(Default::default())
/// );
///
/// let pokemon_evolution_item = PokemonEvolutionItem::new(0, 0);
///
/// assert_eq!(
///     pokemon_evolution_item,
///     PokemonEvolution::ITEM(Default::default())
/// );
///
/// let pokemon_evolution_trade = PokemonEvolutionTrade::new(0);
///
/// assert_eq!(
///     pokemon_evolution_trade,
///     PokemonEvolution::TRADE(Default::default())
/// );
/// ```
#[derive(Debug, PartialEq)]
pub enum PokemonEvolution {
    LEVEL(PokemonEvolutionLevel),
    ITEM(PokemonEvolutionItem),
    TRADE(PokemonEvolutionTrade),
}

impl From<&[u8]> for PokemonEvolution {
    fn from(rom: &[u8]) -> Self {
        let mut rom = rom.iter();
        let id = rom.next().unwrap();

        match id {
            0x01 => PokemonEvolution::LEVEL(PokemonEvolutionLevel {
                level: *rom.next().unwrap(),
                internal_id: *rom.next().unwrap() - 1,
                pokedex_id: 0x00,
            }),
            0x02 => PokemonEvolution::ITEM(PokemonEvolutionItem {
                item_id: *rom.next().unwrap(),
                internal_id: *rom.skip(1).next().unwrap() - 1,
                pokedex_id: 0x00,
            }),
            0x03 => PokemonEvolution::TRADE(PokemonEvolutionTrade {
                internal_id: *rom.skip(1).next().unwrap() - 1,
                pokedex_id: 0x00,
            }),
            _ => unreachable!(),
        }
    }
}

impl PokemonEvolution {
    /// Pokémon evolution to raw bytes
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::*;
    ///
    /// let pokemon_evolution_level = PokemonEvolutionLevel::new(112, 32);
    /// let raw = pokemon_evolution_level.to_raw();
    ///
    /// assert_eq!(raw, vec![0x01, 0x20, 0x01]);
    ///
    /// let pokemon_evolution_item = PokemonEvolutionItem::new(112, 32);
    /// let raw = pokemon_evolution_item.to_raw();
    ///
    /// assert_eq!(raw, vec![0x02, 0x20, 0x01, 0x01]);
    ///
    /// let pokemon_evolution_trade = PokemonEvolutionTrade::new(112);
    /// let raw = pokemon_evolution_trade.to_raw();
    ///
    /// assert_eq!(raw, vec![0x03, 0x01, 0x01]);
    /// ```
    pub fn to_raw(&self) -> Vec<u8> {
        match self {
            PokemonEvolution::LEVEL(evolution) => {
                vec![0x01, evolution.level, evolution.internal_id + 1]
            }
            PokemonEvolution::ITEM(evolution) => {
                vec![0x02, evolution.item_id, 0x01, evolution.internal_id + 1]
            }
            PokemonEvolution::TRADE(evolution) => vec![0x03, 0x01, evolution.internal_id + 1],
        }
    }
}

/// Pokémon evolution by level
///
/// ```
/// use pkmnapi_db::*;
///
/// let pokemon_evolution_level = PokemonEvolutionLevel::new(0, 0);
///
/// assert_eq!(
///     pokemon_evolution_level,
///     PokemonEvolution::LEVEL(Default::default())
/// );
/// ```
#[derive(Debug, PartialEq)]
pub struct PokemonEvolutionLevel {
    pub level: u8,
    pub pokedex_id: u8,
    pub(crate) internal_id: u8,
}

impl Default for PokemonEvolutionLevel {
    fn default() -> Self {
        PokemonEvolutionLevel {
            level: 0,
            pokedex_id: 0,
            internal_id: 0,
        }
    }
}

impl PokemonEvolutionLevel {
    /// Create new Pokémon evolution by level
    ///
    /// ```
    /// use pkmnapi_db::*;
    ///
    /// let pokemon_evolution_level = PokemonEvolutionLevel::new(0, 0);
    ///
    /// assert_eq!(
    ///     pokemon_evolution_level,
    ///     PokemonEvolution::LEVEL(Default::default())
    /// );
    /// ```
    pub fn new(pokedex_id: u8, level: u8) -> PokemonEvolution {
        PokemonEvolution::LEVEL(PokemonEvolutionLevel {
            level,
            pokedex_id,
            ..Default::default()
        })
    }
}

/// Pokémon evolution by item
///
/// ```
/// use pkmnapi_db::*;
///
/// let pokemon_evolution_item = PokemonEvolutionItem::new(0, 0);
///
/// assert_eq!(
///     pokemon_evolution_item,
///     PokemonEvolution::ITEM(Default::default())
/// );
/// ```
#[derive(Debug, PartialEq)]
pub struct PokemonEvolutionItem {
    pub item_id: u8,
    pub pokedex_id: u8,
    pub(crate) internal_id: u8,
}

impl Default for PokemonEvolutionItem {
    fn default() -> Self {
        PokemonEvolutionItem {
            item_id: 0,
            pokedex_id: 0,
            internal_id: 0,
        }
    }
}

impl PokemonEvolutionItem {
    /// Create new Pokémon evolution by item
    ///
    /// ```
    /// use pkmnapi_db::*;
    ///
    /// let pokemon_evolution_item = PokemonEvolutionItem::new(0, 0);
    ///
    /// assert_eq!(
    ///     pokemon_evolution_item,
    ///     PokemonEvolution::ITEM(Default::default())
    /// );
    /// ```
    pub fn new(pokedex_id: u8, item_id: u8) -> PokemonEvolution {
        PokemonEvolution::ITEM(PokemonEvolutionItem {
            item_id,
            pokedex_id,
            ..Default::default()
        })
    }
}

/// Pokémon evolution by trade
///
/// ```
/// use pkmnapi_db::*;
///
/// let pokemon_evolution_trade = PokemonEvolutionTrade::new(0);
///
/// assert_eq!(
///     pokemon_evolution_trade,
///     PokemonEvolution::TRADE(Default::default())
/// );
/// ```
#[derive(Debug, PartialEq)]
pub struct PokemonEvolutionTrade {
    pub pokedex_id: u8,
    pub(crate) internal_id: u8,
}

impl Default for PokemonEvolutionTrade {
    fn default() -> Self {
        PokemonEvolutionTrade {
            pokedex_id: 0,
            internal_id: 0,
        }
    }
}

impl PokemonEvolutionTrade {
    /// Create new Pokémon evolution by trade
    ///
    /// ```
    /// use pkmnapi_db::*;
    ///
    /// let pokemon_evolution_trade = PokemonEvolutionTrade::new(0);
    ///
    /// assert_eq!(
    ///     pokemon_evolution_trade,
    ///     PokemonEvolution::TRADE(Default::default())
    /// );
    /// ```
    pub fn new(pokedex_id: u8) -> PokemonEvolution {
        PokemonEvolution::TRADE(PokemonEvolutionTrade {
            pokedex_id,
            ..Default::default()
        })
    }
}
