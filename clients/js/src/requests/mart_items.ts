import { BaseRequest } from './base';

export type MartItemsRequest = BaseRequest<MartItemsRequestType, MartItemsRequestAttributes>;

enum MartItemsRequestType {
    mart_items = 'mart_items',
}

export interface MartItemsRequestAttributes {
    mart_items: Array<MartItemsRequestAttributesItem>;
}

export interface MartItemsRequestAttributesItem {
    id: string;
    type: MartItemsRequestAttributesItemType | string;
}

export enum MartItemsRequestAttributesItemType {
    item_names = 'item_names',
    tm_names = 'tm_names',
}

export const MartItemsRequestDefault = (attributes: MartItemsRequestAttributes): MartItemsRequest => {
    return {
        data: {
            type: MartItemsRequestType.mart_items,
            attributes,
        },
    };
};
