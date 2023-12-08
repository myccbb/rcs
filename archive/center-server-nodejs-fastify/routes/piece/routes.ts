import { FastifyInstance } from "fastify";
import { create_piece_handler, create_piece_schema, createPieceReq } from "./create_piece";
import { get_piece_handler } from "./get_piece";


module.exports = function (fastify: FastifyInstance, opts: any, done: any) {
    fastify.get("/piece", get_piece_handler);
    fastify.post<{Body: createPieceReq}>("/piece", create_piece_schema, create_piece_handler);

    done()
}

