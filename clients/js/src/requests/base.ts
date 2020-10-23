export interface BaseRequest<T, U> {
    data: BaseRequestData<T, U>;
}

export interface BaseRequestData<T, U> {
    type: T;
    attributes: U;
}
