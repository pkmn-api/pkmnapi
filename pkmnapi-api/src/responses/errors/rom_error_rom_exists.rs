use rocket::response::status;
use rocket_contrib::json::Json;
use serde::Serialize;

use crate::responses::errors::*;

pub type RomErrorRomExists = BaseErrorResponse<RomErrorRomExistsAttributes>;

impl RomErrorRomExists {
    pub fn new() -> ResponseError {
        let response = RomErrorRomExists {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_roms_rom_exists,
                _type: BaseErrorResponseType::errors,
                attributes: RomErrorRomExistsAttributes {
                    message: "ROM already exists".to_owned(),
                },
            },
        };

        ResponseError::RomErrorRomExists(status::Forbidden(Some(Json(response))))
    }
}

#[derive(Debug, Serialize)]
pub struct RomErrorRomExistsAttributes {
    pub message: String,
}
