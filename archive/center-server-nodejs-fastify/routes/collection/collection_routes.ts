import { FastifyInstance } from "fastify";
import { create_collection_handler, create_collection_schema, createCollectionReq } from "./create_collection";
import { get_collection_handler } from "./get_collection";


module.exports = function (fastify: FastifyInstance, opts: any, done: any) {
    fastify.get("/collection", get_collection_handler);
    fastify.post<{Body: createCollectionReq}>("/collection", create_collection_schema, create_collection_handler);

    done()
}

