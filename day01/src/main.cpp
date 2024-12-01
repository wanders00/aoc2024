#include <algorithm>
#include <chrono>
#include <cmath>
#include <fstream>
#include <iostream>
#include <sstream>
#include <string>
#include <unordered_map>
#include <vector>

std::vector<int> left_list;
std::vector<int> right_list;

int main() {
    // ----- Part 1 -----
    auto start = std::chrono::high_resolution_clock::now();

    // Load input.txt
    std::ifstream file("input.txt");
    if (!file.is_open()) {
        std::cerr << "Failed to open the file." << std::endl;
        return 1;
    }

    std::string line;
    while (std::getline(file, line)) {
        // Decipher input
        // Example input: 3   4
        // 3 --> left_list
        // 4 --> right_list
        std::stringstream ss(line);
        int left, right;
        ss >> left >> right;
        left_list.push_back(left);
        right_list.push_back(right);
    }

    file.close();

    // Sort left_list and right_list
    std::sort(left_list.begin(), left_list.end());
    std::sort(right_list.begin(), right_list.end());

    // Iterate over left_list (and right_list)
    int distance = 0;
    for (size_t i = 0; i < left_list.size(); i++) {
        int cur_distance = (left_list[i] - right_list[i]);
        distance = distance + std::abs(cur_distance);
    }

    auto end = std::chrono::high_resolution_clock::now();
    std::chrono::duration<double> elapsed = end - start;

    std::cout << "Total Distance: " << distance << std::endl;
    std::cout << "Time Elapsed (part 1): " << elapsed.count() << " seconds" << std::endl;

    // ----- Part 2 -----
    start = std::chrono::high_resolution_clock::now();

    // Index right_list
    std::unordered_map<int, int> occurrence_map;
    for (size_t i = 0; i < right_list.size(); i++) {
        int key = right_list[i];
        if (occurrence_map.find(key) != occurrence_map.end()) {
            int value = occurrence_map[key];
            occurrence_map[key] = value + 1;
        } else {
            occurrence_map[key] = 1;
        }
    }

    // Find similarity
    int similarity = 0;
    for (size_t i = 0; i < left_list.size(); i++) {
        int key = left_list[i];
        auto it = occurrence_map.find(key);
        if (it != occurrence_map.end()) {
            int value = key * it->second;
            similarity = similarity + value;
        }  // Else: do nothing
    }

    end = std::chrono::high_resolution_clock::now();
    elapsed = end - start;

    std::cout << "Total Similarity: " << similarity << std::endl;
    std::cout << "Time Elapsed (part 2): " << elapsed.count() << " seconds" << std::endl;

    return 0;
}
