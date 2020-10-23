import { BaseResponse, BaseResponseData, BaseResponseAll } from './base';
import { MoveNameResponseData } from './move_names';

export type PokemonLearnsetResponse = BaseResponse<PokemonLearnsetResponseAttributes>;
export type PokemonLearnsetResponseData = BaseResponseData<PokemonLearnsetResponseAttributes>;
export type PokemonLearnsetResponseAll = BaseResponseAll<PokemonLearnsetResponseData>;

export interface PokemonLearnsetResponseAttributes {
    learnset: Array<PokemonLearnsetResponseAttributesLearnset>;
}

export interface PokemonLearnsetResponseAttributesLearnset {
    level: number;
    move: MoveNameResponseData;
}
