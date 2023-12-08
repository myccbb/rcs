package mybase.routes

import io.ktor.server.application.*
import io.ktor.server.response.*
import io.ktor.server.routing.*

fun Route.createCollection() {
    post("/collections") {
        call.respondText("success")
    }
}
