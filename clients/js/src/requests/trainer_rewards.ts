import { BaseRequest } from './base';

export type TrainerRewardRequest = BaseRequest<TrainerRewardRequestType, TrainerRewardRequestAttributes>;

enum TrainerRewardRequestType {
    trainer_rewards = 'trainer_rewards',
}

export interface TrainerRewardRequestAttributes {
    reward: number;
}

export const TrainerRewardRequestDefault = (attributes: TrainerRewardRequestAttributes): TrainerRewardRequest => {
    return {
        data: {
            type: TrainerRewardRequestType.trainer_rewards,
            attributes,
        },
    };
};
