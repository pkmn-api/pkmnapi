import { BaseRequest } from './base';

export type TypeEffectRequest = BaseRequest<TypeEffectRequestType, TypeEffectRequestAttributes>;

enum TypeEffectRequestType {
    type_effects = 'type_effects',
}

export interface TypeEffectRequestAttributes {
    attacking_type: TypeEffectRequestAttributesType;
    defending_type: TypeEffectRequestAttributesType;
    multiplier: number;
}

export interface TypeEffectRequestAttributesType {
    id: string;
}

export const TypeEffectRequestDefault = (attributes: TypeEffectRequestAttributes): TypeEffectRequest => {
    return {
        data: {
            type: TypeEffectRequestType.type_effects,
            attributes,
        },
    };
};
