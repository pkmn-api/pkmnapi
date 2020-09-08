use rocket::response::status;
use rocket_contrib::json::Json;
use serde::Serialize;

#[derive(Debug, Responder)]
pub enum ResponseError {
    AccessTokenErrorInvalid(status::BadRequest<Json<AccessTokenErrorInvalid>>),
    AccessTokenErrorUnauthorized(status::Unauthorized<Json<AccessTokenErrorUnauthorized>>),
    AccessTokenErrorForbidden(status::Forbidden<Json<AccessTokenErrorForbidden>>),
    PatchResponseError(status::NotFound<Json<PatchResponseError>>),
    RomResponseErrorInvalidRom(status::BadRequest<Json<RomResponseErrorInvalidRom>>),
    RomResponseErrorNoRom(status::Forbidden<Json<RomResponseErrorNoRom>>),
    RomResponseErrorRomExists(status::Forbidden<Json<RomResponseErrorRomExists>>),
    StatsResponseError(status::NotFound<Json<StatsResponseError>>),
    StatsResponseErrorInvalid(status::BadRequest<Json<StatsResponseErrorInvalid>>),
    TypeResponseError(status::NotFound<Json<TypeResponseError>>),
    TypeResponseErrorInvalid(status::BadRequest<Json<TypeResponseErrorInvalid>>),
    TypeEffectResponseError(status::NotFound<Json<TypeEffectResponseError>>),
    TypeEffectResponseErrorInvalid(status::BadRequest<Json<TypeEffectResponseErrorInvalid>>),
}

#[derive(Debug, Serialize)]
#[allow(non_camel_case_types)]
pub enum ResponseErrorType {
    errors,
}

#[derive(Debug, Serialize)]
#[allow(non_camel_case_types)]
pub enum ResponseErrorId {
    error_access_tokens_invalid,
    error_access_tokens_unauthorized,
    error_access_tokens_forbidden,
    error_patches,
    error_roms_invalid_rom,
    error_roms_no_rom,
    error_roms_rom_exists,
    error_stats,
    error_stats_invalid,
    error_types,
    error_types_invalid,
    error_type_effects,
    error_type_effects_invalid,
}

#[derive(Debug, Serialize)]
pub struct AccessTokenErrorInvalid {
    pub data: AccessTokenErrorInvalidData,
}

impl AccessTokenErrorInvalid {
    pub fn new(message: &String) -> ResponseError {
        let response = AccessTokenErrorInvalid {
            data: AccessTokenErrorInvalidData {
                id: ResponseErrorId::error_access_tokens_invalid,
                _type: ResponseErrorType::errors,
                attributes: AccessTokenErrorInvalidDataAttributes {
                    message: message.to_owned(),
                },
            },
        };

        ResponseError::AccessTokenErrorInvalid(status::BadRequest(Some(Json(response))))
    }
}

#[derive(Debug, Serialize)]
pub struct AccessTokenErrorInvalidData {
    pub id: ResponseErrorId,
    #[serde(rename = "type")]
    pub _type: ResponseErrorType,
    pub attributes: AccessTokenErrorInvalidDataAttributes,
}

#[derive(Debug, Serialize)]
pub struct AccessTokenErrorInvalidDataAttributes {
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct AccessTokenErrorUnauthorized {
    pub data: AccessTokenErrorUnauthorizedData,
}

impl AccessTokenErrorUnauthorized {
    pub fn new() -> ResponseError {
        let response = AccessTokenErrorUnauthorized {
            data: AccessTokenErrorUnauthorizedData {
                id: ResponseErrorId::error_access_tokens_unauthorized,
                _type: ResponseErrorType::errors,
                attributes: AccessTokenErrorUnauthorizedDataAttributes {
                    message: "Authorization header must be set".to_owned(),
                },
            },
        };

        ResponseError::AccessTokenErrorUnauthorized(status::Unauthorized(Some(Json(response))))
    }
}

#[derive(Debug, Serialize)]
pub struct AccessTokenErrorUnauthorizedData {
    pub id: ResponseErrorId,
    #[serde(rename = "type")]
    pub _type: ResponseErrorType,
    pub attributes: AccessTokenErrorUnauthorizedDataAttributes,
}

#[derive(Debug, Serialize)]
pub struct AccessTokenErrorUnauthorizedDataAttributes {
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct AccessTokenErrorForbidden {
    pub data: AccessTokenErrorForbiddenData,
}

impl AccessTokenErrorForbidden {
    pub fn new() -> ResponseError {
        let response = AccessTokenErrorForbidden {
            data: AccessTokenErrorForbiddenData {
                id: ResponseErrorId::error_access_tokens_forbidden,
                _type: ResponseErrorType::errors,
                attributes: AccessTokenErrorForbiddenDataAttributes {
                    message: "Authorization header must not be set".to_owned(),
                },
            },
        };

        ResponseError::AccessTokenErrorForbidden(status::Forbidden(Some(Json(response))))
    }
}

#[derive(Debug, Serialize)]
pub struct AccessTokenErrorForbiddenData {
    pub id: ResponseErrorId,
    #[serde(rename = "type")]
    pub _type: ResponseErrorType,
    pub attributes: AccessTokenErrorForbiddenDataAttributes,
}

#[derive(Debug, Serialize)]
pub struct AccessTokenErrorForbiddenDataAttributes {
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct PatchResponseError {
    pub data: PatchResponseErrorData,
}

impl PatchResponseError {
    pub fn new() -> ResponseError {
        let response = PatchResponseError {
            data: PatchResponseErrorData {
                id: ResponseErrorId::error_patches,
                _type: ResponseErrorType::errors,
                attributes: PatchResponseErrorDataAttributes {
                    message: "No patch found".to_owned(),
                },
            },
        };

        ResponseError::PatchResponseError(status::NotFound(Json(response)))
    }
}

#[derive(Debug, Serialize)]
pub struct PatchResponseErrorData {
    pub id: ResponseErrorId,
    #[serde(rename = "type")]
    pub _type: ResponseErrorType,
    pub attributes: PatchResponseErrorDataAttributes,
}

#[derive(Debug, Serialize)]
pub struct PatchResponseErrorDataAttributes {
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct RomResponseErrorNoRom {
    pub data: RomResponseErrorNoRomData,
}

impl RomResponseErrorNoRom {
    pub fn new() -> ResponseError {
        let response = RomResponseErrorNoRom {
            data: RomResponseErrorNoRomData {
                id: ResponseErrorId::error_roms_no_rom,
                _type: ResponseErrorType::errors,
                attributes: RomResponseErrorNoRomDataAttributes {
                    message: "No ROM uploaded".to_owned(),
                },
            },
        };

        ResponseError::RomResponseErrorNoRom(status::Forbidden(Some(Json(response))))
    }
}

#[derive(Debug, Serialize)]
pub struct RomResponseErrorNoRomData {
    pub id: ResponseErrorId,
    #[serde(rename = "type")]
    pub _type: ResponseErrorType,
    pub attributes: RomResponseErrorNoRomDataAttributes,
}

#[derive(Debug, Serialize)]
pub struct RomResponseErrorNoRomDataAttributes {
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct RomResponseErrorInvalidRom {
    pub data: RomResponseErrorInvalidRomData,
}

impl RomResponseErrorInvalidRom {
    pub fn new() -> ResponseError {
        let response = RomResponseErrorInvalidRom {
            data: RomResponseErrorInvalidRomData {
                id: ResponseErrorId::error_roms_invalid_rom,
                _type: ResponseErrorType::errors,
                attributes: RomResponseErrorInvalidRomDataAttributes {
                    message: "Invalid ROM provided".to_owned(),
                },
            },
        };

        ResponseError::RomResponseErrorInvalidRom(status::BadRequest(Some(Json(response))))
    }
}

#[derive(Debug, Serialize)]
pub struct RomResponseErrorInvalidRomData {
    pub id: ResponseErrorId,
    #[serde(rename = "type")]
    pub _type: ResponseErrorType,
    pub attributes: RomResponseErrorInvalidRomDataAttributes,
}

#[derive(Debug, Serialize)]
pub struct RomResponseErrorInvalidRomDataAttributes {
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct RomResponseErrorRomExists {
    pub data: RomResponseErrorRomExistsData,
}

impl RomResponseErrorRomExists {
    pub fn new() -> ResponseError {
        let response = RomResponseErrorRomExists {
            data: RomResponseErrorRomExistsData {
                id: ResponseErrorId::error_roms_rom_exists,
                _type: ResponseErrorType::errors,
                attributes: RomResponseErrorRomExistsDataAttributes {
                    message: "ROM already exists".to_owned(),
                },
            },
        };

        ResponseError::RomResponseErrorRomExists(status::Forbidden(Some(Json(response))))
    }
}

#[derive(Debug, Serialize)]
pub struct RomResponseErrorRomExistsData {
    pub id: ResponseErrorId,
    #[serde(rename = "type")]
    pub _type: ResponseErrorType,
    pub attributes: RomResponseErrorRomExistsDataAttributes,
}

#[derive(Debug, Serialize)]
pub struct RomResponseErrorRomExistsDataAttributes {
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct StatsResponseError {
    pub data: StatsResponseErrorData,
}

impl StatsResponseError {
    pub fn new(message: &String) -> ResponseError {
        let response = StatsResponseError {
            data: StatsResponseErrorData {
                id: ResponseErrorId::error_stats,
                _type: ResponseErrorType::errors,
                attributes: StatsResponseErrorDataAttributes {
                    message: message.to_owned(),
                },
            },
        };

        ResponseError::StatsResponseError(status::NotFound(Json(response)))
    }
}

#[derive(Debug, Serialize)]
pub struct StatsResponseErrorData {
    pub id: ResponseErrorId,
    #[serde(rename = "type")]
    pub _type: ResponseErrorType,
    pub attributes: StatsResponseErrorDataAttributes,
}

#[derive(Debug, Serialize)]
pub struct StatsResponseErrorDataAttributes {
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct StatsResponseErrorInvalid {
    pub data: StatsResponseErrorInvalidData,
}

impl StatsResponseErrorInvalid {
    pub fn new(message: &String) -> ResponseError {
        let response = StatsResponseErrorInvalid {
            data: StatsResponseErrorInvalidData {
                id: ResponseErrorId::error_stats_invalid,
                _type: ResponseErrorType::errors,
                attributes: StatsResponseErrorInvalidDataAttributes {
                    message: message.to_owned(),
                },
            },
        };

        ResponseError::StatsResponseErrorInvalid(status::BadRequest(Some(Json(response))))
    }
}

#[derive(Debug, Serialize)]
pub struct StatsResponseErrorInvalidData {
    pub id: ResponseErrorId,
    #[serde(rename = "type")]
    pub _type: ResponseErrorType,
    pub attributes: StatsResponseErrorInvalidDataAttributes,
}

#[derive(Debug, Serialize)]
pub struct StatsResponseErrorInvalidDataAttributes {
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct TypeResponseError {
    pub data: TypeResponseErrorData,
}

impl TypeResponseError {
    pub fn new(message: &String) -> ResponseError {
        let response = TypeResponseError {
            data: TypeResponseErrorData {
                id: ResponseErrorId::error_types,
                _type: ResponseErrorType::errors,
                attributes: TypeResponseErrorDataAttributes {
                    message: message.to_owned(),
                },
            },
        };

        ResponseError::TypeResponseError(status::NotFound(Json(response)))
    }
}

#[derive(Debug, Serialize)]
pub struct TypeResponseErrorData {
    pub id: ResponseErrorId,
    #[serde(rename = "type")]
    pub _type: ResponseErrorType,
    pub attributes: TypeResponseErrorDataAttributes,
}

#[derive(Debug, Serialize)]
pub struct TypeResponseErrorDataAttributes {
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct TypeResponseErrorInvalid {
    pub data: TypeResponseErrorInvalidData,
}

impl TypeResponseErrorInvalid {
    pub fn new(message: &String) -> ResponseError {
        let response = TypeResponseErrorInvalid {
            data: TypeResponseErrorInvalidData {
                id: ResponseErrorId::error_types_invalid,
                _type: ResponseErrorType::errors,
                attributes: TypeResponseErrorInvalidDataAttributes {
                    message: message.to_owned(),
                },
            },
        };

        ResponseError::TypeResponseErrorInvalid(status::BadRequest(Some(Json(response))))
    }
}

#[derive(Debug, Serialize)]
pub struct TypeResponseErrorInvalidData {
    pub id: ResponseErrorId,
    #[serde(rename = "type")]
    pub _type: ResponseErrorType,
    pub attributes: TypeResponseErrorInvalidDataAttributes,
}

#[derive(Debug, Serialize)]
pub struct TypeResponseErrorInvalidDataAttributes {
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct TypeEffectResponseError {
    pub data: TypeEffectResponseErrorData,
}

impl TypeEffectResponseError {
    pub fn new(message: &String) -> ResponseError {
        let response = TypeEffectResponseError {
            data: TypeEffectResponseErrorData {
                id: ResponseErrorId::error_type_effects,
                _type: ResponseErrorType::errors,
                attributes: TypeEffectResponseErrorDataAttributes {
                    message: message.to_owned(),
                },
            },
        };

        ResponseError::TypeEffectResponseError(status::NotFound(Json(response)))
    }
}

#[derive(Debug, Serialize)]
pub struct TypeEffectResponseErrorData {
    pub id: ResponseErrorId,
    #[serde(rename = "type")]
    pub _type: ResponseErrorType,
    pub attributes: TypeEffectResponseErrorDataAttributes,
}

#[derive(Debug, Serialize)]
pub struct TypeEffectResponseErrorDataAttributes {
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct TypeEffectResponseErrorInvalid {
    pub data: TypeEffectResponseErrorInvalidData,
}

impl TypeEffectResponseErrorInvalid {
    pub fn new(message: &String) -> ResponseError {
        let response = TypeEffectResponseErrorInvalid {
            data: TypeEffectResponseErrorInvalidData {
                id: ResponseErrorId::error_type_effects_invalid,
                _type: ResponseErrorType::errors,
                attributes: TypeEffectResponseErrorInvalidDataAttributes {
                    message: message.to_owned(),
                },
            },
        };

        ResponseError::TypeEffectResponseErrorInvalid(status::BadRequest(Some(Json(response))))
    }
}

#[derive(Debug, Serialize)]
pub struct TypeEffectResponseErrorInvalidData {
    pub id: ResponseErrorId,
    #[serde(rename = "type")]
    pub _type: ResponseErrorType,
    pub attributes: TypeEffectResponseErrorInvalidDataAttributes,
}

#[derive(Debug, Serialize)]
pub struct TypeEffectResponseErrorInvalidDataAttributes {
    pub message: String,
}
