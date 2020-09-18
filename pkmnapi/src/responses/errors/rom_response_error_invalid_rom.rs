use rocket::response::status;
use rocket_contrib::json::Json;
use serde::Serialize;

use crate::responses::errors::*;

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
