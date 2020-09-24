use rocket::Request;

use crate::responses::errors::*;

#[catch(404)]
pub fn not_found(_req: &Request) -> Result<ResponseError, ResponseError> {
    Ok(NotFoundError::new())
}
