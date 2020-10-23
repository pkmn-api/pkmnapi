import { BaseRequest } from './base';

export type MoveNameRequest = BaseRequest<MoveNameRequestType, MoveNameRequestAttributes>;

enum MoveNameRequestType {
    move_names = 'move_names',
}

export interface MoveNameRequestAttributes {
    name: string;
}

export const MoveNameRequestDefault = (attributes: MoveNameRequestAttributes): MoveNameRequest => {
    return {
        data: {
            type: MoveNameRequestType.move_names,
            attributes,
        },
    };
};
