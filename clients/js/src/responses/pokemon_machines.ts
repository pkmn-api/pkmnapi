import { BaseResponse, BaseResponseData, BaseResponseAll } from './base';
import { HMMoveResponseData } from './hm_moves';
import { TMMoveResponseData } from './tm_moves';

export type PokemonMachinesResponse = BaseResponse<PokemonMachinesResponseAttributes>;
export type PokemonMachinesResponseData = BaseResponseData<PokemonMachinesResponseAttributes>;
export type PokemonMachinesResponseAll = BaseResponseAll<PokemonMachinesResponseData>;

export interface PokemonMachinesResponseAttributes {
    machines: Array<TMMoveResponseData | HMMoveResponseData>;
}
