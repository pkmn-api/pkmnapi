use rocket::response::status;
use rocket_contrib::json::Json;
use serde::Serialize;

#[derive(Debug, Responder)]
pub enum ResponseError {
    AccessTokenErrorInvalid(status::BadRequest<Json<AccessTokenErrorInvalid>>),
    AccessTokenErrorUnauthorized(status::Unauthorized<Json<AccessTokenErrorUnauthorized>>),
    AccessTokenErrorForbidden(status::Forbidden<Json<AccessTokenErrorForbidden>>),
    RomResponseErrorInvalidRom(status::BadRequest<Json<RomResponseErrorInvalidRom>>),
    RomResponseErrorNoRom(status::Forbidden<Json<RomResponseErrorNoRom>>),
    RomResponseErrorRomExists(status::Forbidden<Json<RomResponseErrorRomExists>>),
    TypeResponseError(status::NotFound<Json<TypeResponseError>>),
    TypeResponseErrorInvalid(status::BadRequest<Json<TypeResponseErrorInvalid>>),
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
    error_roms_invalid_rom,
    error_roms_no_rom,
    error_roms_rom_exists,
    error_types,
    error_types_invalid,
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
