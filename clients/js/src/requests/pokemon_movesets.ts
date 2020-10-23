import { BaseRequest } from './base';

export type PokemonMovesetRequest = BaseRequest<PokemonMovesetRequestType, PokemonMovesetRequestAttributes>;

enum PokemonMovesetRequestType {
    pokemon_movesets = 'pokemon_movesets',
}

export interface PokemonMovesetRequestAttributes {
    moveset: Array<PokemonMovesetRequestAttributesMoveset>;
}

export interface PokemonMovesetRequestAttributesMoveset {
    move: PokemonMovesetRequestAttributesMovesetMove;
}

export interface PokemonMovesetRequestAttributesMovesetMove {
    id: string;
}

export const PokemonMovesetRequestDefault = (attributes: PokemonMovesetRequestAttributes): PokemonMovesetRequest => {
    return {
        data: {
            type: PokemonMovesetRequestType.pokemon_movesets,
            attributes,
        },
    };
};
