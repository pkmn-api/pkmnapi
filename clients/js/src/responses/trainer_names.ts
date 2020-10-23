import { BaseResponse, BaseResponseData, BaseResponseAll } from './base';

export type TrainerNameResponse = BaseResponse<TrainerNameResponseAttributes>;
export type TrainerNameResponseData = BaseResponseData<TrainerNameResponseAttributes>;
export type TrainerNameResponseAll = BaseResponseAll<TrainerNameResponseData>;

export interface TrainerNameResponseAttributes {
    name: string;
}
