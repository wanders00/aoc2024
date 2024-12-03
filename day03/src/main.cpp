#include <chrono>
#include <fstream>
#include <iostream>
#include <regex>
#include <string>
#include <vector>

/**
 * @brief Calculates the multiplication of a given 'mul' string.
 *
 * @param mul The 'mul' string. Example: "mul(<x>,<y>)"
 * @return int The product of the multiplication.
 */
int compute_mul_string(const std::string& mul) {
    size_t start = mul.find('(') + 1;
    size_t comma = mul.find(',', start);
    size_t end = mul.find(')', comma);

    int first = std::stoi(mul.substr(start, comma - start));
    int second = std::stoi(mul.substr(comma + 1, end - comma - 1));

    return first * second;
}

/**
 * @brief Finds the amount of matches given the regex and their start indices.
 *
 * @param text The text to process.
 * @param regex The regex to follow.
 * @return vector<pair<string, size_t>> of all matches and their start indices
 */
std::vector<std::pair<std::string, size_t>> find_pattern(std::string text, std::string regex) {
    std::regex pattern(regex);
    std::sregex_iterator currentMatch(text.begin(), text.end(), pattern);
    std::sregex_iterator lastMatch;
    std::vector<std::pair<std::string, size_t>> matches;

    while (currentMatch != lastMatch) {
        matches.push_back({currentMatch->str(), currentMatch->position()});
        currentMatch++;
    }

    return matches;
}

const std::string mul_regex = "mul\\(\\d{1,3},\\d{1,3}\\)";
const std::string do_regex = "do\\(\\)";
const std::string dont_regex = "don\\'t\\(\\)";

int main() {
    auto start = std::chrono::high_resolution_clock::now();

    std::ifstream file("input.txt");
    if (!file.is_open()) {
        std::cerr << "Failed to open the file." << std::endl;
        return 1;
    }

    std::string text((std::istreambuf_iterator<char>(file)), std::istreambuf_iterator<char>());
    file.close();

    auto end = std::chrono::high_resolution_clock::now();
    std::chrono::duration<double> elapsed = end - start;

    std::cout << "Time Elapsed - Loading files: " << elapsed.count() << " seconds" << std::endl;

    // Part 1
    start = std::chrono::high_resolution_clock::now();

    std::vector<std::pair<std::string, size_t>> mul_matches = find_pattern(text, mul_regex);

    int result = 0;
    for (const auto& match : mul_matches) {
        result = result + compute_mul_string(match.first);
    }

    end = std::chrono::high_resolution_clock::now();
    elapsed = end - start;

    std::cout << "Result: " << result << std::endl;
    std::cout << "Time Elapsed - Part 1: " << elapsed.count() << " seconds" << std::endl;

    // Part 2
    start = std::chrono::high_resolution_clock::now();

    std::vector<std::pair<std::string, size_t>> do_matches = find_pattern(text, do_regex);
    std::vector<size_t> do_positions;
    for (const auto& match : do_matches) {
        do_positions.push_back(match.second);
    }

    std::vector<std::pair<std::string, size_t>> dont_matches = find_pattern(text, dont_regex);
    std::vector<size_t> dont_positions;
    for (const auto& match : dont_matches) {
        dont_positions.push_back(match.second);
    }

    std::vector<std::pair<size_t, size_t>> to_be_erased = {};
    size_t pos = 0, dont_index = 0, do_index = 0, dont_cur_pos = 0, do_cur_pos = 0;
    while (dont_cur_pos <= pos) {
        if (dont_index >= dont_positions.size()) {
            break;
        }
        dont_cur_pos = dont_positions[dont_index];
        dont_index++;

        while (do_cur_pos < dont_cur_pos) {
            if (do_index >= do_positions.size()) {
                do_cur_pos = text.size() - 1;
                break;
            }
            do_cur_pos = do_positions[do_index];
            do_index++;
        }

        // Really a hack fix, but couldnt be bothered to do better :)
        if (!to_be_erased.empty() && dont_cur_pos <= to_be_erased.back().second) {
            continue;
        }

        to_be_erased.push_back(std::make_pair(dont_cur_pos, do_cur_pos));
        pos = do_cur_pos;
    }

    std::string text_copy = text;
    for (size_t i = to_be_erased.size(); i-- > 0;) {
        text_copy.erase(to_be_erased[i].first, to_be_erased[i].second - to_be_erased[i].first);
    }

    mul_matches = find_pattern(text_copy, mul_regex);

    result = 0;
    for (const auto& match : mul_matches) {
        result = result + compute_mul_string(match.first);
    }

    end = std::chrono::high_resolution_clock::now();
    elapsed = end - start;

    std::cout << "Result after erasing: " << result << std::endl;
    std::cout << "Time Elapsed - Part 2: " << elapsed.count() << " seconds" << std::endl;

    return 0;
}