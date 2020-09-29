use serde::Serialize;

use crate::responses::links::Links;

#[derive(Debug, Serialize)]
pub struct BaseResponse<T> {
    pub data: BaseResponseData<T>,
    pub links: Links,
}

#[derive(Debug, Serialize)]
pub struct BaseResponseData<T> {
    pub id: String,
    #[serde(rename = "type")]
    pub _type: BaseResponseType,
    pub attributes: T,
    pub links: Links,
}

#[derive(Debug, Serialize)]
pub struct BaseResponseAll<T> {
    pub data: Vec<T>,
    pub links: Links,
}

#[derive(Debug, Serialize)]
#[allow(non_camel_case_types)]
pub enum BaseResponseType {
    hms,
    move_names,
    pokedex_entries,
    pokedex_texts,
    pokemon_names,
    pokemon_stats,
    rom_patches,
    roms,
    sav_player_names,
    savs,
    tms,
    trainer_names,
    trainer_parties,
    type_effects,
    type_names,
}
