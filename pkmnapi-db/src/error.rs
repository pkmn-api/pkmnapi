//! Pkmnapi error module
//!
//! ```
//! use pkmnapi_db::error::*;
//!
//! let err = Error::PokedexIDInvalid(255);
//!
//! assert_eq!(err.to_string(), "Invalid Pokédex ID: 255");
//! ```

use std::{fmt, result};

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug, PartialEq)]
pub enum Error {
    CryCouldNotCreate,
    CryCouldNotFinalize,
    CryCouldNotWriteSample,
    HeaderParseError(String),
    HeaderTooSmall,
    HMIDInvalid(u8, usize, usize),
    IconIDInvalid(u8, usize, usize),
    ImgCouldNotRead,
    ImgCouldNotWrite,
    InternalIDInvalid(u8),
    ItemIDInvalid(u8, usize, usize),
    ItemNameWrongSize(usize, usize),
    MapCouldNotWrite,
    MapIDInvalid(u8, usize, usize),
    MapInvalid(u8),
    MapPokemonWrongSize(usize, usize),
    MoveIDInvalid(u8, usize, usize),
    MoveNameWrongSize(usize, usize),
    PicCouldNotRead,
    PicCouldNotWrite,
    PicTooLarge,
    PicWrongSize,
    PlayerNamesWrongSize(usize, usize),
    PokedexEntrySpeciesWrongSize(usize, usize),
    PokedexIDInvalid(u8),
    PokedexTextWrongSize(usize, usize),
    PokemonEvolutionWrongSize(usize, usize),
    PokemonLearnsetWrongSize(usize, usize),
    PokemonLogoWrongSize(usize, usize),
    PokemonTitleWrongSize(usize, usize),
    SavBagItemsWrongSize(usize, usize),
    SavBoxItemsWrongSize(usize, usize),
    SavPlayerNameWrongSize(usize, usize),
    SavRivalNameWrongSize(usize, usize),
    SavWrongSize(usize, usize),
    TMIDInvalid(u8, usize, usize),
    TrainerIDInvalid(u8, usize, usize),
    TrainerNameWrongSize(usize, usize),
    TrainerPartiesWrongDataSize(usize, usize),
    TrainerPartiesWrongSize(usize, usize),
    TypeEffectIDInvalid(u8, usize, usize),
    TypeIDInvalid(u8, usize, usize),
    TypeNameWrongSize(usize, usize),
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self)
    }
}

impl fmt::Display for Error {
    /// Converts the error to a String
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::error;
    ///
    /// let err = error::Error::HeaderParseError("foo".to_owned());
    /// let string = err.to_string();
    ///
    /// assert_eq!(string, "foo");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let output = match self {
            Error::CryCouldNotCreate => "Could not create cry".to_owned(),
            Error::CryCouldNotFinalize => "Could not finalize cry".to_owned(),
            Error::CryCouldNotWriteSample => "Could not write cry sample".to_owned(),
            Error::HeaderParseError(string) => string.to_owned(),
            Error::HeaderTooSmall => "Header too small".to_owned(),
            Error::HMIDInvalid(hm_id, min, max) => {
                format!("Invalid HM ID {}: valid range is {}-{}", hm_id, min, max)
            }
            Error::IconIDInvalid(icon_id, min, max) => format!(
                "Invalid icon ID {}: valid range is {}-{}",
                icon_id, min, max
            ),
            Error::ImgCouldNotRead => "Could not read image".to_owned(),
            Error::ImgCouldNotWrite => "Could not write image".to_owned(),
            Error::InternalIDInvalid(internal_id) => {
                format!("Invalid internal ID: {}", internal_id)
            }
            Error::ItemIDInvalid(item_id, min, max) => format!(
                "Invalid item ID {}: valid range is {}-{}",
                item_id, min, max
            ),
            Error::ItemNameWrongSize(expected, actual) => format!(
                "Item name length mismatch: should be exactly {} characters, found {}",
                expected, actual
            ),
            Error::MapCouldNotWrite => "Could not write image".to_owned(),
            Error::MapIDInvalid(map_id, min, max) => {
                format!("Invalid map ID {}: valid range is {}-{}", map_id, min, max)
            }
            Error::MapInvalid(map_id) => format!("Invalid map ID: {}", map_id),
            Error::MapPokemonWrongSize(expected, actual) => format!(
                "Map Pokémon size mismatch: should be exactly {} bytes, found {}",
                expected, actual
            ),
            Error::MoveIDInvalid(move_id, min, max) => format!(
                "Invalid move ID {}: valid range is {}-{}",
                move_id, min, max
            ),
            Error::MoveNameWrongSize(expected, actual) => format!(
                "Move name length mismatch: should be exactly {} characters, found {}",
                expected, actual
            ),
            Error::PicCouldNotRead => "Could not read image".to_owned(),
            Error::PicCouldNotWrite => "Could not write image".to_owned(),
            Error::PicTooLarge => "Compressed image is too large".to_owned(),
            Error::PicWrongSize => "Image dimensions must be multiples of 8".to_owned(),
            Error::PlayerNamesWrongSize(expected, actual) => format!(
                "Player names length mismatch: should be exactly {} bytes, found {}",
                expected, actual
            ),
            Error::PokedexEntrySpeciesWrongSize(expected, actual) => format!(
                "Pokédex entry species length mismatch: should be exactly {} characters, found {}",
                expected, actual
            ),
            Error::PokedexIDInvalid(pokedex_id) => format!("Invalid Pokédex ID: {}", pokedex_id),
            Error::PokedexTextWrongSize(expected, actual) => format!(
                "Pokédex text length mismatch: should be {} characters or fewer, found {}",
                expected, actual
            ),
            Error::PokemonEvolutionWrongSize(expected, actual) => format!(
                "Pokémon evolutions length mismatch, should be exactly {} bytes, found {}",
                expected, actual
            ),
            Error::PokemonLearnsetWrongSize(expected, actual) => format!(
                "Pokémon learnset length mismatch, should be exactly {} bytes, found {}",
                expected, actual
            ),
            Error::PokemonLogoWrongSize(expected, actual) => format!(
                "Pokémon logo size mismatch: should be exactly {} bytes, found {}",
                expected, actual
            ),
            Error::PokemonTitleWrongSize(expected, actual) => format!(
                "Pokémon title length mismatch: should be {}, found {}",
                expected, actual
            ),
            Error::SavBagItemsWrongSize(expected, actual) => format!(
                "Sav bag items length mismatch: should be {} items or fewer, found {}",
                expected, actual
            ),
            Error::SavBoxItemsWrongSize(expected, actual) => format!(
                "Sav box items length mismatch: should be {} items or fewer, found {}",
                expected, actual
            ),
            Error::SavPlayerNameWrongSize(expected, actual) => format!(
                "Player name length mismatch: should be {} characters or fewer, found {}",
                expected, actual
            ),
            Error::SavRivalNameWrongSize(expected, actual) => format!(
                "Sav rival name length mismatch: should be {} characters or fewer, found {}",
                expected, actual
            ),
            Error::SavWrongSize(expected, actual) => format!(
                "Sav length mismatch: should be {} bytes, found {}",
                expected, actual
            ),
            Error::TMIDInvalid(tm_id, min, max) => {
                format!("Invalid TM ID {}: valid range is {}-{}", tm_id, min, max)
            }
            Error::TrainerIDInvalid(item_id, min, max) => format!(
                "Invalid trainer ID {}: valid range is {}-{}",
                item_id, min, max
            ),
            Error::TrainerNameWrongSize(expected, actual) => format!(
                "Trainer name length mismatch: should be exactly {} characters, found {}",
                expected, actual
            ),
            Error::TrainerPartiesWrongDataSize(expected, actual) => format!(
                "Trainer parties data length mismatch: should be exactly {} bytes, found {}",
                expected, actual
            ),
            Error::TrainerPartiesWrongSize(expected, actual) => format!(
                "Trainer parties length mismatch: should be exactly {} {}, found {}",
                expected,
                if expected == &1 { "party" } else { "parties" },
                actual
            ),
            Error::TypeEffectIDInvalid(type_effect_id, min, max) => format!(
                "Invalid type effect ID {}: valid range is {}-{}",
                type_effect_id, min, max
            ),
            Error::TypeIDInvalid(type_id, min, max) => format!(
                "Invalid type ID {}: valid range is {}-{}",
                type_id, min, max
            ),
            Error::TypeNameWrongSize(expected, actual) => format!(
                "Type name length mismatch: should be {} characters or fewer, found {}",
                expected, actual
            ),
        };

        write!(f, "{}", output)
    }
}
