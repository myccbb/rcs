import { center } from "../../db";

import { gen_handler, Context, Success, Error, Response, Code } from "../handler";

let create_piece_handler = gen_handler(create_piece);


const body_schema = {
    type: "object",
    required: ['title'],
    properties: {
        title: {type: "string"},
    },
}

const create_piece_schema = {
    schema: {
        body: body_schema,
    },
}

interface createPieceReq {
    title: string;
}

async function create_piece(ctx: Context<{ Body: createPieceReq }>): Promise<Response> {
    let body = ctx.req.body
    body.title
    ctx.log.info(`create_piece: ${body.title}`)
    const count = await center.piece.count();
    ctx.log.info(`count: ${count}`);
    return Success({count: count});
}

export { create_piece_handler, create_piece_schema, createPieceReq };
