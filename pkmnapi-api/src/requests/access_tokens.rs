use serde::Deserialize;

use crate::requests::base::BaseRequest;

pub type AccessTokenRequest = BaseRequest<AccessTokenRequestType, AccessTokenRequestAttributes>;

impl AccessTokenRequest {
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
