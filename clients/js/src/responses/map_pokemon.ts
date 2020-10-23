import { BaseResponse, BaseResponseData, BaseResponseAll } from './base';
import { PokemonNameResponseData } from './pokemon_names';

export type MapPokemonResponse = BaseResponse<MapPokemonResponseAttributes>;
export type MapPokemonResponseData = BaseResponseData<MapPokemonResponseAttributes>;
export type MapPokemonResponseAll = BaseResponseAll<MapPokemonResponseData>;

export interface MapPokemonResponseAttributes {
    grass: MapPokemonResponseAttributesArea;
    water: MapPokemonResponseAttributesArea;
}

export interface MapPokemonResponseAttributesArea {
    encounter_rate: number;
    pokemon: Array<MapPokemonResponseAttributesPokemon>;
}

export interface MapPokemonResponseAttributesPokemon {
    level: number;
    pokemon: PokemonNameResponseData;
}
