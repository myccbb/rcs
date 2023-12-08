package mybase.models

import org.ktorm.schema.*

object Piece: Table<Nothing>("pieces") {
    val id = int("id").primaryKey()
    val code = varchar("code")
    val type = varchar("type")
    val title = varchar("title")
    val content = varchar("content")
    val create_time = datetime("create_time")
    val update_time = datetime("update_time")
}