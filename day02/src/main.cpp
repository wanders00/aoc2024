#include <chrono>
#include <cmath>
#include <fstream>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>

/**
 * @brief Checks if the given line (report) is safe.
 *
 * @param line The input string line to be checked.
 * @return int Returns 1 if the line is safe, otherwise 0.
 */
int is_safe(std::vector<int> entry) {
    const int MIN_DIFF = 1, MAX_DIFF = 3;
    int increasing = 0, decreasing = 0;
    int safe = 1;
    int number = entry[0], next_number;

    for (int i = 1; i < entry.size(); i++) {
        next_number = entry[i];

        if (next_number > number) {
            increasing = 1;

            if (decreasing == 1) {
                safe = 0;
                break;
            }
        }

        else if (next_number < number) {
            decreasing = 1;

            if (increasing == 1) {
                safe = 0;
                break;
            }
        }

        // Same number == invalid
        else {
            safe = 0;
            break;
        }

        int diff = std::abs(next_number - number);
        if (diff < MIN_DIFF || diff > MAX_DIFF) {
            safe = 0;
            break;
        }

        number = next_number;
    }

    return safe;
}

int is_safe_remove_one(std::vector<int> entry) {
    if (is_safe(entry)) {
        return 1;
    }

    std::vector<int> copy;
    for (size_t i = 0; i < entry.size(); i++) {
        copy = entry;
        copy.erase(copy.begin() + i);

        if (is_safe(copy)) {
            return 1;
        }
    }

    return 0;
}

int main() {
    auto start = std::chrono::high_resolution_clock::now();

    // Load input.txt
    std::ifstream file("input.txt");
    if (!file.is_open()) {
        std::cerr << "Failed to open the file." << std::endl;
        return 1;
    }

    std::string line;
    std::vector<std::vector<int>> data;

    // Read all lines into a vector of vector of int
    while (std::getline(file, line)) {
        std::istringstream iss(line);
        std::vector<int> numbers;
        int number;
        while (iss >> number) {
            numbers.push_back(number);
        }
        data.push_back(numbers);
    }

    file.close();

    auto end = std::chrono::high_resolution_clock::now();
    std::chrono::duration<double> elapsed = end - start;

    std::cout << "Time Elapsed - Loading files: " << elapsed.count() << " seconds" << std::endl;

    // ----- Part 1 -----
    start = std::chrono::high_resolution_clock::now();

    int amount_safe = 0;
    for (const auto entry : data) {
        amount_safe += is_safe(entry);
    }

    end = std::chrono::high_resolution_clock::now();
    elapsed = end - start;

    std::cout << "Amount Safe Reports - Part 1: " << amount_safe << std::endl;
    std::cout << "Time Elapsed - Part 1: " << elapsed.count() << " seconds" << std::endl;

    // ----- Part 2 -----
    start = std::chrono::high_resolution_clock::now();

    amount_safe = 0;
    for (const auto entry : data) {
        amount_safe += is_safe_remove_one(entry);
    }

    end = std::chrono::high_resolution_clock::now();
    elapsed = end - start;
    std::cout << "Amount Safe Reports - Part 2: " << amount_safe << std::endl;
    std::cout << "Time Elapsed - Part 2: " << elapsed.count() << " seconds" << std::endl;

    return 0;
}