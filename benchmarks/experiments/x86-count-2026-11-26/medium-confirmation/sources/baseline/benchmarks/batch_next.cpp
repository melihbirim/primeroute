#include <primesieve.hpp>
#include <algorithm>
#include <chrono>
#include <cmath>
#include <cstdint>
#include <iostream>
#include <limits>
#include <string>
#include <utility>
#include <vector>

int main(int argc, char** argv)
{
    bool stream = argc > 1 && std::string(argv[1]) == "stream";
    std::vector<uint64_t> numbers;
    uint64_t input;
    while (std::cin >> input) numbers.push_back(input);
    primesieve::set_num_threads(1);
    auto timer = std::chrono::steady_clock::now();
    std::vector<std::pair<uint64_t, size_t>> order;
    for (size_t i = 0; i < numbers.size(); ++i) order.emplace_back(numbers[i], i);
    std::sort(order.begin(), order.end());
    std::vector<uint64_t> unique;
    for (auto item : order)
        if (unique.empty() || unique.back() != item.first) unique.push_back(item.first);
    std::vector<uint64_t> found(unique.size());
    size_t groups = 0;
    for (size_t begin = 0; begin < unique.size();) {
        size_t end = begin + 1;
        // At large starts, restarting is expensive. Allow much larger gaps than
        // our grouping rule, and never impose our materialized-vector span cap.
        uint64_t max_gap = std::max<uint64_t>(1000000,
            4 * static_cast<uint64_t>(std::sqrt(static_cast<long double>(unique[begin]))));
        while (end < unique.size() && (stream || unique[end] - unique[end-1] <= max_gap)) ++end;
        ++groups;
        constexpr uint64_t last_prime = 18446744073709551557ull;
        if (unique[begin] < last_prime) {
            auto last = unique[end-1];
            uint64_t hint = last > UINT64_MAX - 512 ? UINT64_MAX : last + 512;
            primesieve::iterator it(unique[begin] + 1, hint);
            uint64_t prime = it.next_prime();
            for (size_t i = begin; i < end; ++i) {
                if (unique[i] >= last_prime) { found[i] = 0; continue; }
                while (prime <= unique[i]) prime = it.next_prime();
                found[i] = prime;
            }
        }
        begin = end;
    }
    std::vector<uint64_t> answers(numbers.size());
    size_t cursor = 0;
    for (auto item : order) {
        while (unique[cursor] != item.first) ++cursor;
        answers[item.second] = found[cursor];
    }
    auto elapsed = std::chrono::duration_cast<std::chrono::nanoseconds>(
        std::chrono::steady_clock::now() - timer).count();
    std::cout << "{\"elapsed_ns\":" << elapsed << ",\"threads\":" << primesieve::get_num_threads()
              << ",\"groups\":" << groups << ",\"unique_queries\":" << unique.size() << ",\"answers\":[";
    for (size_t i = 0; i < answers.size(); ++i) {
        if (i) std::cout << ',';
        if (answers[i]) std::cout << answers[i]; else std::cout << "null";
    }
    std::cout << "]}\n";
}
