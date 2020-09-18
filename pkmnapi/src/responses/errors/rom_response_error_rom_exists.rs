use rocket::response::status;
use rocket_contrib::json::Json;
use serde::Serialize;

use crate::responses::errors::*;

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
