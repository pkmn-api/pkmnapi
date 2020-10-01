use rocket::response::status;
use rocket_contrib::json::Json;
use serde::Serialize;

use crate::responses::errors::*;

pub type RomErrorNoRom = BaseErrorResponse<RomErrorNoRomAttributes>;

impl RomErrorNoRom {
    pub fn new() -> ResponseError {
        let response = RomErrorNoRom {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_roms_no_rom,
                _type: BaseErrorResponseType::errors,
                attributes: RomErrorNoRomAttributes {
                    message: "No ROM uploaded".to_owned(),
                },
            },
        };

        ResponseError::RomErrorNoRom(status::Forbidden(Some(Json(response))))
    }
}

#[derive(Debug, Serialize)]
pub struct RomErrorNoRomAttributes {
    pub message: String,
}
