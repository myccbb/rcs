import { center } from "../../db";

import { gen_handler, Context, Success, Error, Response, Code } from "../handler";

let get_objtype_handler = gen_handler(get_objtype);

const get_objtype_schema = {}

async function get_objtype(ctx: Context<any>): Promise<Response> {
    const count = await center.collection.count();
    ctx.log.info(`count: ${count}`);
    return Success({count: count});
}

export { get_objtype_handler, get_objtype_schema };
