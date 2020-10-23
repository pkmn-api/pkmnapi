import { BaseRequest } from './base';

export type PokemonLearnsetRequest = BaseRequest<PokemonLearnsetRequestType, PokemonLearnsetRequestAttributes>;

enum PokemonLearnsetRequestType {
    pokemon_learnsets = 'pokemon_learnsets',
}

export interface PokemonLearnsetRequestAttributes {
    learnset: Array<PokemonLearnsetRequestAttributesLearnset>;
}

export interface PokemonLearnsetRequestAttributesLearnset {
    level: number;
    move: PokemonLearnsetRequestAttributesLearnsetMove;
}

export interface PokemonLearnsetRequestAttributesLearnsetMove {
    id: string;
}

export const PokemonLearnsetRequestDefault = (attributes: PokemonLearnsetRequestAttributes): PokemonLearnsetRequest => {
    return {
        data: {
            type: PokemonLearnsetRequestType.pokemon_learnsets,
            attributes,
        },
    };
};
