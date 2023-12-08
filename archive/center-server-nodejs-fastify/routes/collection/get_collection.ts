import { center } from "../../db";

import { gen_handler, Context, Success, Error, Response, Code } from "../handler";

let get_collection_handler = gen_handler(get_collection);

const get_collection_schema = {}

async function get_collection(ctx: Context<any>): Promise<Response> {
    const count = await center.collection.count();
    ctx.log.info(`count: ${count}`);
    return Success({count: count});
}

export { get_collection_handler, get_collection_schema };
