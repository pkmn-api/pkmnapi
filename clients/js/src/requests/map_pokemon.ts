import { BaseRequest } from './base';

export type MapPokemonRequest = BaseRequest<MapPokemonRequestType, MapPokemonRequestAttributes>;

enum MapPokemonRequestType {
    map_pokemon = 'map_pokemon',
}

export interface MapPokemonRequestAttributes {
    grass: MapPokemonRequestAttributesArea;
    water: MapPokemonRequestAttributesArea;
}

export interface MapPokemonRequestAttributesArea {
    encounter_rate: number;
    pokemon: Array<MapPokemonRequestAttributesPokemon>;
}

export interface MapPokemonRequestAttributesPokemon {
    level: number;
    pokemon: MapPokemonRequestAttributesPokemonInfo;
}

export interface MapPokemonRequestAttributesPokemonInfo {
    id: string;
}

export const MapPokemonRequestDefault = (attributes: MapPokemonRequestAttributes): MapPokemonRequest => {
    return {
        data: {
            type: MapPokemonRequestType.map_pokemon,
            attributes,
        },
    };
};
