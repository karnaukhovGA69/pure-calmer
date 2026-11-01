#include <algorithm>
#include <iostream>
#include <optional>
#include <string>
#include <utility>
#include <vector>

// Pure Calmer / C++ — specially for arsendulaev.
namespace calmer {

enum class Status { pending, running, finished };

struct Job {
    std::string name;
    double cpu;
    Status status = Status::pending;

    [[nodiscard]] bool fits(double capacity) const noexcept {
        return cpu <= capacity;
    }
};

template <typename T>
class Queue {
public:
    void push(T value) { items_.push_back(std::move(value)); }

    [[nodiscard]] std::optional<T> pop() {
        if (items_.empty()) return std::nullopt;
        auto value = std::move(items_.back());
        items_.pop_back();
        return value;
    }

private:
    std::vector<T> items_;
};

} // namespace calmer

int main() {
    using namespace calmer;
    constexpr double capacity = 100.0;
    const std::string greeting = "Hello, Arsen!\n";
    std::vector<Job> jobs{
        {"night batch", 18.5},
        {"forecast", 42.0},
        {"packing", 73.5},
    };

    std::sort(jobs.begin(), jobs.end(), [](const Job& a, const Job& b) {
        return a.cpu > b.cpu;
    });

    Queue<Job> queue;
    for (auto& job : jobs) {
        if (!job.fits(capacity)) continue;
        job.status = Status::running;
        queue.push(job);
    }

    std::cout << greeting;
    while (auto job = queue.pop()) {
        const auto& [name, cpu, status] = *job;
        std::cout << name << ": " << cpu << " CPU\n";
    }
    return 0;
}
