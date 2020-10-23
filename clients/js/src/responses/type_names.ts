import { BaseResponse, BaseResponseData, BaseResponseAll } from './base';

export type TypeNameResponse = BaseResponse<TypeNameResponseAttributes>;
export type TypeNameResponseData = BaseResponseData<TypeNameResponseAttributes>;
export type TypeNameResponseAll = BaseResponseAll<TypeNameResponseData>;

export interface TypeNameResponseAttributes {
    name: string;
}
