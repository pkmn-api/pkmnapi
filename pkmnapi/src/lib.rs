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
use rocket::fairing::AdHoc;
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
                    routes::map_pics::get_map_pic_jpeg,
                    routes::map_pics::get_map_pic_png,
                    routes::moves::get_move,
                    routes::moves::post_move,
                    routes::pokemon_names::get_pokemon_name,
                    routes::pokemon_names::post_pokemon_name,
                    routes::pokemon_pics::get_pokemon_pic_jpeg,
                    routes::pokemon_pics::get_pokemon_pic_png,
                    routes::pokemon_pics::post_pokemon_pic_jpeg,
                    routes::pokemon_pics::post_pokemon_pic_png,
                    routes::rom_patches::delete_rom_patch,
                    routes::rom_patches::get_rom_patch,
                    routes::rom_patches::get_rom_patches_raw,
                    routes::rom_patches::get_rom_patches,
                    routes::roms::delete_rom,
                    routes::roms::get_rom,
                    routes::roms::post_rom,
                    routes::sav_player_names::get_sav_player_name,
                    routes::sav_player_names::post_sav_player_name,
                    routes::savs::delete_sav,
                    routes::savs::get_sav,
                    routes::savs::post_sav,
                    routes::stats::get_stats,
                    routes::stats::post_stats,
                    routes::tms::get_tm,
                    routes::tms::post_tm,
                    routes::trainer_names::get_trainer_name,
                    routes::trainer_names::post_trainer_name,
                    routes::trainer_parties::get_trainer_parties,
                    routes::trainer_parties::post_trainer_parties,
                    routes::trainer_pics::get_trainer_pic_jpeg,
                    routes::trainer_pics::get_trainer_pic_png,
                    routes::trainer_pics::post_trainer_pic_jpeg,
                    routes::trainer_pics::post_trainer_pic_png,
                    routes::type_effects::get_type_effect,
                    routes::type_effects::post_type_effect,
                    routes::types::get_type,
                    routes::types::post_type,
                ],
            )
            .attach(AdHoc::on_response("Update Server Name", |_, res| {
                res.set_raw_header("Server", concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")));
            }))
    }
}
