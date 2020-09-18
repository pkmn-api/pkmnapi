use rocket::response::status;
use rocket_contrib::json::Json;
use serde::Serialize;

use crate::responses::errors::*;

pub type SavResponseErrorInvalidSav = BaseErrorResponse<SavResponseErrorInvalidSavAttributes>;

impl SavResponseErrorInvalidSav {
    pub fn new() -> ResponseError {
        let response = SavResponseErrorInvalidSav {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_savs_invalid_sav,
                _type: BaseErrorResponseType::errors,
                attributes: SavResponseErrorInvalidSavAttributes {
                    message: "Invalid SAV provided".to_owned(),
                },
            },
        };

        ResponseError::SavResponseErrorInvalidSav(status::BadRequest(Some(Json(response))))
    }
}

#[derive(Debug, Serialize)]
pub struct SavResponseErrorInvalidSavAttributes {
    pub message: String,
}
