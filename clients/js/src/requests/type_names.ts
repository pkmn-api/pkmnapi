import { BaseRequest } from './base';

export type TypeNameRequest = BaseRequest<TypeNameRequestType, TypeNameRequestAttributes>;

enum TypeNameRequestType {
    type_names = 'type_names',
}

export interface TypeNameRequestAttributes {
    name: string;
}

export const TypeNameRequestDefault = (attributes: TypeNameRequestAttributes): TypeNameRequest => {
    return {
        data: {
            type: TypeNameRequestType.type_names,
            attributes,
        },
    };
};
