use rocket::response::status;
use rocket_contrib::json::Json;
use serde::Serialize;

use crate::responses::errors::*;

pub type ItemNameResponseError = BaseErrorResponse<ItemNameResponseErrorAttributes>;

impl ItemNameResponseError {
    pub fn new(message: &String) -> ResponseError {
        let response = ItemNameResponseError {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_item_names,
                _type: BaseErrorResponseType::errors,
                attributes: ItemNameResponseErrorAttributes {
                    message: message.to_owned(),
                },
            },
        };

        ResponseError::ItemNameResponseError(status::NotFound(Json(response)))
    }
}

#[derive(Debug, Serialize)]
pub struct ItemNameResponseErrorAttributes {
    pub message: String,
}
