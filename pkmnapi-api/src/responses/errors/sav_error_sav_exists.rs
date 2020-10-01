use rocket::response::status;
use rocket_contrib::json::Json;
use serde::Serialize;

use crate::responses::errors::*;

pub type SavErrorSavExists = BaseErrorResponse<SavErrorSavExistsAttributes>;

impl SavErrorSavExists {
    pub fn new() -> ResponseError {
        let response = SavErrorSavExists {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_savs_sav_exists,
                _type: BaseErrorResponseType::errors,
                attributes: SavErrorSavExistsAttributes {
                    message: "SAV already exists".to_owned(),
                },
            },
        };

        ResponseError::SavErrorSavExists(status::Forbidden(Some(Json(response))))
    }
}

#[derive(Debug, Serialize)]
pub struct SavErrorSavExistsAttributes {
    pub message: String,
}
