use rocket::response::status;
use rocket_contrib::json::Json;
use serde::Serialize;

use crate::responses::errors::*;

pub type RomErrorInvalidRom = BaseErrorResponse<RomErrorInvalidRomAttributes>;

impl RomErrorInvalidRom {
    pub fn new() -> ResponseError {
        let response = RomErrorInvalidRom {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_roms_invalid_rom,
                _type: BaseErrorResponseType::errors,
                attributes: RomErrorInvalidRomAttributes {
                    message: "Invalid ROM provided".to_owned(),
                },
            },
        };

        ResponseError::RomErrorInvalidRom(status::BadRequest(Some(Json(response))))
    }
}

#[derive(Debug, Serialize)]
pub struct RomErrorInvalidRomAttributes {
    pub message: String,
}
