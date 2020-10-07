use rocket::response::status;
use rocket_contrib::json::Json;
use serde::Serialize;

mod access_token_error_email;
mod access_token_error_forbidden;
mod access_token_error_invalid;
mod access_token_error_timeout;
mod access_token_error_unauthorized;
mod bad_request_error;
mod etag_error_mismatch;
mod etag_error_missing;
mod internal_server_error;
mod not_found_error;
mod rom_error_invalid_rom;
mod rom_error_no_rom;
mod rom_error_rom_exists;
mod sav_error_invalid_sav;
mod sav_error_no_sav;
mod sav_error_sav_exists;
mod too_many_requests_error;

pub use crate::responses::errors::access_token_error_email::*;
pub use crate::responses::errors::access_token_error_forbidden::*;
pub use crate::responses::errors::access_token_error_invalid::*;
pub use crate::responses::errors::access_token_error_timeout::*;
pub use crate::responses::errors::access_token_error_unauthorized::*;
pub use crate::responses::errors::bad_request_error::*;
pub use crate::responses::errors::etag_error_mismatch::*;
pub use crate::responses::errors::etag_error_missing::*;
pub use crate::responses::errors::internal_server_error::*;
pub use crate::responses::errors::not_found_error::*;
pub use crate::responses::errors::rom_error_invalid_rom::*;
pub use crate::responses::errors::rom_error_no_rom::*;
pub use crate::responses::errors::rom_error_rom_exists::*;
pub use crate::responses::errors::sav_error_invalid_sav::*;
pub use crate::responses::errors::sav_error_no_sav::*;
pub use crate::responses::errors::sav_error_sav_exists::*;
pub use crate::responses::errors::too_many_requests_error::*;

#[derive(Debug, Responder)]
pub enum ResponseError {
    AccessTokenErrorEmail(status::Forbidden<Json<AccessTokenErrorEmail>>),
    AccessTokenErrorForbidden(status::Forbidden<Json<AccessTokenErrorForbidden>>),
    AccessTokenErrorInvalid(status::BadRequest<Json<AccessTokenErrorInvalid>>),
    AccessTokenErrorTimeout(status::Forbidden<Json<AccessTokenErrorTimeout>>),
    AccessTokenErrorUnauthorized(status::Unauthorized<Json<AccessTokenErrorUnauthorized>>),
    BadRequestError(status::BadRequest<Json<BadRequestError>>),
    ETagErrorMismatch(status::BadRequest<Json<ETagErrorMismatch>>),
    ETagErrorMissing(status::Forbidden<Json<ETagErrorMissing>>),
    NotFoundError(status::NotFound<Json<NotFoundError>>),
    RomErrorInvalidRom(status::BadRequest<Json<RomErrorInvalidRom>>),
    RomErrorNoRom(status::Forbidden<Json<RomErrorNoRom>>),
    RomErrorRomExists(status::Forbidden<Json<RomErrorRomExists>>),
    SavErrorInvalidSav(status::BadRequest<Json<SavErrorInvalidSav>>),
    SavErrorNoSav(status::Forbidden<Json<SavErrorNoSav>>),
    SavErrorSavExists(status::Forbidden<Json<SavErrorSavExists>>),
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
    error_etag_mismatch,
    error_etag_missing,
    error_hms_invalid,
    error_hms,
    error_internal_server,
    error_item_names_invalid,
    error_item_names,
    error_map_pics,
    error_map_pokemon_invalid,
    error_map_pokemon,
    error_move_names_invalid,
    error_move_names,
    error_not_found,
    error_pokedex_entries_invalid,
    error_pokedex_entries,
    error_pokedex_texts_invalid,
    error_pokedex_texts,
    error_pokemon_cries_invalid,
    error_pokemon_cries,
    error_pokemon_evolutions_invalid,
    error_pokemon_evolutions,
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
    error_tm_moves_invalid,
    error_tm_moves,
    error_tm_prices_invalid,
    error_tm_prices,
    error_too_many_requests,
    error_trainer_names_invalid,
    error_trainer_names,
    error_trainer_parties_invalid,
    error_trainer_parties,
    error_trainer_pics,
    error_type_effects_invalid,
    error_type_effects,
    error_type_names_invalid,
    error_type_names,
}

#[derive(Debug, Serialize)]
#[allow(non_camel_case_types)]
pub enum BaseErrorResponseType {
    errors,
}
