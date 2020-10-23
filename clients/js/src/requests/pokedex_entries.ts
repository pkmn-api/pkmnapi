import { BaseRequest } from './base';

export type PokedexEntryRequest = BaseRequest<PokedexEntryRequestType, PokedexEntryRequestAttributes>;

enum PokedexEntryRequestType {
    pokedex_entries = 'pokedex_entries',
}

export interface PokedexEntryRequestAttributes {
    species: string;
    height: number;
    weight: number;
}

export const PokedexEntryRequestDefault = (attributes: PokedexEntryRequestAttributes): PokedexEntryRequest => {
    return {
        data: {
            type: PokedexEntryRequestType.pokedex_entries,
            attributes,
        },
    };
};
