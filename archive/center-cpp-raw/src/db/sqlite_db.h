//
// Created by liuyang on 2022/3/26.
//

#include <string>
#include <utility>
#include <vector>
#include <variant>
#include <cstdint>
#include <memory>

#include <sqlite3.h>

#include "spdlog/spdlog.h"

#ifndef CENTER_DB_CONNECTION_H
#define CENTER_DB_CONNECTION_H

namespace logger = spdlog;

using std::vector;
using std::tuple, std::make_tuple, std::tie;
using std::shared_ptr, std::unique_ptr;
using sql_param_t = std::variant<int64_t, double, std::string>;
using sql_value_t = sql_param_t;

using row_t = vector<sql_value_t>;
using rows_t = vector<row_t>;

enum DBCode {
    Success,
    Done,
    Row,
    Error,
    NotConnected,
    NotFound,
    Exist,
    RowLengthNotMatch,
    ColumnTypeNotSupport,
};

enum ColumnType {
    Integer,
    Real,
    Text,
};

DBCode sqlite_2_db_code(int code);

DBCode bind_sqlite_stmt(sqlite3_stmt *stmt, std::vector<sql_param_t> &value_list);

tuple<row_t, DBCode> get_row(sqlite3_stmt *stmt, vector<ColumnType> &column_type_list);

class SqliteExecutor {
private:
    sqlite3 *conn;

public:
    SqliteExecutor() : conn{nullptr} {}

    explicit SqliteExecutor(sqlite3 *conn) : conn{conn} {}

    virtual ~SqliteExecutor() {};

    sqlite3 *get_conn() const {
        return conn;
    }

    void set_conn(sqlite3 *conn) {
        SqliteExecutor::conn = conn;
    }

    DBCode execute(std::string &sql, std::vector<sql_param_t> &value_list);

    tuple<rows_t *, DBCode>
    query_row(
            std::string &sql,
            vector<sql_param_t> &value_list,
            vector<ColumnType> &column_type_list
    );

    tuple<rows_t *, DBCode>
    query_rows(
            std::string &sql,
            vector<sql_param_t> &value_list,
            vector<ColumnType> &column_type_list
    );
};

class SqliteTransactionExecutor : public SqliteExecutor {

};

class SqliteTransaction {
private:
public:
    explicit SqliteTransaction() {}

    ~SqliteTransaction() {
    }

    DBCode begin() { // TODO
        return Success;
    }

    DBCode commit() { // TODO
        return Success;
    }

    DBCode rollback() { // TODO
        return Success;
    }
};

class SqliteDB : private SqliteExecutor {
private:
    const char *db_path;
    bool connected;

public:
    explicit SqliteDB(const char *db_path) :
            db_path{db_path}, connected{false} {}

    SqliteDB(const SqliteDB &conn) :
            db_path{conn.db_path}, connected{false} {}

    ~SqliteDB() {
    }

    DBCode connect();

    DBCode disconnect();

    // TODO 实现事务
    std::tuple<SqliteTransaction, DBCode> begin_statement() {
        sqlite3_stmt *new_stmt{nullptr};
        // int code = sqlite3_prepare_v2(conn,)
        return make_tuple(SqliteTransaction{}, Success);
    }
};


DBCode SqliteExecutor::execute(std::string &sql, std::vector<sql_param_t> &value_list) {
    if (this->conn == nullptr) {
        return NotConnected;
    }
    sqlite3_stmt *stmt{nullptr};
    DBCode code{};
    code = sqlite_2_db_code(sqlite3_prepare_v2(
            this->conn,
            sql.c_str(),
            -1,
            &stmt,
            nullptr));
    if (code != Success) {
        return Error;
    }
    auto finalizer = [](sqlite3_stmt *stmt) {
        logger::info("release stmt");
        sqlite3_finalize(stmt);
    };
    auto finalizer_guard = unique_ptr<sqlite3_stmt, decltype(finalizer)>(stmt, finalizer);
    code = bind_sqlite_stmt(stmt, value_list);
    if (code != Success) {
        return Error;
    }
    code = sqlite_2_db_code(sqlite3_step(stmt));
    if (code != Success) {
        return Error;
    }
    return Success;
}

tuple<rows_t *, DBCode> SqliteExecutor::query_row(
        std::string &sql,
        std::vector<sql_param_t> &value_list,
        vector<ColumnType> &column_type_list
) {
    if (this->conn == nullptr) {
        return make_tuple(nullptr, NotConnected);
    }
    sqlite3_stmt *stmt{nullptr};
    DBCode code{};
    code = sqlite_2_db_code(sqlite3_prepare_v2(
            this->conn,
            sql.c_str(),
            -1,
            &stmt,
            nullptr
    ));
    if (code != Success) {
        return make_tuple(nullptr, Error);
    }
    auto finalizer = [](sqlite3_stmt *stmt) {
        logger::info("release stmt");
        sqlite3_finalize(stmt);
    };
    auto finalizer_guard = unique_ptr<sqlite3_stmt, decltype(finalizer)>(stmt, finalizer);
    code = bind_sqlite_stmt(stmt, value_list);
    if (code != Success) {
        return make_tuple(nullptr, Error);
    }
    code = sqlite_2_db_code(sqlite3_step(stmt));
    switch (code) {
        case Done:
            return make_tuple(nullptr, Success);
            // break;
        case Row:
            break;
        default:
            return make_tuple(nullptr, Error);
    }
    row_t row = row_t{};
    tie(row, code) = get_row(stmt, column_type_list);
    return make_tuple(nullptr, Success);
}

tuple<rows_t *, DBCode> SqliteExecutor::query_rows(
        std::string &sql,
        std::vector<sql_param_t> &value_list,
        vector<ColumnType> &column_type_list
) {
    if (this->conn == nullptr) {
        return make_tuple(nullptr, NotConnected);
    }
    sqlite3_stmt *stmt{nullptr};
    DBCode code{};
    code = sqlite_2_db_code(sqlite3_prepare_v2(
            this->conn,
            sql.c_str(),
            -1,
            &stmt,
            nullptr));
    if (code != Success) {
        return make_tuple(nullptr, Error);
    }
    auto finalizer = [](sqlite3_stmt *stmt) {
        logger::info("release stmt");
        sqlite3_finalize(stmt);
    };
    auto finalizer_guard = unique_ptr<sqlite3_stmt, decltype(finalizer)>(stmt, finalizer);
    code = bind_sqlite_stmt(stmt, value_list);
    if (code != Success) {
        return make_tuple(nullptr, Error);
    }
    code = sqlite_2_db_code(sqlite3_step(stmt));
    if (code != Success) {
        return make_tuple(nullptr, Error);
    }
    return make_tuple(nullptr, Success);
}

DBCode SqliteDB::connect() {
    if (!connected) {
        sqlite3 *conn{nullptr};
        int code = sqlite_2_db_code(sqlite3_open(db_path, &conn));
        if (code != Success) {
            return Error;
        }
        connected = true;
        SqliteExecutor::set_conn(conn);
        return Success;
    }
    return Success;
}

DBCode SqliteDB::disconnect() {
    if (connected) {
        sqlite3_close(SqliteExecutor::get_conn());
        SqliteExecutor::set_conn(nullptr);
        connected = false;
        return Success;
    }
    return Success;
}

tuple<row_t, DBCode> get_row(sqlite3_stmt *stmt, vector<ColumnType> &column_type_list) {
    if (column_type_list.empty() || column_type_list.size() != sqlite3_column_count(stmt)) {
        logger::info("column_type_list {}, sqlite column count {}",
                     column_type_list.size(),
                     sqlite3_column_count(stmt));
        return make_tuple(row_t{}, RowLengthNotMatch);
    }
    int index{};
    row_t row = row_t{column_type_list.size()};
    for (auto &type: column_type_list) {
        switch (type) {
            case ColumnType::Integer: {
                row[index] = sqlite3_column_int64(stmt, index);
                break;
            }
            case ColumnType::Text: {
                const unsigned char *str = sqlite3_column_text(stmt, index);
                row[index] = std::string{reinterpret_cast<const char *>(str)};
                break;
            }
            case ColumnType::Real: {
                row[index] = sqlite3_column_double(stmt, index);
                break;
            }
            default: {
                logger::info("column type {} not support", type);
                return make_tuple(row_t{}, ColumnTypeNotSupport);
                // break;
            }
        }
        index += 1;
    }
    return make_tuple(row, Success);
}

DBCode sqlite_2_db_code(int code) {
    switch (code) {
        case SQLITE_OK:
            return Success;
        case SQLITE_ROW:
            return Row;
        case SQLITE_DONE:
            return Done;
        case SQLITE_ERROR:
            return Error;
        case SQLITE_NOTFOUND:
            return NotFound;
        case SQLITE_CONSTRAINT:
            return Exist;
        default:
            return Error;
    }
}

DBCode bind_sqlite_stmt(sqlite3_stmt *stmt, std::vector<sql_param_t> &value_list) {
    DBCode code{};
    int8_t index = 1;
    for (auto &value: value_list) {
        switch (value.index()) {
            case 0:
                code = sqlite_2_db_code(sqlite3_bind_int64(stmt, index, std::get<int64_t>(value)));
                if (code != Success) {
                    return code;
                }
                break;
            case 1:
                code = sqlite_2_db_code(sqlite3_bind_double(stmt, index, std::get<double>(value)));
                if (code != Success) {
                    return code;
                }
            case 2:
                code = sqlite_2_db_code(sqlite3_bind_text(
                        stmt, index, std::get<std::string>(value).c_str(), -1, SQLITE_STATIC
                ));
                if (code != Success) {
                    return code;
                }
        }
        index += 1;
    }
    return Success;
}


#endif //CENTER_DB_CONNECTION_H
