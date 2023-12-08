import { center } from "../../db";

import { gen_handler, Context, Success, Error, Response, Code } from "../handler";

let list_objtype_handler = gen_handler(list_objtype);

const list_objtype_schema = {}

async function list_objtype(ctx: Context<any>): Promise<Response> {
    const records = await center.object_type.findMany();
    let results = [];
    for (let key in records) {
        const item = records[key];
        results.push({
            id: item.id,
            category: item.category,
            name: item.name,
            create_time: item.create_time,
            update_time: item.update_time,
        })
    }
    return Success({results: results});
}

export { list_objtype_handler, list_objtype_schema };
