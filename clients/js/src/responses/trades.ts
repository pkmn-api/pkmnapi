import { BaseResponse, BaseResponseData, BaseResponseAll } from './base';
import { PokemonNameResponseData } from './pokemon_names';

export type TradeResponse = BaseResponse<TradeResponseAttributes>;
export type TradeResponseData = BaseResponseData<TradeResponseAttributes>;
export type TradeResponseAll = BaseResponseAll<TradeResponseData>;

export interface TradeResponseAttributes {
    give: PokemonNameResponseData;
    get: PokemonNameResponseData;
    nickname: string;
}
