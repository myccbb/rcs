import { Center, center } from "../../db";

import { gen_handler, Context, Success, Error, Response, Code } from "../handler";

import { objtype_id } from "../../utils/id";

import { DB_TIME_FORMAT, now_china } from "../../utils/time";
import { Category } from "../../types/object_type";

let create_objtype_handler = gen_handler(create_objtype);


const body_schema = {
    type: "object",
    required: ['name', 'category'],
    properties: {
        name: { type: "string" },
        category: { type: "string", enum: [Category.Piece, Category.Collection] },
    },
}

const create_objtype_schema = {
    schema: {
        body: body_schema,
    },
}

interface createObjtypeReq {
    name: string;
    category: string;
}

async function create_objtype(ctx: Context<{ Body: createObjtypeReq }>): Promise<Response> {
    let body = ctx.req.body;
    const now = now_china();
    let record: Center.object_typeCreateInput = {
        id: objtype_id(),
        name: body.name,
        category: body.category,
        create_time: now.local().toDate(),
        update_time: now.toDate(),
    }
    //const objtype = await center.object_type.create({
    //    data: record,
    //})
    return Success(record);
}

export { create_objtype_handler, create_objtype_schema, createObjtypeReq };
