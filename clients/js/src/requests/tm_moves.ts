import { BaseRequest } from './base';

export type TMMoveRequest = BaseRequest<TMMoveRequestType, TMMoveRequestAttributes>;

enum TMMoveRequestType {
    tm_moves = 'tm_moves',
}

export interface TMMoveRequestAttributes {
    move: TMMoveRequestAttributesMove;
}

export interface TMMoveRequestAttributesMove {
    id: string;
}

export const TMMoveRequestDefault = (attributes: TMMoveRequestAttributes): TMMoveRequest => {
    return {
        data: {
            type: TMMoveRequestType.tm_moves,
            attributes,
        },
    };
};
