import { BaseResponse, BaseResponseData, BaseResponseAll } from './base';
import { TypeNameResponseData } from './type_names';

export type PokemonStatsResponse = BaseResponse<PokemonStatsResponseAttributes>;
export type PokemonStatsResponseData = BaseResponseData<PokemonStatsResponseAttributes>;
export type PokemonStatsResponseAll = BaseResponseAll<PokemonStatsResponseData>;

export interface PokemonStatsResponseAttributes {
    base_hp: number;
    base_attack: number;
    base_defence: number;
    base_speed: number;
    base_special: number;
    types: Array<TypeNameResponseData>;
    catch_rate: number;
    base_exp_yield: number;
    growth_rate: number;
}
