package mybase

import io.ktor.server.application.*
import io.ktor.server.engine.*
import io.ktor.server.netty.*
import io.ktor.server.resources.*
import io.ktor.server.plugins.contentnegotiation.*
import io.ktor.serialization.kotlinx.json.*
import kotlinx.serialization.json.*
import io.ktor.server.plugins.callloging.*
import io.ktor.server.request.*

import mybase.routes.*
import org.slf4j.event.Level

fun main() {
    embeddedServer(Netty, port = 8080, host = "0.0.0.0", watchPaths = listOf("classes")) {
        install(Resources)
        install(CallLogging) {
            level = Level.INFO
            //filter { call ->
            //    call.request.path().startsWith("")
            //}
            format { call ->
                val status = call.response.status()
                val httpMethod = call.request.httpMethod.value
                val userAgent = call.request.headers["User-Agent"]
                "Status: $status, HTTP method: $httpMethod, User agent: $userAgent"
            }
        }
        install(ContentNegotiation) {
            json(Json {
                //prettyPrint = true
                //isLenient = true
            })
        }
        configureRouting()
    }.start(wait = true)
}
