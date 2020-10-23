import { BaseResponse, BaseResponseData, BaseResponseAll } from './base';
import { MoveNameResponseData } from './move_names';

export type PokemonMovesetResponse = BaseResponse<PokemonMovesetResponseAttributes>;
export type PokemonMovesetResponseData = BaseResponseData<PokemonMovesetResponseAttributes>;
export type PokemonMovesetResponseAll = BaseResponseAll<PokemonMovesetResponseData>;

export interface PokemonMovesetResponseAttributes {
    moveset: Array<PokemonMovesetResponseAttributesMoveset>;
}

export interface PokemonMovesetResponseAttributesMoveset {
    move: MoveNameResponseData;
}
