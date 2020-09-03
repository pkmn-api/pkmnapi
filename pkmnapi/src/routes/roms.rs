use pkmnapi_db::*;
use rocket::response::status;
use rocket::Data;
use rocket_contrib::json::Json;

use crate::responses::links::Links;
use crate::responses::roms::{RomResponse, RomResponseAttributes};

#[post("/roms", data = "<data>")]
pub fn post_rom(data: Data) -> status::Created<Json<RomResponse>> {
    let mut rom = Vec::new();

    data.stream_to(&mut rom).unwrap();

    let db = PkmnapiDB::new(&rom).unwrap();
    let rom = RomResponse {
        id: "1337".to_string(),
        _type: "roms".to_string(),
        attributes: RomResponseAttributes {
            name: db.header.title,
            hash: db.hash,
            valid: true,
        },
        links: Links {
            _self: "foo".to_string(),
        },
    };

    status::Created("foo".to_string(), Some(Json(rom)))
}
