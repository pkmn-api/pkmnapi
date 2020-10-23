import { BaseResponse, BaseResponseData, BaseResponseAll } from './base';

export type TrainerRewardResponse = BaseResponse<TrainerRewardResponseAttributes>;
export type TrainerRewardResponseData = BaseResponseData<TrainerRewardResponseAttributes>;
export type TrainerRewardResponseAll = BaseResponseAll<TrainerRewardResponseData>;

export interface TrainerRewardResponseAttributes {
    reward: number;
}
