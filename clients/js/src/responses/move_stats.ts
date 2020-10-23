import { BaseResponse, BaseResponseData, BaseResponseAll } from './base';
import { TypeNameResponseData } from './type_names';

export type MoveStatsResponse = BaseResponse<MoveStatsResponseAttributes>;
export type MoveStatsResponseData = BaseResponseData<MoveStatsResponseAttributes>;
export type MoveStatsResponseAll = BaseResponseAll<MoveStatsResponseData>;

export interface MoveStatsResponseAttributes {
    effect: number;
    power: number;
    type: TypeNameResponseData;
    accuracy: number;
    pp: number;
}
