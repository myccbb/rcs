import Fastify from 'fastify'

import { config } from "./config";

// Require the framework and instantiate it
const fastify: any = Fastify({
    logger: true,
})


/*
// Declare a route
fastify.route({
    method: 'GET',
    url: '/',
    schema: {
        // request needs to have a querystring with a `name` parameter
        querystring: {
            name: {type: 'string'}
        },
        // the response needs to be an object with an `hello` property of type 'string'
        response: {
            200: {
                type: 'object',
                properties: {
                    hello: {type: 'string'}
                }
            }
        }
    },
    // this function is executed for every request before the handler is executed
    preHandler: async (request: any, reply: any) => {
        // E.g. check authentication
    },
    handler: async (request: any, reply: any) => {
        return {hello: 'world'}
    }
})
 */

fastify.register(require('./routes/piece/routes'), {prefix: '/center/api/v1'});
fastify.register(require('./routes/objtype/routes'), {prefix: '/center/api/v1'});

// Run the server!
const start = async () => {
    const http_config = config.http_server
    try {
        await fastify.listen({
            host: http_config.host,
            port: http_config.port,
        })
    } catch (err) {
        fastify.log.error(err)
        process.exit(1)
    }
}

start()
