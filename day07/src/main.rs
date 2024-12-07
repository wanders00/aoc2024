use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::time::Instant;

fn main() {
    let filename = "src/input.txt";
    let mut result = 0;
    match read_file(filename) {
        Ok(lines) => {
            let start = Instant::now();
            for line in &lines {
                let numbers = string_to_numbers(line);
                result += solve(&numbers, numbers[1], 1, false);
            }
            let duration = start.elapsed();
            println!("Time elapsed - Part 1: {:?}", duration);
            println!("Result - Part 1: {}", result);

            let start = Instant::now();
            result = 0;

            for line in &lines {
                let numbers = string_to_numbers(line);
                result += solve(&numbers, numbers[1], 1, true);
            }
            let duration = start.elapsed();
            println!("Time elapsed - Part 2: {:?}", duration);
            println!("Result - Part 2: {}", result);
        }
        Err(e) => eprintln!("Failed to read file: {}", e),
    }
}

fn solve(input: &Vec<i64>, total: i64, start: usize, part2: bool) -> i64 {
    if input.len() < 2 {
        return 0;
    }

    if start >= input.len() - 1 {
        if total == input[0] {
            return input[0];
        }
        return 0;
    }

    let addition = total + input[start + 1];
    let multiplication = total * input[start + 1];

    if solve(input, addition, start + 1, part2) != 0 {
        return input[0];
    }

    if solve(input, multiplication, start + 1, part2) != 0 {
        return input[0];
    }

    if !part2 {
        return 0;
    }

    let mut total_str = total.to_string();
    total_str.push_str(&input[start + 1].to_string());
    let concat = total_str.parse::<i64>().unwrap();
    if solve(input, concat, start + 1, part2) != 0 {
        return input[0];
    }

    return 0;
}

// "1050908860: 1 6 8 5 93 443 20" -> [1050908860, 1, 6, 8, 5, 93, 443, 20]
fn string_to_numbers(input: &str) -> Vec<i64> {
    let parts: Vec<&str> = input.split(':').collect();
    if parts.len() != 2 {
        eprintln!("Invalid input format");
        return Vec::new();
    }

    let mut numbers = Vec::new();

    // Parse the first number
    if let Ok(first_number) = parts[0].trim().parse() {
        numbers.push(first_number);
    } else {
        eprintln!("Invalid first number format");
        return Vec::new();
    }

    // Parse the rest of the numbers
    let numbers_str = parts[1].trim();
    let rest_numbers: Vec<i64> = numbers_str
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    numbers.extend(rest_numbers);

    numbers
}

fn read_file(filename: &str) -> io::Result<Vec<String>> {
    let file = File::open(filename);

    let file = match file {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to open file: {}", e);
            return Err(e);
        }
    };

    let reader = BufReader::new(file);

    let mut lines: Vec<String> = Vec::new();
    for line in reader.lines() {
        match line {
            Ok(line) => lines.push(line),
            Err(e) => {
                eprintln!("Failed to read line: {}", e);
                return Err(e);
            }
        }
    }

    Ok(lines)
}
