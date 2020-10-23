import { BaseResponse, BaseResponseData, BaseResponseAll } from './base';

export type HMNameResponse = BaseResponse<HMNameResponseAttributes>;
export type HMNameResponseData = BaseResponseData<HMNameResponseAttributes>;
export type HMNameResponseAll = BaseResponseAll<HMNameResponseData>;

export interface HMNameResponseAttributes {
    name: string;
}
