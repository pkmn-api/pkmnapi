#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

pub mod responses;
pub mod routes;

use rocket::Rocket;

pub struct Pkmnapi {}

impl Pkmnapi {
    pub fn init() -> Rocket {
        rocket::ignite()
            .mount("/", routes![routes::status::status,])
            .mount("/v1", routes![routes::roms::post_rom,])
    }
}
