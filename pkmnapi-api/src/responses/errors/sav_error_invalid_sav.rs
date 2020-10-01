use rocket::response::status;
use rocket_contrib::json::Json;
use serde::Serialize;

use crate::responses::errors::*;

pub type SavErrorInvalidSav = BaseErrorResponse<SavErrorInvalidSavAttributes>;

impl SavErrorInvalidSav {
    pub fn new() -> ResponseError {
        let response = SavErrorInvalidSav {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_savs_invalid_sav,
                _type: BaseErrorResponseType::errors,
                attributes: SavErrorInvalidSavAttributes {
                    message: "Invalid SAV provided".to_owned(),
                },
            },
        };

        ResponseError::SavErrorInvalidSav(status::BadRequest(Some(Json(response))))
    }
}

#[derive(Debug, Serialize)]
pub struct SavErrorInvalidSavAttributes {
    pub message: String,
}
