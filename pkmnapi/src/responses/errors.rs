use crate::responses::access_tokens::AccessTokenErrorInvalid;
use crate::responses::roms::{
    RomResponseErrorInvalidRom, RomResponseErrorNoRom, RomResponseErrorRomExists,
};
use rocket::response::status;
use rocket_contrib::json::Json;

#[derive(Debug, Responder)]
pub enum ResponseError {
    AccessTokenErrorInvalid(status::BadRequest<Json<AccessTokenErrorInvalid>>),
    RomResponseErrorInvalidRom(status::BadRequest<Json<RomResponseErrorInvalidRom>>),
    RomResponseErrorNoRom(status::BadRequest<Json<RomResponseErrorNoRom>>),
    RomResponseErrorRomExists(status::Forbidden<Json<RomResponseErrorRomExists>>),
}
