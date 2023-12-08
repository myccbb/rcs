cmake_minimum_required(VERSION 3.16.0)

project(center VERSION 0.1.0)

set(CMAKE_CXX_STANDARD 20)

enable_testing()

add_subdirectory(libs/Catch2)
add_subdirectory(libs/fmt)
add_subdirectory(libs/json)
add_subdirectory(libs/spdlog)

find_package(workflow REQUIRED)

include(CTest)

add_executable(${PROJECT_NAME} src/center.cpp)
target_link_libraries(${PROJECT_NAME} fmt)
target_link_libraries(${PROJECT_NAME} spdlog)
target_link_libraries(${PROJECT_NAME} ssl)
target_link_libraries(${PROJECT_NAME} crypto)
target_link_libraries(${PROJECT_NAME} workflow)
