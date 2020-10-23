import { BaseResponse, BaseResponseData, BaseResponseAll } from './base';
import { ItemNameResponseData } from './item_names';
import { TMNameResponseData } from './tm_names';

export type MartItemsResponse = BaseResponse<MartItemsResponseAttributes>;
export type MartItemsResponseData = BaseResponseData<MartItemsResponseAttributes>;
export type MartItemsResponseAll = BaseResponseAll<MartItemsResponseData>;

export interface MartItemsResponseAttributes {
    mart_items: Array<ItemNameResponseData | TMNameResponseData>;
}
