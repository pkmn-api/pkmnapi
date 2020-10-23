import { BaseResponse, BaseResponseData, BaseResponseAll } from './base';

export type PokemonNameResponse = BaseResponse<PokemonNameResponseAttributes>;
export type PokemonNameResponseData = BaseResponseData<PokemonNameResponseAttributes>;
export type PokemonNameResponseAll = BaseResponseAll<PokemonNameResponseData>;

export interface PokemonNameResponseAttributes {
    name: string;
}
