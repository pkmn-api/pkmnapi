import { BaseRequest } from './base';

export type AccessTokenRequest = BaseRequest<AccessTokenRequestType, AccessTokenRequestAttributes>;
export type AccessTokenDeleteRequest = BaseRequest<AccessTokenRequestType, AccessTokenDeleteRequestAttributes>;

enum AccessTokenRequestType {
    access_tokens = 'access_tokens',
}

export interface AccessTokenRequestAttributes {
    email_address: string;
}

export interface AccessTokenDeleteRequestAttributes {
    code: string;
    email_address: string;
}

export const AccessTokenRequestDefault = (attributes: AccessTokenRequestAttributes): AccessTokenRequest => {
    return {
        data: {
            type: AccessTokenRequestType.access_tokens,
            attributes,
        },
    };
};

export const AccessTokenDeleteRequestDefault = (
    attributes: AccessTokenDeleteRequestAttributes,
): AccessTokenDeleteRequest => {
    return {
        data: {
            type: AccessTokenRequestType.access_tokens,
            attributes,
        },
    };
};
