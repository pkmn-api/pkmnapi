import { BaseRequest } from './base';

export type PokemonNameRequest = BaseRequest<PokemonNameRequestType, PokemonNameRequestAttributes>;

enum PokemonNameRequestType {
    pokemon_names = 'pokemon_names',
}

export interface PokemonNameRequestAttributes {
    name: string;
}

export const PokemonNameRequestDefault = (attributes: PokemonNameRequestAttributes): PokemonNameRequest => {
    return {
        data: {
            type: PokemonNameRequestType.pokemon_names,
            attributes,
        },
    };
};
