import { BaseRequest } from './base';

export type ItemNameRequest = BaseRequest<ItemNameRequestType, ItemNameRequestAttributes>;

enum ItemNameRequestType {
    item_names = 'item_names',
}

export interface ItemNameRequestAttributes {
    name: string;
}

export const ItemNameRequestDefault = (attributes: ItemNameRequestAttributes): ItemNameRequest => {
    return {
        data: {
            type: ItemNameRequestType.item_names,
            attributes,
        },
    };
};
