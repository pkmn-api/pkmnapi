use rocket::response::status;
use rocket_contrib::json::Json;
use serde::Serialize;

use crate::responses::base::*;

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
    PokemonNameResponseError(status::NotFound<Json<PokemonNameResponseError>>),
    PokemonNameResponseErrorInvalid(status::BadRequest<Json<PokemonNameResponseErrorInvalid>>),
    PokemonPicResponseError(status::NotFound<Json<PokemonPicResponseError>>),
    RomPatchResponseError(status::NotFound<Json<RomPatchResponseError>>),
    RomResponseErrorInvalidRom(status::BadRequest<Json<RomResponseErrorInvalidRom>>),
    RomResponseErrorNoRom(status::Forbidden<Json<RomResponseErrorNoRom>>),
    RomResponseErrorRomExists(status::Forbidden<Json<RomResponseErrorRomExists>>),
    SavResponseErrorInvalidSav(status::BadRequest<Json<SavResponseErrorInvalidSav>>),
    SavResponseErrorNoSav(status::Forbidden<Json<SavResponseErrorNoSav>>),
    SavResponseErrorSavExists(status::Forbidden<Json<SavResponseErrorSavExists>>),
    StatsResponseError(status::NotFound<Json<StatsResponseError>>),
    StatsResponseErrorInvalid(status::BadRequest<Json<StatsResponseErrorInvalid>>),
    TMResponseError(status::NotFound<Json<TMResponseError>>),
    TMResponseErrorInvalid(status::BadRequest<Json<TMResponseErrorInvalid>>),
    TrainerNameResponseError(status::NotFound<Json<TrainerNameResponseError>>),
    TrainerNameResponseErrorInvalid(status::BadRequest<Json<TrainerNameResponseErrorInvalid>>),
    TrainerPicResponseError(status::NotFound<Json<TrainerPicResponseError>>),
    TypeEffectResponseError(status::NotFound<Json<TypeEffectResponseError>>),
    TypeEffectResponseErrorInvalid(status::BadRequest<Json<TypeEffectResponseErrorInvalid>>),
    TypeResponseError(status::NotFound<Json<TypeResponseError>>),
    TypeResponseErrorInvalid(status::BadRequest<Json<TypeResponseErrorInvalid>>),
}

pub type AccessTokenErrorEmail = BaseErrorResponse<AccessTokenErrorEmailAttributes>;

impl AccessTokenErrorEmail {
    pub fn new(message: &String) -> ResponseError {
        let response = AccessTokenErrorEmail {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_access_tokens_email,
                _type: BaseErrorResponseType::errors,
                attributes: AccessTokenErrorEmailAttributes {
                    message: message.to_owned(),
                },
            },
        };

        ResponseError::AccessTokenErrorEmail(status::Forbidden(Some(Json(response))))
    }
}

#[derive(Debug, Serialize)]
pub struct AccessTokenErrorEmailAttributes {
    pub message: String,
}

pub type AccessTokenErrorForbidden = BaseErrorResponse<AccessTokenErrorForbiddenAttributes>;

impl AccessTokenErrorForbidden {
    pub fn new() -> ResponseError {
        let response = AccessTokenErrorForbidden {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_access_tokens_forbidden,
                _type: BaseErrorResponseType::errors,
                attributes: AccessTokenErrorForbiddenAttributes {
                    message: "Authorization header must not be set".to_owned(),
                },
            },
        };

        ResponseError::AccessTokenErrorForbidden(status::Forbidden(Some(Json(response))))
    }
}

#[derive(Debug, Serialize)]
pub struct AccessTokenErrorForbiddenAttributes {
    pub message: String,
}

pub type AccessTokenErrorInvalid = BaseErrorResponse<AccessTokenErrorInvalidAttributes>;

impl AccessTokenErrorInvalid {
    pub fn new(message: &String) -> ResponseError {
        let response = AccessTokenErrorInvalid {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_access_tokens_invalid,
                _type: BaseErrorResponseType::errors,
                attributes: AccessTokenErrorInvalidAttributes {
                    message: message.to_owned(),
                },
            },
        };

        ResponseError::AccessTokenErrorInvalid(status::BadRequest(Some(Json(response))))
    }
}

#[derive(Debug, Serialize)]
pub struct AccessTokenErrorInvalidAttributes {
    pub message: String,
}

pub type AccessTokenErrorTimeout = BaseErrorResponse<AccessTokenErrorTimeoutAttributes>;

impl AccessTokenErrorTimeout {
    pub fn new(message: &String) -> ResponseError {
        let response = AccessTokenErrorTimeout {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_access_tokens_timeout,
                _type: BaseErrorResponseType::errors,
                attributes: AccessTokenErrorTimeoutAttributes {
                    message: message.to_owned(),
                },
            },
        };

        ResponseError::AccessTokenErrorTimeout(status::Forbidden(Some(Json(response))))
    }
}

#[derive(Debug, Serialize)]
pub struct AccessTokenErrorTimeoutAttributes {
    pub message: String,
}

pub type AccessTokenErrorUnauthorized = BaseErrorResponse<AccessTokenErrorUnauthorizedAttributes>;

impl AccessTokenErrorUnauthorized {
    pub fn new() -> ResponseError {
        let response = AccessTokenErrorUnauthorized {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_access_tokens_unauthorized,
                _type: BaseErrorResponseType::errors,
                attributes: AccessTokenErrorUnauthorizedAttributes {
                    message: "Authorization header must be set".to_owned(),
                },
            },
        };

        ResponseError::AccessTokenErrorUnauthorized(status::Unauthorized(Some(Json(response))))
    }
}

#[derive(Debug, Serialize)]
pub struct AccessTokenErrorUnauthorizedAttributes {
    pub message: String,
}

pub type MapPicResponseError = BaseErrorResponse<MapPicResponseErrorAttributes>;

impl MapPicResponseError {
    pub fn new(message: &String) -> ResponseError {
        let response = MapPicResponseError {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_map_pics,
                _type: BaseErrorResponseType::errors,
                attributes: MapPicResponseErrorAttributes {
                    message: message.to_owned(),
                },
            },
        };

        ResponseError::MapPicResponseError(status::NotFound(Json(response)))
    }
}

#[derive(Debug, Serialize)]
pub struct MapPicResponseErrorAttributes {
    pub message: String,
}

pub type MoveResponseError = BaseErrorResponse<MoveResponseErrorAttributes>;

impl MoveResponseError {
    pub fn new(message: &String) -> ResponseError {
        let response = MoveResponseError {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_moves,
                _type: BaseErrorResponseType::errors,
                attributes: MoveResponseErrorAttributes {
                    message: message.to_owned(),
                },
            },
        };

        ResponseError::MoveResponseError(status::NotFound(Json(response)))
    }
}

#[derive(Debug, Serialize)]
pub struct MoveResponseErrorAttributes {
    pub message: String,
}

pub type MoveResponseErrorInvalid = BaseErrorResponse<MoveResponseErrorInvalidAttributes>;

impl MoveResponseErrorInvalid {
    pub fn new(message: &String) -> ResponseError {
        let response = MoveResponseErrorInvalid {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_moves_invalid,
                _type: BaseErrorResponseType::errors,
                attributes: MoveResponseErrorInvalidAttributes {
                    message: message.to_owned(),
                },
            },
        };

        ResponseError::MoveResponseErrorInvalid(status::BadRequest(Some(Json(response))))
    }
}

#[derive(Debug, Serialize)]
pub struct MoveResponseErrorInvalidAttributes {
    pub message: String,
}

pub type PokemonNameResponseError = BaseErrorResponse<PokemonNameResponseErrorAttributes>;

impl PokemonNameResponseError {
    pub fn new(message: &String) -> ResponseError {
        let response = PokemonNameResponseError {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_pokemon_names,
                _type: BaseErrorResponseType::errors,
                attributes: PokemonNameResponseErrorAttributes {
                    message: message.to_owned(),
                },
            },
        };

        ResponseError::PokemonNameResponseError(status::NotFound(Json(response)))
    }
}

#[derive(Debug, Serialize)]
pub struct PokemonNameResponseErrorAttributes {
    pub message: String,
}

pub type PokemonNameResponseErrorInvalid =
    BaseErrorResponse<PokemonNameResponseErrorInvalidAttributes>;

impl PokemonNameResponseErrorInvalid {
    pub fn new(message: &String) -> ResponseError {
        let response = PokemonNameResponseErrorInvalid {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_pokemon_names_invalid,
                _type: BaseErrorResponseType::errors,
                attributes: PokemonNameResponseErrorInvalidAttributes {
                    message: message.to_owned(),
                },
            },
        };

        ResponseError::PokemonNameResponseErrorInvalid(status::BadRequest(Some(Json(response))))
    }
}

#[derive(Debug, Serialize)]
pub struct PokemonNameResponseErrorInvalidAttributes {
    pub message: String,
}

pub type PokemonPicResponseError = BaseErrorResponse<PokemonPicResponseErrorAttributes>;

impl PokemonPicResponseError {
    pub fn new(message: &String) -> ResponseError {
        let response = PokemonPicResponseError {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_pokemon_pics,
                _type: BaseErrorResponseType::errors,
                attributes: PokemonPicResponseErrorAttributes {
                    message: message.to_owned(),
                },
            },
        };

        ResponseError::PokemonPicResponseError(status::NotFound(Json(response)))
    }
}

#[derive(Debug, Serialize)]
pub struct PokemonPicResponseErrorAttributes {
    pub message: String,
}

pub type RomPatchResponseError = BaseErrorResponse<RomPatchResponseErrorAttributes>;

impl RomPatchResponseError {
    pub fn new() -> ResponseError {
        let response = RomPatchResponseError {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_rom_patches,
                _type: BaseErrorResponseType::errors,
                attributes: RomPatchResponseErrorAttributes {
                    message: "No ROM patch found".to_owned(),
                },
            },
        };

        ResponseError::RomPatchResponseError(status::NotFound(Json(response)))
    }
}

#[derive(Debug, Serialize)]
pub struct RomPatchResponseErrorAttributes {
    pub message: String,
}

pub type RomResponseErrorNoRom = BaseErrorResponse<RomResponseErrorNoRomAttributes>;

impl RomResponseErrorNoRom {
    pub fn new() -> ResponseError {
        let response = RomResponseErrorNoRom {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_roms_no_rom,
                _type: BaseErrorResponseType::errors,
                attributes: RomResponseErrorNoRomAttributes {
                    message: "No ROM uploaded".to_owned(),
                },
            },
        };

        ResponseError::RomResponseErrorNoRom(status::Forbidden(Some(Json(response))))
    }
}

#[derive(Debug, Serialize)]
pub struct RomResponseErrorNoRomAttributes {
    pub message: String,
}

pub type RomResponseErrorInvalidRom = BaseErrorResponse<RomResponseErrorInvalidRomAttributes>;

impl RomResponseErrorInvalidRom {
    pub fn new() -> ResponseError {
        let response = RomResponseErrorInvalidRom {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_roms_invalid_rom,
                _type: BaseErrorResponseType::errors,
                attributes: RomResponseErrorInvalidRomAttributes {
                    message: "Invalid ROM provided".to_owned(),
                },
            },
        };

        ResponseError::RomResponseErrorInvalidRom(status::BadRequest(Some(Json(response))))
    }
}

#[derive(Debug, Serialize)]
pub struct RomResponseErrorInvalidRomAttributes {
    pub message: String,
}

pub type RomResponseErrorRomExists = BaseErrorResponse<RomResponseErrorRomExistsAttributes>;

impl RomResponseErrorRomExists {
    pub fn new() -> ResponseError {
        let response = RomResponseErrorRomExists {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_roms_rom_exists,
                _type: BaseErrorResponseType::errors,
                attributes: RomResponseErrorRomExistsAttributes {
                    message: "ROM already exists".to_owned(),
                },
            },
        };

        ResponseError::RomResponseErrorRomExists(status::Forbidden(Some(Json(response))))
    }
}

#[derive(Debug, Serialize)]
pub struct RomResponseErrorRomExistsAttributes {
    pub message: String,
}

pub type SavResponseErrorNoSav = BaseErrorResponse<SavResponseErrorNoSavAttributes>;

impl SavResponseErrorNoSav {
    pub fn new() -> ResponseError {
        let response = SavResponseErrorNoSav {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_savs_no_sav,
                _type: BaseErrorResponseType::errors,
                attributes: SavResponseErrorNoSavAttributes {
                    message: "No SAV uploaded".to_owned(),
                },
            },
        };

        ResponseError::SavResponseErrorNoSav(status::Forbidden(Some(Json(response))))
    }
}

#[derive(Debug, Serialize)]
pub struct SavResponseErrorNoSavAttributes {
    pub message: String,
}

pub type SavResponseErrorInvalidSav = BaseErrorResponse<SavResponseErrorInvalidSavAttributes>;

impl SavResponseErrorInvalidSav {
    pub fn new() -> ResponseError {
        let response = SavResponseErrorInvalidSav {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_savs_invalid_sav,
                _type: BaseErrorResponseType::errors,
                attributes: SavResponseErrorInvalidSavAttributes {
                    message: "Invalid SAV provided".to_owned(),
                },
            },
        };

        ResponseError::SavResponseErrorInvalidSav(status::BadRequest(Some(Json(response))))
    }
}

#[derive(Debug, Serialize)]
pub struct SavResponseErrorInvalidSavAttributes {
    pub message: String,
}

pub type SavResponseErrorSavExists = BaseErrorResponse<SavResponseErrorSavExistsAttributes>;

impl SavResponseErrorSavExists {
    pub fn new() -> ResponseError {
        let response = SavResponseErrorSavExists {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_savs_sav_exists,
                _type: BaseErrorResponseType::errors,
                attributes: SavResponseErrorSavExistsAttributes {
                    message: "SAV already exists".to_owned(),
                },
            },
        };

        ResponseError::SavResponseErrorSavExists(status::Forbidden(Some(Json(response))))
    }
}

#[derive(Debug, Serialize)]
pub struct SavResponseErrorSavExistsAttributes {
    pub message: String,
}

pub type StatsResponseError = BaseErrorResponse<StatsResponseErrorAttributes>;

impl StatsResponseError {
    pub fn new(message: &String) -> ResponseError {
        let response = StatsResponseError {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_stats,
                _type: BaseErrorResponseType::errors,
                attributes: StatsResponseErrorAttributes {
                    message: message.to_owned(),
                },
            },
        };

        ResponseError::StatsResponseError(status::NotFound(Json(response)))
    }
}

#[derive(Debug, Serialize)]
pub struct StatsResponseErrorAttributes {
    pub message: String,
}

pub type StatsResponseErrorInvalid = BaseErrorResponse<StatsResponseErrorInvalidAttributes>;

impl StatsResponseErrorInvalid {
    pub fn new(message: &String) -> ResponseError {
        let response = StatsResponseErrorInvalid {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_stats_invalid,
                _type: BaseErrorResponseType::errors,
                attributes: StatsResponseErrorInvalidAttributes {
                    message: message.to_owned(),
                },
            },
        };

        ResponseError::StatsResponseErrorInvalid(status::BadRequest(Some(Json(response))))
    }
}

#[derive(Debug, Serialize)]
pub struct StatsResponseErrorInvalidAttributes {
    pub message: String,
}

pub type TMResponseError = BaseErrorResponse<TMResponseErrorAttributes>;

impl TMResponseError {
    pub fn new(message: &String) -> ResponseError {
        let response = TMResponseError {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_tms,
                _type: BaseErrorResponseType::errors,
                attributes: TMResponseErrorAttributes {
                    message: message.to_owned(),
                },
            },
        };

        ResponseError::TMResponseError(status::NotFound(Json(response)))
    }
}

#[derive(Debug, Serialize)]
pub struct TMResponseErrorAttributes {
    pub message: String,
}

pub type TMResponseErrorInvalid = BaseErrorResponse<TMResponseErrorInvalidAttributes>;

impl TMResponseErrorInvalid {
    pub fn new(message: &String) -> ResponseError {
        let response = TMResponseErrorInvalid {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_tms_invalid,
                _type: BaseErrorResponseType::errors,
                attributes: TMResponseErrorInvalidAttributes {
                    message: message.to_owned(),
                },
            },
        };

        ResponseError::TMResponseErrorInvalid(status::BadRequest(Some(Json(response))))
    }
}

#[derive(Debug, Serialize)]
pub struct TMResponseErrorInvalidAttributes {
    pub message: String,
}

pub type TrainerNameResponseError = BaseErrorResponse<TrainerNameResponseErrorAttributes>;

impl TrainerNameResponseError {
    pub fn new(message: &String) -> ResponseError {
        let response = TrainerNameResponseError {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_trainer_names,
                _type: BaseErrorResponseType::errors,
                attributes: TrainerNameResponseErrorAttributes {
                    message: message.to_owned(),
                },
            },
        };

        ResponseError::TrainerNameResponseError(status::NotFound(Json(response)))
    }
}

#[derive(Debug, Serialize)]
pub struct TrainerNameResponseErrorAttributes {
    pub message: String,
}

pub type TrainerNameResponseErrorInvalid =
    BaseErrorResponse<TrainerNameResponseErrorInvalidAttributes>;

impl TrainerNameResponseErrorInvalid {
    pub fn new(message: &String) -> ResponseError {
        let response = TrainerNameResponseErrorInvalid {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_trainer_names_invalid,
                _type: BaseErrorResponseType::errors,
                attributes: TrainerNameResponseErrorInvalidAttributes {
                    message: message.to_owned(),
                },
            },
        };

        ResponseError::TrainerNameResponseErrorInvalid(status::BadRequest(Some(Json(response))))
    }
}

#[derive(Debug, Serialize)]
pub struct TrainerNameResponseErrorInvalidAttributes {
    pub message: String,
}

pub type TrainerPicResponseError = BaseErrorResponse<TrainerPicResponseErrorAttributes>;

impl TrainerPicResponseError {
    pub fn new(message: &String) -> ResponseError {
        let response = TrainerPicResponseError {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_trainer_pics,
                _type: BaseErrorResponseType::errors,
                attributes: TrainerPicResponseErrorAttributes {
                    message: message.to_owned(),
                },
            },
        };

        ResponseError::TrainerPicResponseError(status::NotFound(Json(response)))
    }
}

#[derive(Debug, Serialize)]
pub struct TrainerPicResponseErrorAttributes {
    pub message: String,
}

pub type TypeResponseError = BaseErrorResponse<TypeResponseErrorAttributes>;

impl TypeResponseError {
    pub fn new(message: &String) -> ResponseError {
        let response = TypeResponseError {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_types,
                _type: BaseErrorResponseType::errors,
                attributes: TypeResponseErrorAttributes {
                    message: message.to_owned(),
                },
            },
        };

        ResponseError::TypeResponseError(status::NotFound(Json(response)))
    }
}

#[derive(Debug, Serialize)]
pub struct TypeResponseErrorAttributes {
    pub message: String,
}

pub type TypeResponseErrorInvalid = BaseErrorResponse<TypeResponseErrorInvalidAttributes>;

impl TypeResponseErrorInvalid {
    pub fn new(message: &String) -> ResponseError {
        let response = TypeResponseErrorInvalid {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_types_invalid,
                _type: BaseErrorResponseType::errors,
                attributes: TypeResponseErrorInvalidAttributes {
                    message: message.to_owned(),
                },
            },
        };

        ResponseError::TypeResponseErrorInvalid(status::BadRequest(Some(Json(response))))
    }
}

#[derive(Debug, Serialize)]
pub struct TypeResponseErrorInvalidAttributes {
    pub message: String,
}

pub type TypeEffectResponseError = BaseErrorResponse<TypeEffectResponseErrorAttributes>;

impl TypeEffectResponseError {
    pub fn new(message: &String) -> ResponseError {
        let response = TypeEffectResponseError {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_type_effects,
                _type: BaseErrorResponseType::errors,
                attributes: TypeEffectResponseErrorAttributes {
                    message: message.to_owned(),
                },
            },
        };

        ResponseError::TypeEffectResponseError(status::NotFound(Json(response)))
    }
}

#[derive(Debug, Serialize)]
pub struct TypeEffectResponseErrorAttributes {
    pub message: String,
}

pub type TypeEffectResponseErrorInvalid =
    BaseErrorResponse<TypeEffectResponseErrorInvalidAttributes>;

impl TypeEffectResponseErrorInvalid {
    pub fn new(message: &String) -> ResponseError {
        let response = TypeEffectResponseErrorInvalid {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_type_effects_invalid,
                _type: BaseErrorResponseType::errors,
                attributes: TypeEffectResponseErrorInvalidAttributes {
                    message: message.to_owned(),
                },
            },
        };

        ResponseError::TypeEffectResponseErrorInvalid(status::BadRequest(Some(Json(response))))
    }
}

#[derive(Debug, Serialize)]
pub struct TypeEffectResponseErrorInvalidAttributes {
    pub message: String,
}
