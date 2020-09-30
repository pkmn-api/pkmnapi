use rocket::response::status;
use rocket_contrib::json::Json;
use serde::Serialize;

use crate::responses::errors::*;

pub type ItemNameResponseErrorInvalid = BaseErrorResponse<ItemNameResponseErrorInvalidAttributes>;

impl ItemNameResponseErrorInvalid {
    pub fn new(message: &String) -> ResponseError {
        let response = ItemNameResponseErrorInvalid {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_item_names_invalid,
                _type: BaseErrorResponseType::errors,
                attributes: ItemNameResponseErrorInvalidAttributes {
                    message: message.to_owned(),
                },
            },
        };

        ResponseError::ItemNameResponseErrorInvalid(status::BadRequest(Some(Json(response))))
    }
}

#[derive(Debug, Serialize)]
pub struct ItemNameResponseErrorInvalidAttributes {
    pub message: String,
}
