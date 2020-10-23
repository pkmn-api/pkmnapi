import { BaseResponse, BaseResponseData, BaseResponseAll } from './base';

export type PlayerNamesResponse = BaseResponse<PlayerNamesResponseAttributes>;
export type PlayerNamesResponseData = BaseResponseData<PlayerNamesResponseAttributes>;

export interface PlayerNamesResponseAttributes {
    player: Array<string>;
    rival: Array<string>;
}
