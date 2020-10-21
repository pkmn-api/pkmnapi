use crate::error::Result;
use crate::patch::*;
use crate::PkmnapiDB;
use std::cmp::Ordering;
use std::collections::HashMap;

impl PkmnapiDB {
    pub fn get_pokemon_machines_all(
        &self,
        pokedex_ids: &Vec<u8>,
    ) -> Result<HashMap<u8, Vec<PokemonMachine>>> {
        self.get_all(pokedex_ids, |id| self.get_pokemon_machines(id))
    }

    /// Get Pokémon machines by Pokédex ID
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
    /// let pokemon_machines = db.get_pokemon_machines(&1).unwrap();
    ///
    /// assert_eq!(
    ///     pokemon_machines,
    ///     vec![
    ///         PokemonMachine::TM(0x03),
    ///         PokemonMachine::TM(0x06),
    ///         PokemonMachine::TM(0x08),
    ///         PokemonMachine::TM(0x09),
    ///         PokemonMachine::TM(0x0A),
    ///         PokemonMachine::TM(0x14),
    ///         PokemonMachine::TM(0x15),
    ///         PokemonMachine::TM(0x16),
    ///         PokemonMachine::TM(0x1F),
    ///         PokemonMachine::TM(0x20),
    ///         PokemonMachine::TM(0x21),
    ///         PokemonMachine::TM(0x22),
    ///         PokemonMachine::TM(0x2C),
    ///         PokemonMachine::TM(0x32),
    ///         PokemonMachine::HM(0x01),
    ///     ]
    /// );
    /// ```
    pub fn get_pokemon_machines(&self, pokedex_id: &u8) -> Result<Vec<PokemonMachine>> {
        let _internal_id = self.pokedex_id_to_internal_id(pokedex_id)?;
        let (_, max_pokedex_id) = self.pokedex_id_bounds();

        let offset = {
            if pokedex_id == &(max_pokedex_id as u8) {
                0x425B
            } else {
                let offset_base = PkmnapiDB::ROM_PAGE * 0x0E;

                (offset_base + 0x03DE) + (((*pokedex_id as usize) - 1) * 0x1C)
            }
        } + 0x14;

        let (_min_tm_id, max_tm_id) = self.tm_id_validate(&0x01)?;
        let (_min_hm_id, max_hm_id) = self.hm_id_validate(&0x01)?;

        let machines: Vec<PokemonMachine> = self.rom[offset..(offset + 0x07)]
            .iter()
            .map(|byte| {
                (0..8)
                    .map(|i| (byte & (0x01 << i)) >> i)
                    .collect::<Vec<u8>>()
            })
            .flatten()
            .enumerate()
            .filter_map(|(i, bit)| {
                if bit == 0 {
                    return None;
                }

                if i >= max_tm_id {
                    let hm_id = (i + 1) - max_tm_id;

                    if hm_id > max_hm_id {
                        None
                    } else {
                        Some(PokemonMachine::HM(hm_id as u8))
                    }
                } else {
                    Some(PokemonMachine::TM((i as u8) + 1))
                }
            })
            .collect::<Vec<PokemonMachine>>();

        Ok(machines)
    }

    /// Set Pokémon machines by Pokédex ID
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
    /// let patch = db
    ///     .set_pokemon_machines(
    ///         &1,
    ///         &vec![
    ///             PokemonMachine::TM(0x02),
    ///             PokemonMachine::TM(0x07),
    ///             PokemonMachine::TM(0x09),
    ///             PokemonMachine::TM(0x0A),
    ///             PokemonMachine::TM(0x0D),
    ///             PokemonMachine::TM(0x11),
    ///             PokemonMachine::TM(0x12),
    ///             PokemonMachine::TM(0x13),
    ///             PokemonMachine::TM(0x15),
    ///             PokemonMachine::TM(0x16),
    ///             PokemonMachine::TM(0x19),
    ///             PokemonMachine::TM(0x21),
    ///             PokemonMachine::TM(0x24),
    ///             PokemonMachine::TM(0x26),
    ///             PokemonMachine::TM(0x27),
    ///             PokemonMachine::TM(0x2A),
    ///             PokemonMachine::TM(0x2F),
    ///             PokemonMachine::TM(0x31),
    ///             PokemonMachine::TM(0x32),
    ///             PokemonMachine::HM(0x01),
    ///             PokemonMachine::HM(0x02)
    ///         ]
    ///     )
    ///     .unwrap();
    ///
    /// assert_eq!(
    ///     patch,
    ///     Patch {
    ///         offset: 0x383F2,
    ///         length: 0x07,
    ///         data: vec![0x42, 0x13, 0x37, 0x01, 0x69, 0x42, 0x0F]
    ///     }
    /// );
    /// ```
    pub fn set_pokemon_machines(
        &self,
        pokedex_id: &u8,
        pokemon_machines: &Vec<PokemonMachine>,
    ) -> Result<Patch> {
        let _internal_id = self.pokedex_id_to_internal_id(pokedex_id)?;
        let (_, max_pokedex_id) = self.pokedex_id_bounds();

        let offset = {
            if pokedex_id == &(max_pokedex_id as u8) {
                0x425B
            } else {
                let offset_base = PkmnapiDB::ROM_PAGE * 0x0E;

                (offset_base + 0x03DE) + (((*pokedex_id as usize) - 1) * 0x1C)
            }
        } + 0x14;

        let (_min_id, max_id) = self.tm_id_validate(&0x01)?;
        let mut pokemon_machines = pokemon_machines.to_vec();

        pokemon_machines.sort();

        let pokemon_machines = pokemon_machines
            .iter()
            .map(|pokemon_machine| {
                match pokemon_machine {
                    PokemonMachine::TM(tm_id) => {
                        self.tm_id_validate(&tm_id)?;
                    }
                    PokemonMachine::HM(hm_id) => {
                        self.hm_id_validate(&hm_id)?;
                    }
                };

                Ok(pokemon_machine)
            })
            .collect::<Result<Vec<_>>>()?;

        let machine_ids: Vec<u8> = pokemon_machines
            .iter()
            .map(|pokemon_machine| match pokemon_machine {
                PokemonMachine::TM(tm_id) => tm_id - 1,
                PokemonMachine::HM(hm_id) => (hm_id - 1) + (max_id as u8),
            })
            .collect();

        let data: Vec<u8> = (0..(7 * 8))
            .map(|i| {
                if machine_ids.contains(&(i as u8)) {
                    0x01
                } else {
                    0x00
                }
            })
            .collect::<Vec<u8>>()
            .as_slice()
            .chunks(8)
            .map(|chunk| (0..8).map(|i| chunk[7 - i] << 7 - i).fold(0, |a, b| a | b))
            .collect();

        Ok(Patch::new(&offset, &data))
    }
}

#[derive(Clone, Debug, Eq)]
pub enum PokemonMachine {
    TM(u8),
    HM(u8),
}

impl Ord for PokemonMachine {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (PokemonMachine::TM(a), PokemonMachine::TM(b)) if a == b => Ordering::Equal,
            (PokemonMachine::TM(a), PokemonMachine::TM(b)) if a < b => Ordering::Less,
            (PokemonMachine::TM(a), PokemonMachine::TM(b)) if a > b => Ordering::Greater,
            (PokemonMachine::HM(a), PokemonMachine::HM(b)) if a == b => Ordering::Equal,
            (PokemonMachine::HM(a), PokemonMachine::HM(b)) if a < b => Ordering::Less,
            (PokemonMachine::HM(a), PokemonMachine::HM(b)) if a > b => Ordering::Greater,
            (PokemonMachine::TM(_), PokemonMachine::HM(_)) => Ordering::Less,
            (PokemonMachine::HM(_), PokemonMachine::TM(_)) => Ordering::Greater,
            _ => Ordering::Equal,
        }
    }
}

impl PartialOrd for PokemonMachine {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for PokemonMachine {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (PokemonMachine::TM(a), PokemonMachine::TM(b)) => a == b,
            (PokemonMachine::HM(a), PokemonMachine::HM(b)) => a == b,
            _ => false,
        }
    }
}
