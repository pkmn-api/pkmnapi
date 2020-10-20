use crate::error::Result;
use crate::patch::*;
use crate::sav::Sav;

impl Sav {
    /// Get save Pokémon seen
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::sav::*;
    /// use std::fs;
    /// # use std::env;
    /// # let sav_path = env::var("PKMN_SAV").expect("Set the PKMN_SAV environment variable to point to the SAV location");
    ///
    /// let sav_data = fs::read(sav_path).unwrap();
    /// let sav = Sav::new(&sav_data).unwrap();
    ///
    /// let pokemon_seen = sav.get_pokemon_seen().unwrap();
    ///
    /// assert_eq!(
    ///     pokemon_seen,
    ///     vec![0x01]
    /// );
    /// ```
    pub fn get_pokemon_seen(&self) -> Result<Vec<u8>> {
        let offset = 0x25B6;

        let save_pokemon_seen: Vec<u8> = self.sav[offset..(offset + 19)]
            .iter()
            .map(|byte| (0..8).map(move |i| (byte & (0x01 << i)) >> i))
            .flatten()
            .enumerate()
            .filter_map(
                |(i, bit)| {
                    if bit == 0x01 {
                        Some(i as u8 + 1)
                    } else {
                        None
                    }
                },
            )
            .collect();

        Ok(save_pokemon_seen)
    }

    /// Set save Pokémon seen
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::patch::*;
    /// use pkmnapi_db::sav::*;
    /// use std::fs;
    /// # use std::env;
    /// # let sav_path = env::var("PKMN_SAV").expect("Set the PKMN_SAV environment variable to point to the SAV location");
    ///
    /// let sav_data = fs::read(sav_path).unwrap();
    /// let sav = Sav::new(&sav_data).unwrap();
    ///
    /// let patch = sav
    ///     .set_pokemon_seen(
    ///         &vec![0x01, 0x04, 0x07]
    ///     )
    ///     .unwrap();
    ///
    /// assert_eq!(
    ///     patch,
    ///     Patch {
    ///         offset: 0x25B6,
    ///         length: 0x13,
    ///         data: vec![0x49, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
    ///     }
    /// );
    /// ```
    pub fn set_pokemon_seen(&self, save_pokemon_seen: &Vec<u8>) -> Result<Patch> {
        let offset = 0x25B6;

        let bits: Vec<u8> = (1..=152)
            .map(|id| {
                if save_pokemon_seen.contains(&id) {
                    0x01
                } else {
                    0x00
                }
            })
            .collect::<Vec<u8>>()
            .chunks(8)
            .map(|chunk| {
                (0..8)
                    .map(move |i| chunk[i] << i)
                    .fold(0, |acc, bit| acc | bit)
            })
            .collect();

        Ok(Patch::new(&offset, &bits))
    }
}
