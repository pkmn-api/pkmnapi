import { BaseRequest } from './base';

export type TradeRequest = BaseRequest<TradeRequestType, TradeRequestAttributes>;

enum TradeRequestType {
    trades = 'trades',
}

export interface TradeRequestAttributes {
    give: TradeRequestAttributesPokemon;
    get: TradeRequestAttributesPokemon;
    nickname: string;
}

export interface TradeRequestAttributesPokemon {
    id: string;
}

export const TradeRequestDefault = (attributes: TradeRequestAttributes): TradeRequest => {
    return {
        data: {
            type: TradeRequestType.trades,
            attributes,
        },
    };
};
