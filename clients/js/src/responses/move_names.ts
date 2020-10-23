import { BaseResponse, BaseResponseData, BaseResponseAll } from './base';

export type MoveNameResponse = BaseResponse<MoveNameResponseAttributes>;
export type MoveNameResponseData = BaseResponseData<MoveNameResponseAttributes>;
export type MoveNameResponseAll = BaseResponseAll<MoveNameResponseData>;

export interface MoveNameResponseAttributes {
    name: string;
}
