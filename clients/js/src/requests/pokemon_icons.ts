import { BaseRequest } from './base';

export type PokemonIconRequest = BaseRequest<PokemonIconRequestType, PokemonIconRequestAttributes>;

enum PokemonIconRequestType {
    pokemon_icons = 'pokemon_icons',
}

export interface PokemonIconRequestAttributes {
    icon: PokemonIconRequestAttributesIcon;
}

export interface PokemonIconRequestAttributesIcon {
    id: string;
}

export const PokemonIconRequestDefault = (attributes: PokemonIconRequestAttributes): PokemonIconRequest => {
    return {
        data: {
            type: PokemonIconRequestType.pokemon_icons,
            attributes,
        },
    };
};
