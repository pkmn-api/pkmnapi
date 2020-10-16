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

use governor::clock::DefaultClock;
use governor::state::keyed::HashMapStateStore;
use governor::{Quota, RateLimiter};
use pkmnapi_sql::*;
use rocket::fairing::AdHoc;
use rocket::Rocket;
use rocket_cors::AllowedHeaders;
use std::env;
use std::num::NonZeroU32;
use std::time::Duration;

pub struct Pkmnapi {}

impl Pkmnapi {
    pub fn init() -> Rocket {
        let sql = PkmnapiSQL::new();
        let lim: RateLimiter<String, HashMapStateStore<String>, DefaultClock> =
            RateLimiter::hashmap({
                let duration = env::var("RATE_LIMIT_DURATION")
                    .unwrap_or("60".to_owned())
                    .parse::<u64>()
                    .unwrap_or(60);
                let count = env::var("RATE_LIMIT_COUNT")
                    .unwrap_or("120".to_owned())
                    .parse::<u32>()
                    .unwrap_or(120);

                Quota::with_period(Duration::from_secs(duration))
                    .unwrap()
                    .allow_burst(NonZeroU32::new(count).unwrap())
            });
        let cors = rocket_cors::CorsOptions {
            allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
            allow_credentials: true,
            ..Default::default()
        }
        .to_cors()
        .unwrap();

        rocket::ignite()
            .manage(sql)
            .manage(lim)
            .mount("/", routes![routes::status::status,])
            .mount(
                "/v1",
                routes![
                    routes::access_tokens::post_access_token,
                    routes::hm_moves::get_hm_move_all,
                    routes::hm_moves::get_hm_move,
                    routes::hm_moves::post_hm_move,
                    routes::icons::get_icon,
                    routes::imgs::get_pokemon_logo_jpeg,
                    routes::imgs::get_pokemon_logo_png,
                    routes::imgs::get_town_map_jpeg,
                    routes::imgs::get_town_map_png,
                    routes::imgs::post_pokemon_logo_jpeg,
                    routes::imgs::post_pokemon_logo_png,
                    routes::item_names::get_item_name_all,
                    routes::item_names::get_item_name,
                    routes::item_names::post_item_name,
                    routes::map_pics::get_map_pic_jpeg,
                    routes::map_pics::get_map_pic_png,
                    routes::map_pokemon::get_map_pokemon_all,
                    routes::map_pokemon::get_map_pokemon,
                    routes::map_pokemon::post_map_pokemon,
                    routes::move_names::get_move_name_all,
                    routes::move_names::get_move_name,
                    routes::move_names::post_move_name,
                    routes::player_names::get_player_names,
                    routes::player_names::post_player_names,
                    routes::pokedex_entries::get_pokedex_entry_all,
                    routes::pokedex_entries::get_pokedex_entry,
                    routes::pokedex_entries::post_pokedex_entry,
                    routes::pokedex_texts::get_pokedex_text_all,
                    routes::pokedex_texts::get_pokedex_text,
                    routes::pokedex_texts::post_pokedex_text,
                    routes::pokemon_cries::get_pokemon_cry_all,
                    routes::pokemon_cries::get_pokemon_cry_json,
                    routes::pokemon_cries::get_pokemon_cry_wav,
                    routes::pokemon_cries::post_pokemon_cry,
                    routes::pokemon_evolutions::get_pokemon_evolutions_all,
                    routes::pokemon_evolutions::get_pokemon_evolutions,
                    routes::pokemon_evolutions::post_pokemon_evolutions,
                    routes::pokemon_icons::get_pokemon_icon_all,
                    routes::pokemon_icons::get_pokemon_icon,
                    routes::pokemon_icons::post_pokemon_icon,
                    routes::pokemon_learnsets::get_pokemon_learnset_all,
                    routes::pokemon_learnsets::get_pokemon_learnset,
                    routes::pokemon_learnsets::post_pokemon_learnset,
                    routes::pokemon_machines::get_pokemon_machines_all,
                    routes::pokemon_machines::get_pokemon_machines,
                    routes::pokemon_machines::post_pokemon_machines,
                    routes::pokemon_names::get_pokemon_name_all,
                    routes::pokemon_names::get_pokemon_name,
                    routes::pokemon_names::post_pokemon_name,
                    routes::pokemon_pics::get_pokemon_pic_jpeg,
                    routes::pokemon_pics::get_pokemon_pic_png,
                    routes::pokemon_pics::post_pokemon_pic_jpeg,
                    routes::pokemon_pics::post_pokemon_pic_png,
                    routes::pokemon_stats::get_pokemon_stats_all,
                    routes::pokemon_stats::get_pokemon_stats,
                    routes::pokemon_stats::post_pokemon_stats,
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
                    routes::tm_moves::get_tm_move_all,
                    routes::tm_moves::get_tm_move,
                    routes::tm_moves::post_tm_move,
                    routes::tm_prices::get_tm_price_all,
                    routes::tm_prices::get_tm_price,
                    routes::tm_prices::post_tm_price,
                    routes::trainer_names::get_trainer_name_all,
                    routes::trainer_names::get_trainer_name,
                    routes::trainer_names::post_trainer_name,
                    routes::trainer_parties::get_trainer_parties_all,
                    routes::trainer_parties::get_trainer_parties,
                    routes::trainer_parties::post_trainer_parties,
                    routes::trainer_pics::get_trainer_pic_jpeg,
                    routes::trainer_pics::get_trainer_pic_png,
                    routes::trainer_pics::post_trainer_pic_jpeg,
                    routes::trainer_pics::post_trainer_pic_png,
                    routes::trainer_rewards::get_trainer_reward_all,
                    routes::trainer_rewards::get_trainer_reward,
                    routes::trainer_rewards::post_trainer_reward,
                    routes::type_effects::get_type_effect_all,
                    routes::type_effects::get_type_effect,
                    routes::type_effects::post_type_effect,
                    routes::type_names::get_type_name_all,
                    routes::type_names::get_type_name,
                    routes::type_names::post_type_name,
                ],
            )
            .register(catchers![
                routes::errors::not_found,
                routes::errors::too_many_requests,
                routes::errors::internal_server_error
            ])
            .attach(AdHoc::on_response("Update Server Name", |_, res| {
                res.set_raw_header("Server", concat!("pkmnapi/", env!("CARGO_PKG_VERSION")));
            }))
            .attach(cors)
    }
}
