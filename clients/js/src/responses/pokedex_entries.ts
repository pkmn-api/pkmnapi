import { BaseResponse, BaseResponseData, BaseResponseAll } from './base';

export type PokedexEntryResponse = BaseResponse<PokedexEntryResponseAttributes>;
export type PokedexEntryResponseData = BaseResponseData<PokedexEntryResponseAttributes>;
export type PokedexEntryResponseAll = BaseResponseAll<PokedexEntryResponseData>;

export interface PokedexEntryResponseAttributes {
    species: string;
    height: number;
    weight: number;
}
