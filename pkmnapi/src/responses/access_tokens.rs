use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AccessTokenInvalid {
    pub data: AccessTokenInvalidData,
}

impl AccessTokenInvalid {
    pub fn new(message: &String) -> Self {
        AccessTokenInvalid {
            data: AccessTokenInvalidData {
                id: String::from("error_access_tokens_invalid_data"),
                _type: AccessTokenRequestDataType::errors,
                attributes: AccessTokenInvalidDataAttributes {
                    message: message.to_string(),
                },
            },
        }
    }
}

#[derive(Debug, Serialize)]
pub struct AccessTokenInvalidData {
    pub id: String,
    #[serde(rename = "type")]
    pub _type: AccessTokenRequestDataType,
    pub attributes: AccessTokenInvalidDataAttributes,
}

#[derive(Debug, Serialize)]
#[allow(non_camel_case_types)]
pub enum AccessTokenRequestDataType {
    errors,
}

#[derive(Debug, Serialize)]
pub struct AccessTokenInvalidDataAttributes {
    pub message: String,
}
