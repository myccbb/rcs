import { FastifyInstance } from "fastify";
import { create_objtype_handler, create_objtype_schema, createObjtypeReq } from "./create_objtype";
import { get_objtype_handler } from "./get_objtype";
import { list_objtype_handler, list_objtype_schema } from "./list_objtype";



module.exports = function (fastify: FastifyInstance, opts: any, done: any) {
    fastify.get("/objtype/list", list_objtype_schema, list_objtype_handler);
    fastify.get("/objtype", get_objtype_handler);
    fastify.post<{ Body: createObjtypeReq }>("/objtype", create_objtype_schema, create_objtype_handler);

    done()
}

