use rocket::response::status;
use rocket_contrib::json::Json;
use serde::Serialize;

#[derive(Debug, Responder)]
pub enum ResponseError {
    AccessTokenErrorInvalid(status::BadRequest<Json<AccessTokenErrorInvalid>>),
    RomResponseErrorInvalidRom(status::BadRequest<Json<RomResponseErrorInvalidRom>>),
    RomResponseErrorNoRom(status::BadRequest<Json<RomResponseErrorNoRom>>),
    RomResponseErrorRomExists(status::Forbidden<Json<RomResponseErrorRomExists>>),
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
    error_roms_invalid_rom,
    error_roms_no_rom,
    error_roms_rom_exists,
}

#[derive(Debug, Serialize)]
pub struct AccessTokenErrorInvalid {
    pub data: AccessTokenErrorInvalidData,
}

impl AccessTokenErrorInvalid {
    pub fn new(message: &String) -> Self {
        AccessTokenErrorInvalid {
            data: AccessTokenErrorInvalidData {
                id: ResponseErrorId::error_access_tokens_invalid,
                _type: ResponseErrorType::errors,
                attributes: AccessTokenErrorInvalidDataAttributes {
                    message: message.to_string(),
                },
            },
        }
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
pub struct RomResponseErrorNoRom {
    pub data: RomResponseErrorNoRomData,
}

impl RomResponseErrorNoRom {
    pub fn new() -> Self {
        RomResponseErrorNoRom {
            data: RomResponseErrorNoRomData {
                id: ResponseErrorId::error_roms_no_rom,
                _type: ResponseErrorType::errors,
                attributes: RomResponseErrorNoRomDataAttributes {
                    message: String::from("No ROM uploaded"),
                },
            },
        }
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
    pub fn new() -> Self {
        RomResponseErrorInvalidRom {
            data: RomResponseErrorInvalidRomData {
                id: ResponseErrorId::error_roms_invalid_rom,
                _type: ResponseErrorType::errors,
                attributes: RomResponseErrorInvalidRomDataAttributes {
                    message: String::from("Invalid ROM provided"),
                },
            },
        }
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
    pub fn new() -> Self {
        RomResponseErrorRomExists {
            data: RomResponseErrorRomExistsData {
                id: ResponseErrorId::error_roms_rom_exists,
                _type: ResponseErrorType::errors,
                attributes: RomResponseErrorRomExistsDataAttributes {
                    message: String::from("ROM already exists"),
                },
            },
        }
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
