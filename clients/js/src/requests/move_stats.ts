import { BaseRequest } from './base';

export type MoveStatsRequest = BaseRequest<MoveStatsRequestType, MoveStatsRequestAttributes>;

enum MoveStatsRequestType {
    move_stats = 'move_stats',
}

export interface MoveStatsRequestAttributes {
    effect: number;
    power: number;
    type: MoveStatsRequestAttributesType;
    accuracy: number;
    pp: number;
}

export interface MoveStatsRequestAttributesType {
    id: string;
}

export const MoveStatsRequestDefault = (attributes: MoveStatsRequestAttributes): MoveStatsRequest => {
    return {
        data: {
            type: MoveStatsRequestType.move_stats,
            attributes,
        },
    };
};
