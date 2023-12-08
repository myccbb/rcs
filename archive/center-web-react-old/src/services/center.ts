

interface Response<T> {
    code: number,
    msg: string,
    data: T,
}

interface ListData<T> {
    page: number,
    page_size: number,
    total: number,
    results: T[],
}


function handle_response<T>(data: Response<T>): any {
    //if (data === null || typeof data !== "object") {
    //    throw new Error("invalid data")
    //}
    if (data.code !== 0) {
        throw new ServiceError(data.code, data.msg)
    }
    return data.data
}

class ServiceError extends Error {
    constructor (code: number, msg: string) {
        super(msg);
        this.name = "ServiceError";
        this.code = code;
        this.msg = msg;
    }
    code: number;
    msg: string
}

class HttpError extends Error {
    constructor(http_code: number, msg: string) {
        super(http_code.toString()+ ': ' + msg)
        this.http_code = http_code
        this.msg = msg
    }
    http_code: number;
    msg: string;
}

export type {
    Response,
    ListData,
}

export {
    HttpError,
    handle_response,
}