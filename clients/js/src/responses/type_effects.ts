import { BaseResponse, BaseResponseData, BaseResponseAll } from './base';
import { TypeNameResponseData } from './type_names';

export type TypeEffectResponse = BaseResponse<TypeEffectResponseAttributes>;
export type TypeEffectResponseData = BaseResponseData<TypeEffectResponseAttributes>;
export type TypeEffectResponseAll = BaseResponseAll<TypeEffectResponseData>;

export interface TypeEffectResponseAttributes {
    attacking_type: TypeNameResponseData;
    defending_type: TypeNameResponseData;
    multiplier: number;
}
