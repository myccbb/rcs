package mybase.routes

import io.ktor.server.application.*
import io.ktor.server.request.*
import io.ktor.server.response.*
import io.ktor.server.routing.*
import kotlinx.serialization.*

@Serializable
data class Piece(
    val type: String,
    val title: String = "",
    val content: String = "",
)

fun Route.createPiece() {
    post("/pieces") {
        val req: Piece
        try {
            req = call.receive<Piece>()
        } catch (e: SerializationException) {
            call.respond(e.toString())
            return@post
        }
        call.respond(req)
    }
}
