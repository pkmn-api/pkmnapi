use rocket::response::status;
use rocket_contrib::json::Json;
use serde::Serialize;

use crate::responses::errors::*;

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
