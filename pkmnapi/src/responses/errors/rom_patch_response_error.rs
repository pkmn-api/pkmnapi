use rocket::response::status;
use rocket_contrib::json::Json;
use serde::Serialize;

use crate::responses::errors::*;

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
