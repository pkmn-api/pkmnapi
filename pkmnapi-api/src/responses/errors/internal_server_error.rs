use serde::Serialize;

use crate::responses::errors::*;

pub type InternalServerError = BaseErrorResponse<InternalServerErrorAttributes>;

impl InternalServerError {
    pub fn new() -> Self {
        InternalServerError {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_internal_server,
                _type: BaseErrorResponseType::errors,
                attributes: InternalServerErrorAttributes {
                    message: "An unknown error has occurred".to_owned(),
                },
            },
        }
    }
}

#[derive(Debug, Serialize)]
pub struct InternalServerErrorAttributes {
    pub message: String,
}
