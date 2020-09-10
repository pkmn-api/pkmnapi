use serde::Serialize;

use crate::responses::links::Links;

#[derive(Debug, Serialize)]
pub struct BaseResponse<T> {
    pub id: String,
    #[serde(rename = "type")]
    pub _type: BaseResponseType,
    pub attributes: T,
    pub links: Links,
}

#[derive(Debug, Serialize)]
pub struct BaseResponseAll<T> {
    pub data: Vec<T>,
    pub links: Links,
}

#[derive(Debug, Serialize)]
#[allow(non_camel_case_types)]
pub enum BaseResponseType {
    moves,
    patches,
    roms,
    stats,
    tms,
    type_effects,
    types,
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
    error_access_tokens_forbidden,
    error_access_tokens_invalid,
    error_access_tokens_unauthorized,
    error_moves_invalid,
    error_moves,
    error_patches,
    error_pokemon_pics,
    error_roms_invalid_rom,
    error_roms_no_rom,
    error_roms_rom_exists,
    error_stats_invalid,
    error_stats,
    error_tms_invalid,
    error_tms,
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
