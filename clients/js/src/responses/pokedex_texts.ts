import { BaseResponse, BaseResponseData, BaseResponseAll } from './base';

export type PokedexTextResponse = BaseResponse<PokedexTextResponseAttributes>;
export type PokedexTextResponseData = BaseResponseData<PokedexTextResponseAttributes>;
export type PokedexTextResponseAll = BaseResponseAll<PokedexTextResponseData>;

export interface PokedexTextResponseAttributes {
    text: string;
}
