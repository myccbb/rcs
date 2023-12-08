import { center } from "../../db";

import { gen_handler, Context, Success, Error, Response, Code } from "../handler";

const body_schema = {
    type: "object",
    required: ['id', 'name'],
    properties: {
        id: {type: "string"},
        name: {type: "string"},
        category: {type: "string"},
    },
}

const create_objtype_schema = {
    schema: {
        body: body_schema,
    },
}

interface createObjtypeReq {
    title: string;
}

async function create_objtype(ctx: Context<{ Body: createObjtypeReq }>): Promise<Response> {
    let body = ctx.req.body
    body.title
    ctx.log.info(`create_collection: ${body.title}`)
    const count = await center.object_type.count();
    ctx.log.info(`count: ${count}`);
    return Success({count: count});
}
