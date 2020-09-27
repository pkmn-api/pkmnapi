use serde::Serialize;

use crate::responses::errors::*;

pub type TooManyRequestsError = BaseErrorResponse<TooManyRequestsErrorAttributes>;

impl TooManyRequestsError {
    pub fn new(wait_time: u64) -> Self {
        TooManyRequestsError {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_too_many_requests,
                _type: BaseErrorResponseType::errors,
                attributes: TooManyRequestsErrorAttributes {
                    message: format!(
                        "Too many requests. Please try again in {} seconds.",
                        wait_time
                    ),
                },
            },
        }
    }
}

#[derive(Debug, Serialize)]
pub struct TooManyRequestsErrorAttributes {
    pub message: String,
}
