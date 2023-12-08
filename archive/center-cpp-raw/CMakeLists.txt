cmake_minimum_required(VERSION 3.16.0)

project(center VERSION 0.1.0)

set(CMAKE_CXX_STANDARD 20)

enable_testing()

include(CTest)

include(${CMAKE_BINARY_DIR}/conanbuildinfo.cmake)
conan_basic_setup()

add_library(${PROJECT_NAME}_db STATIC src/db/piece_record.cpp src/db/piece_record.h src/db/sqlite_db.cpp src/db/sqlite_db.h)
target_link_libraries(${PROJECT_NAME}_db sqlite3)

add_executable(${PROJECT_NAME} src/center.cpp)
target_link_libraries(${PROJECT_NAME} ${PROJECT_NAME}_db)
target_link_libraries(${PROJECT_NAME} fmt)
target_link_libraries(${PROJECT_NAME} spdlog)
target_link_libraries(${PROJECT_NAME} sqlite3)

add_executable(http_server_stackless src/http_server_stackless.cpp)
