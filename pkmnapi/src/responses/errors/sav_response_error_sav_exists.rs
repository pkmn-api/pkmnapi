use rocket::response::status;
use rocket_contrib::json::Json;
use serde::Serialize;

use crate::responses::errors::*;

pub type SavResponseErrorSavExists = BaseErrorResponse<SavResponseErrorSavExistsAttributes>;

impl SavResponseErrorSavExists {
    pub fn new() -> ResponseError {
        let response = SavResponseErrorSavExists {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_savs_sav_exists,
                _type: BaseErrorResponseType::errors,
                attributes: SavResponseErrorSavExistsAttributes {
                    message: "SAV already exists".to_owned(),
                },
            },
        };

        ResponseError::SavResponseErrorSavExists(status::Forbidden(Some(Json(response))))
    }
}

#[derive(Debug, Serialize)]
pub struct SavResponseErrorSavExistsAttributes {
    pub message: String,
}
