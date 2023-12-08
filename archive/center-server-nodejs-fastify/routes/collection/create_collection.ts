import { center } from "../../db";

import { gen_handler, Context, Success, Error, Response, Code } from "../handler";

let create_collection_handler = gen_handler(create_collection);


const body_schema = {
    type: "object",
    required: ['title'],
    properties: {
        title: {type: "string"},
    },
}

const create_collection_schema = {
    schema: {
        body: body_schema,
    },
}

interface createCollectionReq {
    title: string;
}

async function create_collection(ctx: Context<{ Body: createCollectionReq }>): Promise<Response> {
    let body = ctx.req.body
    body.title
    ctx.log.info(`create_collection: ${body.title}`)
    const count = await center.collection.count();
    ctx.log.info(`count: ${count}`);
    return Success({count: count});
}

export { create_collection_handler, create_collection_schema, createCollectionReq };
