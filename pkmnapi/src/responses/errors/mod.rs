use rocket::response::status;
use rocket_contrib::json::Json;
use serde::Serialize;

mod access_token_error_email;
mod access_token_error_forbidden;
mod access_token_error_invalid;
mod access_token_error_timeout;
mod access_token_error_unauthorized;
mod map_pic_response_error;
mod move_response_error;
mod move_response_error_invalid;
mod not_found_error;
mod pokemon_name_response_error;
mod pokemon_name_response_error_invalid;
mod pokemon_pic_response_error;
mod pokemon_stats_response_error;
mod pokemon_stats_response_error_invalid;
mod rom_patch_response_error;
mod rom_response_error_invalid_rom;
mod rom_response_error_no_rom;
mod rom_response_error_rom_exists;
mod sav_player_name_response_error;
mod sav_player_name_response_error_invalid;
mod sav_response_error_invalid_sav;
mod sav_response_error_no_sav;
mod sav_response_error_sav_exists;
mod tm_response_error;
mod tm_response_error_invalid;
mod trainer_name_response_error;
mod trainer_name_response_error_invalid;
mod trainer_parties_response_error;
mod trainer_parties_response_error_invalid;
mod trainer_pic_response_error;
mod type_effect_response_error;
mod type_effect_response_error_invalid;
mod type_response_error;
mod type_response_error_invalid;

pub use crate::responses::errors::access_token_error_email::*;
pub use crate::responses::errors::access_token_error_forbidden::*;
pub use crate::responses::errors::access_token_error_invalid::*;
pub use crate::responses::errors::access_token_error_timeout::*;
pub use crate::responses::errors::access_token_error_unauthorized::*;
pub use crate::responses::errors::map_pic_response_error::*;
pub use crate::responses::errors::move_response_error::*;
pub use crate::responses::errors::move_response_error_invalid::*;
pub use crate::responses::errors::not_found_error::*;
pub use crate::responses::errors::pokemon_name_response_error::*;
pub use crate::responses::errors::pokemon_name_response_error_invalid::*;
pub use crate::responses::errors::pokemon_pic_response_error::*;
pub use crate::responses::errors::pokemon_stats_response_error::*;
pub use crate::responses::errors::pokemon_stats_response_error_invalid::*;
pub use crate::responses::errors::rom_patch_response_error::*;
pub use crate::responses::errors::rom_response_error_invalid_rom::*;
pub use crate::responses::errors::rom_response_error_no_rom::*;
pub use crate::responses::errors::rom_response_error_rom_exists::*;
pub use crate::responses::errors::sav_player_name_response_error::*;
pub use crate::responses::errors::sav_player_name_response_error_invalid::*;
pub use crate::responses::errors::sav_response_error_invalid_sav::*;
pub use crate::responses::errors::sav_response_error_no_sav::*;
pub use crate::responses::errors::sav_response_error_sav_exists::*;
pub use crate::responses::errors::tm_response_error::*;
pub use crate::responses::errors::tm_response_error_invalid::*;
pub use crate::responses::errors::trainer_name_response_error::*;
pub use crate::responses::errors::trainer_name_response_error_invalid::*;
pub use crate::responses::errors::trainer_parties_response_error::*;
pub use crate::responses::errors::trainer_parties_response_error_invalid::*;
pub use crate::responses::errors::trainer_pic_response_error::*;
pub use crate::responses::errors::type_effect_response_error::*;
pub use crate::responses::errors::type_effect_response_error_invalid::*;
pub use crate::responses::errors::type_response_error::*;
pub use crate::responses::errors::type_response_error_invalid::*;

#[derive(Debug, Responder)]
pub enum ResponseError {
    AccessTokenErrorEmail(status::Forbidden<Json<AccessTokenErrorEmail>>),
    AccessTokenErrorForbidden(status::Forbidden<Json<AccessTokenErrorForbidden>>),
    AccessTokenErrorInvalid(status::BadRequest<Json<AccessTokenErrorInvalid>>),
    AccessTokenErrorTimeout(status::Forbidden<Json<AccessTokenErrorTimeout>>),
    AccessTokenErrorUnauthorized(status::Unauthorized<Json<AccessTokenErrorUnauthorized>>),
    MapPicResponseError(status::NotFound<Json<MapPicResponseError>>),
    MoveResponseError(status::NotFound<Json<MoveResponseError>>),
    MoveResponseErrorInvalid(status::BadRequest<Json<MoveResponseErrorInvalid>>),
    NotFoundError(status::NotFound<Json<NotFoundError>>),
    PokemonNameResponseError(status::NotFound<Json<PokemonNameResponseError>>),
    PokemonNameResponseErrorInvalid(status::BadRequest<Json<PokemonNameResponseErrorInvalid>>),
    PokemonPicResponseError(status::NotFound<Json<PokemonPicResponseError>>),
    PokemonStatsResponseError(status::NotFound<Json<PokemonStatsResponseError>>),
    PokemonStatsResponseErrorInvalid(status::BadRequest<Json<PokemonStatsResponseErrorInvalid>>),
    RomPatchResponseError(status::NotFound<Json<RomPatchResponseError>>),
    RomResponseErrorInvalidRom(status::BadRequest<Json<RomResponseErrorInvalidRom>>),
    RomResponseErrorNoRom(status::Forbidden<Json<RomResponseErrorNoRom>>),
    RomResponseErrorRomExists(status::Forbidden<Json<RomResponseErrorRomExists>>),
    SavPlayerNameResponseError(status::NotFound<Json<SavPlayerNameResponseError>>),
    SavPlayerNameResponseErrorInvalid(status::BadRequest<Json<SavPlayerNameResponseErrorInvalid>>),
    SavResponseErrorInvalidSav(status::BadRequest<Json<SavResponseErrorInvalidSav>>),
    SavResponseErrorNoSav(status::Forbidden<Json<SavResponseErrorNoSav>>),
    SavResponseErrorSavExists(status::Forbidden<Json<SavResponseErrorSavExists>>),
    TMResponseError(status::NotFound<Json<TMResponseError>>),
    TMResponseErrorInvalid(status::BadRequest<Json<TMResponseErrorInvalid>>),
    TrainerNameResponseError(status::NotFound<Json<TrainerNameResponseError>>),
    TrainerNameResponseErrorInvalid(status::BadRequest<Json<TrainerNameResponseErrorInvalid>>),
    TrainerPartiesResponseError(status::NotFound<Json<TrainerPartiesResponseError>>),
    TrainerPartiesResponseErrorInvalid(
        status::BadRequest<Json<TrainerPartiesResponseErrorInvalid>>,
    ),
    TrainerPicResponseError(status::NotFound<Json<TrainerPicResponseError>>),
    TypeEffectResponseError(status::NotFound<Json<TypeEffectResponseError>>),
    TypeEffectResponseErrorInvalid(status::BadRequest<Json<TypeEffectResponseErrorInvalid>>),
    TypeResponseError(status::NotFound<Json<TypeResponseError>>),
    TypeResponseErrorInvalid(status::BadRequest<Json<TypeResponseErrorInvalid>>),
}

#[derive(Debug, Serialize)]
pub struct BaseErrorResponse<T> {
    pub data: BaseErrorResponseData<T>,
}

#[derive(Debug, Serialize)]
pub struct BaseErrorResponseData<T> {
    pub id: BaseErrorResponseId,
    #[serde(rename = "type")]
    pub _type: BaseErrorResponseType,
    pub attributes: T,
}

#[derive(Debug, Serialize)]
#[allow(non_camel_case_types)]
pub enum BaseErrorResponseId {
    error_access_tokens_email,
    error_access_tokens_forbidden,
    error_access_tokens_invalid,
    error_access_tokens_timeout,
    error_access_tokens_unauthorized,
    error_map_pics,
    error_moves_invalid,
    error_moves,
    error_not_found,
    error_pokemon_names_invalid,
    error_pokemon_names,
    error_pokemon_pics,
    error_pokemon_stats_invalid,
    error_pokemon_stats,
    error_rom_patches,
    error_roms_invalid_rom,
    error_roms_no_rom,
    error_roms_rom_exists,
    error_sav_player_names_invalid,
    error_sav_player_names,
    error_savs_invalid_sav,
    error_savs_no_sav,
    error_savs_sav_exists,
    error_tms_invalid,
    error_tms,
    error_trainer_names_invalid,
    error_trainer_names,
    error_trainer_parties_invalid,
    error_trainer_parties,
    error_trainer_pics,
    error_type_effects_invalid,
    error_type_effects,
    error_types_invalid,
    error_types,
}

#[derive(Debug, Serialize)]
#[allow(non_camel_case_types)]
pub enum BaseErrorResponseType {
    errors,
}
