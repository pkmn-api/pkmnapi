import { BaseResponse, BaseResponseData, BaseResponseAll } from './base';
import { ItemNameResponseData } from './item_names';
import { PokemonNameResponseData } from './pokemon_names';

export type PokemonEvolutionsResponse = BaseResponse<PokemonEvolutionsResponseAttributes>;
export type PokemonEvolutionsResponseData = BaseResponseData<PokemonEvolutionsResponseAttributes>;
export type PokemonEvolutionsResponseAll = BaseResponseAll<PokemonEvolutionsResponseData>;

export interface PokemonEvolutionsResponseAttributes {
    evolution: Array<PokemonEvolutionsResponseAttributesEvolution>;
}

export interface PokemonEvolutionsResponseAttributesEvolution {
    evolution_type: PokemonEvolutionsResponseAttributesEvolutionType;
    level?: number;
    item?: ItemNameResponseData;
    pokemon: PokemonNameResponseData;
}

enum PokemonEvolutionsResponseAttributesEvolutionType {
    level = 'level',
    item = 'item',
    trade = 'trade',
}
