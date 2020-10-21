use serde::Deserialize;

use crate::requests::base::BaseRequest;

pub type AccessTokenRequest = BaseRequest<AccessTokenRequestType, AccessTokenRequestAttributes>;
pub type AccessTokenDeleteRequest =
    BaseRequest<AccessTokenRequestType, AccessTokenDeleteRequestAttributes>;

impl AccessTokenRequest {
    pub fn get_email_address(&self) -> &String {
        &self.data.attributes.email_address
    }
}

impl AccessTokenDeleteRequest {
    pub fn get_code(&self) -> &String {
        &self.data.attributes.code
    }

    pub fn get_email_address(&self) -> &String {
        &self.data.attributes.email_address
    }
}

#[derive(Debug, Deserialize)]
#[allow(non_camel_case_types)]
pub enum AccessTokenRequestType {
    access_tokens,
}

#[derive(Debug, Deserialize)]
pub struct AccessTokenRequestAttributes {
    pub email_address: String,
}

#[derive(Debug, Deserialize)]
pub struct AccessTokenDeleteRequestAttributes {
    pub code: String,
    pub email_address: String,
}
