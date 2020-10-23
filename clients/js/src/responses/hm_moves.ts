import { BaseResponse, BaseResponseData, BaseResponseAll } from './base';

import { MoveNameResponseData } from './move_names';

export type HMMoveResponse = BaseResponse<HMMoveResponseAttributes>;
export type HMMoveResponseData = BaseResponseData<HMMoveResponseAttributes>;
export type HMMoveResponseAll = BaseResponseAll<HMMoveResponseData>;

export interface HMMoveResponseAttributes {
    move: MoveNameResponseData;
}
