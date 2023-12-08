//
// Created by liuya on 2022/3/26.
//

#include <string>
#include <vector>

#include "spdlog/spdlog.h"

#include "sqlite_db.h"

#ifndef CENTER_PIECE_RECORD_H
#define CENTER_PIECE_RECORD_H

// pieces table record

class SqlitePieceRecord {
public:
    std::string id;
    std::string type;
    std::string title;
    std::string content;
    std::string create_time;
    std::string update_time;
    SqlitePieceRecord get_by_id(SqliteDB &conn, std::string &id);
};

SqlitePieceRecord SqlitePieceRecord::get_by_id(SqliteDB &conn, std::string &id) {
    SqlitePieceRecord piece_record;
    /*
    // query data
    sqlite3_stmt *query_data_stmt{nullptr};
    int code;
    code = sqlite3_prepare_v2(db, "select id,name from data;", -1, &query_data_stmt, nullptr);
    if (code != SQLITE_OK) {
        spdlog::info("failed to prepare query {}", code);
        return code;
    }
    // execute query
    code = sqlite3_step(query_data_stmt);
    if (code != SQLITE_ROW) {
        spdlog::info("failed to execute query {}", code);
        return code;
    }
    // get data
    data data{};
    data.id = sqlite3_column_int(query_data_stmt, 0);
    data.name = reinterpret_cast<const char *>(sqlite3_column_text(query_data_stmt, 1));
    spdlog::info("data id {} name {}", data.id, data.name);
    // execute query
    code = sqlite3_step(query_data_stmt);
    if (code != SQLITE_ROW) {
        spdlog::info("failed to execute query {}", code);
        return code;
    }
    // finalize query
    code = sqlite3_finalize(query_data_stmt);
    if (code != SQLITE_OK) {
        spdlog::info("failed to finalize query {}", code);
        return code;
    }
     */

    return piece_record;
}

class SqlitePieceRecordList {
public:
    std::vector<SqlitePieceRecord> list;
};


#endif //CENTER_PIECE_RECORD_H
