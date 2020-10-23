import { BaseRequest } from './base';

export type HMMoveRequest = BaseRequest<HMMoveRequestType, HMMoveRequestAttributes>;

enum HMMoveRequestType {
    hm_moves = 'hm_moves',
}

export interface HMMoveRequestAttributes {
    move: HMMoveRequestAttributesMove;
}

export interface HMMoveRequestAttributesMove {
    id: string;
}

export const HMMoveRequestDefault = (attributes: HMMoveRequestAttributes): HMMoveRequest => {
    return {
        data: {
            type: HMMoveRequestType.hm_moves,
            attributes,
        },
    };
};
