#include <filesystem>
#include "spdlog/spdlog.h"

#include <boost/beast/core.hpp>

#include <sqlite3.h>
#include <string>
#include <nlohmann/json.hpp>
#include <unordered_map>

namespace logger = spdlog;
namespace fs = std::filesystem;
using json = nlohmann::json;

class data {
public:
    int32_t id;
    std::string name;
    std::string value;
};

using CollectionType = std::string;

const CollectionType CollectionTypeArticle{"article"};
const CollectionType CollectionTypeToc{"toc"};


using PieceType = std::string;

const PieceType PieceTypeTodo{"todo"};
const PieceType PieceTypeNote{"note"};

// enum CollectionType {
//     CollectionTypeArticle,
//     CollectionTypeToc,
// };

class CollectionTypeHelper {
public:
    static CollectionType getCollectionType(std::string_view type) {
        if (type == "article") {
            return CollectionTypeArticle;
        } else if (type == "toc") {
            return CollectionTypeToc;
        } else {
            throw std::runtime_error("unknown collection type");
        }
    }
};


struct Piece {
    PieceType type;
    std::string content;

    // convert json to Piece
    static Piece fromJson(const json& j) {
        Piece p;
        p.type = j["type"].get<std::string>();
        p.content = j["content"].get<std::string>();
        return p;
    }

    // to json
    json to_json() const {
        json j;
        j["type"] = type;
        j["content"] = content;
        return j;
    }
};


struct Collection {
    CollectionType type;
    Collection *collection;
    std::vector<Piece> pieces;

    Collection() : collection(nullptr), pieces{} {
        spdlog::info("construct Collection()");
    }
    Collection(int id): collection(nullptr), pieces{} {
        spdlog::info("construct Collection(int id)");
    }

    // from json
    static Collection from_json(const json &j) {
        Collection c;
        c.type = j["type"].get<CollectionType>();

        // iter over pieces
        for (const auto& p : j["pieces"]) {
            c.pieces.push_back(Piece::fromJson(p));
        }
        return c;
    }

    // to json
    json to_json() const {
        json j;
        j["type"] = type;
        if (collection) {
            j["collection"] = collection->to_json();
        }
        if (!pieces.empty()) {
            j["pieces"] = json::array();
            for (const auto &piece : pieces) {
                j["pieces"].push_back(piece.to_json());
            }
        }
        return j;
    }
};

class C {
public:
    C() {
        spdlog::info("construct C()");
    }
    ~C() {
        spdlog::info("destruct C()");
    }
};

int main() {
    logger::info("creating pc");
    C *pc = new C{};
    delete pc;
    logger::info("delete pc");
    logger::info("------------------");

    logger::info("construct collection begin");
    Collection c = Collection{};
    logger::info("construct collection over");
    c.type = CollectionTypeArticle;
    c.pieces = {
        {.type = PieceTypeNote, .content = "note"},
        {.type = PieceTypeTodo, .content = "todo"},
    };

    // convert c to json string
    std::string json_str = c.to_json().dump();
    spdlog::info("json_str: {}", json_str);

    Collection c1 = Collection{};
    std::string json_str1 = R"({"pieces":[{"content":"note111","type":"note"},{"content":"todo111","type":"todo"}],"type":"article"})";
    // from json_str1 to c1
    c1 = Collection::from_json(json::parse(json_str1));
    // convert c1 to json string
    std::string json_str2 = c1.to_json().dump();
    spdlog::info("json_str: {}", json_str2);

    spdlog::info("hello");
    sqlite3 *db = nullptr;
    int code{0};
    code = sqlite3_open("center.db3", &db);
    if (code != SQLITE_OK) {
        spdlog::info("failed to open sqlite db {}", code);
        return code;
    }
    spdlog::info("open db success");

    // drop table
    std::string_view drop_table_sql{"drop table if exists data;"};
    code = sqlite3_exec(db, drop_table_sql.data(), nullptr, nullptr, nullptr);
    if (code != SQLITE_OK) {
        spdlog::info("failed to drop table {}", code);
        return code;
    }

    // create table
    std::string_view create_table_sql{
            "create table if not exists data(id integer primary key autoincrement, name text, value text);"};
    code = sqlite3_exec(db, create_table_sql.data(), nullptr, nullptr, nullptr);
    if (code != SQLITE_OK) {
        spdlog::info("failed to create table {}", code);
        return code;
    }

    // prepare insert
    sqlite3_stmt *insert_data_stmt{nullptr};
    code = sqlite3_prepare_v2(db, "insert into data(name, value) values(?,?);", -1, &insert_data_stmt, nullptr);
    if (code != SQLITE_OK) {
        spdlog::info("failed to prepare insert {}", code);
        return code;
    }
    // bind insert data
    code = sqlite3_bind_text(insert_data_stmt, 1, "hello", -1, SQLITE_STATIC);
    if (code != SQLITE_OK) {
        spdlog::info("failed to bind bind insert column name {}", code);
        return code;
    }
    code = sqlite3_bind_text(insert_data_stmt, 2, "world", -1, SQLITE_STATIC);
    if (code != SQLITE_OK) {
        spdlog::info("failed to bind bind insert column value {}", code);
        return code;
    }
    // execute insert
    code = sqlite3_step(insert_data_stmt);
    if (code != SQLITE_DONE) {
        spdlog::info("failed to execute insert {}", code);
        return code;
    }
    // finalize insert
    code = sqlite3_finalize(insert_data_stmt);
    if (code != SQLITE_OK) {
        spdlog::info("failed to finalize insert {}", code);
        return code;
    }

    // query data
    sqlite3_stmt *query_data_stmt{nullptr};
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

    sqlite3_close(db);
    spdlog::info("{}", fs::current_path().string());
    return 0;
}