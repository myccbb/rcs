#include <iostream>

// #include "fmt/core.h"
#include "spdlog/spdlog.h"
#include "workflow/WFHttpServer.h"

int main() {
    fmt::print("{}\n", "hello");
    spdlog::info("hello\n");
    WFHttpServer server([](WFHttpTask *task) {
        auto uri = task->get_req()->get_request_uri();
        spdlog::info("{}\n", uri);
        task->get_resp()->append_output_body("fack you");
    });
    if (server.start(50001) == 0) {
        char c;
        std::cin >> c;
        server.stop();
    }
}