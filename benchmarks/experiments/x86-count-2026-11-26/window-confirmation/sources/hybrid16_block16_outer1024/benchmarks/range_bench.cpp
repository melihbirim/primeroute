#include <primesieve.hpp>
#include <chrono>
#include <cmath>
#include <cstdint>
#include <cstdlib>
#include <iostream>
#include <stdexcept>
#include <string>
#include <utility>
#include <vector>

using Result = std::pair<uint64_t, uint64_t>;

Result query(uint64_t start, uint64_t end, bool generate)
{
    if (!generate)
        return {primesieve::count_primes(start, end), 0};

    auto capacity = static_cast<size_t>((end - start + 1) /
        std::log(static_cast<double>(end < 3 ? 3 : end)) * 1.2 + 64.0);
    std::vector<uint64_t> primes;
    primes.reserve(capacity);
    primesieve::generate_primes(start, end, &primes);
    uint64_t checksum = 0;
    for (auto p : primes)
        checksum += p;
    return {primes.size(), checksum};
}

int main(int argc, char** argv)
{
    if (argc != 6)
        throw std::runtime_error("Usage: range-bench START END count|generate REPEATS SIEVE_KIB(0=default)");
    uint64_t start = std::stoull(argv[1]);
    uint64_t end = std::stoull(argv[2]);
    bool generate = std::string(argv[3]) == "generate";
    unsigned repeats = std::stoul(argv[4]);
    int size = std::stoi(argv[5]);
    if (start > end || end == UINT64_MAX || repeats == 0)
        throw std::runtime_error("Invalid range or repeat count");
    primesieve::set_num_threads(1);
    if (size)
        primesieve::set_sieve_size(size);

    auto expected = query(start, end, generate);
    auto timer = std::chrono::steady_clock::now();
    for (unsigned i = 0; i < repeats; ++i)
        if (query(start, end, generate) != expected)
            throw std::runtime_error("Inconsistent result");
    auto ns = std::chrono::duration_cast<std::chrono::nanoseconds>(
        std::chrono::steady_clock::now() - timer).count();
    std::cout << "{\"elapsed_ns\":" << ns << ",\"repeats\":" << repeats
              << ",\"count\":" << expected.first << ",\"checksum\":" << expected.second
              << ",\"threads\":" << primesieve::get_num_threads() << "}\n";
}
