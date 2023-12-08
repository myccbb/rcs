import { center } from "../../db";

import { gen_handler, Context, Success, Error, Response, Code } from "../handler";

let get_piece_handler = gen_handler(get_piece);

const get_piece_schema = {}

async function get_piece(ctx: Context<any>): Promise<Response> {
    const count = await center.piece.count();
    ctx.log.info(`count: ${count}`);
    return Success({count: count});
}

export { get_piece_handler, get_piece_schema };
