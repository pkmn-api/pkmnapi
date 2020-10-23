import { BaseRequest } from './base';

export type PokemonStatsRequest = BaseRequest<PokemonStatsRequestType, PokemonStatsRequestAttributes>;

enum PokemonStatsRequestType {
    pokemon_stats = 'pokemon_stats',
}

export interface PokemonStatsRequestAttributes {
    base_hp: number;
    base_attack: number;
    base_defence: number;
    base_speed: number;
    base_special: number;
    types: Array<PokemonStatsRequestAttributesType>;
    catch_rate: number;
    base_exp_yield: number;
    growth_rate: number;
}

export interface PokemonStatsRequestAttributesType {
    id: string;
}

export const PokemonStatsRequestDefault = (attributes: PokemonStatsRequestAttributes): PokemonStatsRequest => {
    return {
        data: {
            type: PokemonStatsRequestType.pokemon_stats,
            attributes,
        },
    };
};
