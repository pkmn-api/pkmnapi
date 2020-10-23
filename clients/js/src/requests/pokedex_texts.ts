import { BaseRequest } from './base';

export type PokedexTextRequest = BaseRequest<PokedexTextRequestType, PokedexTextRequestAttributes>;

enum PokedexTextRequestType {
    pokedex_texts = 'pokedex_texts',
}

export interface PokedexTextRequestAttributes {
    text: string;
}

export const PokedexTextRequestDefault = (attributes: PokedexTextRequestAttributes): PokedexTextRequest => {
    return {
        data: {
            type: PokedexTextRequestType.pokedex_texts,
            attributes,
        },
    };
};
