interface Response<T> {
    code: number;
    msg: string;
    data: null | T;
}

interface ListData<T> {
    page: number;
    page_size: number;
    total: number;
    results: null | T[];
}

async function request<T>(
    method: string,
    url: string,
    data: any
): Promise<any> {
    url = "//" + window.location.hostname + ":50001" + url;
    let req: any = {
        method: method,
        cache: "no-cache",
        headers: {
            "Content-Type": "application/json",
        },
    };
    if (data != null) {
        if (method === "GET") {
            let params = new URLSearchParams(data).toString();
            if (params.length > 0) {
                url = url + "?" + params.toString();
            }
        } else {
            req.body = JSON.stringify(data);
        }
    }
    const response = await fetch(url, req);
    if (!response.ok) {
        throw new HttpError(response.status, response.statusText);
    }
    let rawData = await response.json();
    return handleResponse(rawData);
}

function handleResponse<T>(data: Response<T>): any {
    //if (data === null || typeof data !== "object") {
    //    throw new Error("invalid data")
    //}
    if (data.code !== 0) {
        throw new ServiceError(data.code, data.msg);
    }
    return data.data;
}

class ServiceError extends Error {
    constructor(code: number, msg: string) {
        super(msg);
        this.name = "ServiceError";
        this.code = code;
        this.msg = msg;
    }

    code: number;
    msg: string;
}

class HttpError extends Error {
    constructor(http_code: number, msg: string) {
        super(http_code.toString() + ": " + msg);
        this.http_code = http_code;
        this.msg = msg;
    }

    http_code: number;
    msg: string;
}

export type { Response, ListData };

export { HttpError, handleResponse, request };
