import { BaseRequest } from './base';

export type PokemonEvolutionsRequest = BaseRequest<PokemonEvolutionsRequestType, PokemonEvolutionsRequestAttributes>;

enum PokemonEvolutionsRequestType {
    pokemon_evolutions = 'pokemon_evolutions',
}

export interface PokemonEvolutionsRequestAttributes {
    evolutions: Array<
        | PokemonEvolutionsRequestAttributesEvolutionLevel
        | PokemonEvolutionsRequestAttributesEvolutionItem
        | PokemonEvolutionsRequestAttributesEvolutionTrade
    >;
}

export interface PokemonEvolutionsRequestAttributesEvolutionLevel {
    evolution_type: PokemonEvolutionsRequestAttributesEvolutionLevelType | string;
    level: number;
    pokemon: PokemonEvolutionsRequestAttributesEvolutionPokemon;
}

enum PokemonEvolutionsRequestAttributesEvolutionLevelType {
    level = 'level',
}

export interface PokemonEvolutionsRequestAttributesEvolutionItem {
    evolution_type: PokemonEvolutionsRequestAttributesEvolutionItemType | string;
    item: PokemonEvolutionsRequestAttributesEvolutionItemAttributes;
    pokemon: PokemonEvolutionsRequestAttributesEvolutionPokemon;
}

enum PokemonEvolutionsRequestAttributesEvolutionItemType {
    item = 'item',
}

export interface PokemonEvolutionsRequestAttributesEvolutionTrade {
    evolution_type: PokemonEvolutionsRequestAttributesEvolutionTradeType | string;
    pokemon: PokemonEvolutionsRequestAttributesEvolutionPokemon;
}

enum PokemonEvolutionsRequestAttributesEvolutionTradeType {
    trade = 'trade',
}

export interface PokemonEvolutionsRequestAttributesEvolutionItemAttributes {
    id: string;
}

export interface PokemonEvolutionsRequestAttributesEvolutionPokemon {
    id: string;
}

export const PokemonEvolutionsRequestDefault = (
    attributes: PokemonEvolutionsRequestAttributes,
): PokemonEvolutionsRequest => {
    return {
        data: {
            type: PokemonEvolutionsRequestType.pokemon_evolutions,
            attributes,
        },
    };
};
