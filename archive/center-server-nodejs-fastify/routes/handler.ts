import {FastifyLoggerInstance, FastifyReply, FastifyRequest} from "fastify";

class Context<RouterGeneric> {
    req: FastifyRequest<RouterGeneric>;
    reply: FastifyReply;
    log: FastifyLoggerInstance;

    constructor(req: FastifyRequest<RouterGeneric>, reply: FastifyReply) {
        this.req = req;
        this.reply = reply;
        this.log = req.log;
    }
}

type Handler<RouterGeneric> = (ctx: Context<RouterGeneric>) => Promise<Response>;

function gen_handler<RouterGeneric>(handler: Handler<RouterGeneric>) {
    return async (req: FastifyRequest<RouterGeneric>, reply: FastifyReply) => {
        const ctx = new Context(req, reply);
        const res = await handler(ctx);
        reply.send(res);
    }
}

enum Code {
    Success = 0,
    SystemError = 1,
    DBError = 2,
}

class Response {
    code: Code = Code.Success;
    msg: string = "success";
    data: any = null;

    constructor(code: Code, msg: string, data: any) {
        this.code = code;
        this.msg = msg;
        this.data = data;
    }
}

function Success(data: any) {
    return new Response(Code.Success, "success", data);
}

function Error(code: Code, msg: string, data: any) {
    return new Response(code, msg, data);
}

export {gen_handler, Context, Success, Error, Response, Code};