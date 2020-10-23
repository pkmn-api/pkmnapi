import { BaseResponse, BaseResponseData, BaseResponseAll } from './base';

export type ItemNameResponse = BaseResponse<ItemNameResponseAttributes>;
export type ItemNameResponseData = BaseResponseData<ItemNameResponseAttributes>;
export type ItemNameResponseAll = BaseResponseAll<ItemNameResponseData>;

export interface ItemNameResponseAttributes {
    name: string;
}
