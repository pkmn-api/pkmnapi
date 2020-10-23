import { BaseResponse, BaseResponseData, BaseResponseAll } from './base';
import { MoveNameResponseData } from './move_names';

export type TMMoveResponse = BaseResponse<TMMoveResponseAttributes>;
export type TMMoveResponseData = BaseResponseData<TMMoveResponseAttributes>;
export type TMMoveResponseAll = BaseResponseAll<TMMoveResponseData>;

export interface TMMoveResponseAttributes {
    move: MoveNameResponseData;
}
