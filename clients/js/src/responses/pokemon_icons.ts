import { BaseResponse, BaseResponseData, BaseResponseAll, BaseResponseType } from './base';
import { Links } from './links';

export type PokemonIconResponse = BaseResponse<PokemonIconResponseAttributes>;
export type PokemonIconResponseData = BaseResponseData<PokemonIconResponseAttributes>;
export type PokemonIconResponseAll = BaseResponseAll<PokemonIconResponseData>;

export interface PokemonIconResponseAttributes {
    icon: PokemonIconResponseAttributesIcon;
}

export interface PokemonIconResponseAttributesIcon {
    id: string;
    type: BaseResponseType;
    attributes: unknown;
    links: Links;
}
