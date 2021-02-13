use rocket_okapi::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::responses::links::Links;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct BaseResponse<T> {
    pub data: BaseResponseData<T>,
    pub links: Links,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct BaseResponseData<T> {
    pub id: String,
    #[serde(rename = "type")]
    pub _type: BaseResponseType,
    pub attributes: T,
    pub links: Links,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct BaseResponseAll<T> {
    pub data: Vec<T>,
    pub links: Links,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[allow(non_camel_case_types)]
pub enum BaseResponseType {
    hm_moves,
    hm_names,
    icons,
    item_names,
    map_pokemon,
    mart_items,
    move_names,
    move_stats,
    player_names,
    pokedex_entries,
    pokedex_texts,
    pokemon_cries,
    pokemon_evolutions,
    pokemon_icons,
    pokemon_learnsets,
    pokemon_machines,
    pokemon_movesets,
    pokemon_names,
    pokemon_stats,
    rom_patches,
    roms,
    sav_player_names,
    savs,
    tm_moves,
    tm_names,
    tm_prices,
    trades,
    trainer_names,
    trainer_parties,
    trainer_rewards,
    type_effects,
    type_names,
}
