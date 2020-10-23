import { BaseResponse, BaseResponseData, BaseResponseAll } from './base';

export type TMNameResponse = BaseResponse<TMNameResponseAttributes>;
export type TMNameResponseData = BaseResponseData<TMNameResponseAttributes>;
export type TMNameResponseAll = BaseResponseAll<TMNameResponseData>;

export interface TMNameResponseAttributes {
    name: string;
}
