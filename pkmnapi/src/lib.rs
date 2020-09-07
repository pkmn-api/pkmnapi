#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_contrib;

pub mod guards;
pub mod requests;
pub mod responses;
pub mod routes;
pub mod utils;

use pkmnapi_sql::*;
use rocket::Rocket;

pub struct Pkmnapi {}

impl Pkmnapi {
    pub fn init() -> Rocket {
        let sql = PkmnapiSQL::new();

        rocket::ignite()
            .manage(sql)
            .mount("/", routes![routes::status::status,])
            .mount(
                "/v1",
                routes![
                    routes::access_tokens::post_access_token,
                    routes::roms::post_rom,
                    routes::roms::get_rom,
                    routes::roms::delete_rom,
                    routes::types::get_type,
                    routes::types::post_type
                ],
            )
    }
}
