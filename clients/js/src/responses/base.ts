import { Links } from './links';

export interface BaseResponse<T> {
    data: BaseResponseData<T>;
    links: Links;
}

export interface BaseResponseData<T> {
    id: string;
    type: BaseResponseType;
    attributes: T;
    links: Links;
}

export interface BaseResponseAll<T> {
    data: Array<T>;
    links: Links;
}

export enum BaseResponseType {
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
