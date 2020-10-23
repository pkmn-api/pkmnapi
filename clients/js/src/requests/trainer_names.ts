import { BaseRequest } from './base';

export type TrainerNameRequest = BaseRequest<TrainerNameRequestType, TrainerNameRequestAttributes>;

enum TrainerNameRequestType {
    trainer_names = 'trainer_names',
}

export interface TrainerNameRequestAttributes {
    name: string;
}

export const TrainerNameRequestDefault = (attributes: TrainerNameRequestAttributes): TrainerNameRequest => {
    return {
        data: {
            type: TrainerNameRequestType.trainer_names,
            attributes,
        },
    };
};
