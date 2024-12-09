use std::fs::File;
use std::io::{self, Read};
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let filename = "src/input.txt";
    match read_file(filename) {
        Ok(content) => {
            let duration = start.elapsed();
            println!("Time elapsed - Reading File: {:?}", duration);

            // Part 1
            let start = Instant::now();

            println!(
                "Result: {}",
                calculate_checksum(rearrange_vector_part1(process_input(&content)))
            );

            let duration = start.elapsed();
            println!("Time elapsed - Part 1: {:?}", duration);

            // Part 2
            let start = Instant::now();

            println!(
                "Result: {}",
                calculate_checksum(rearrange_vector_part2(process_input(&content)))
            );

            let duration = start.elapsed();
            println!("Time elapsed - Part 2: {:?}", duration);
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}

fn process_input(input: &String) -> Vec<usize> {
    let mut result = Vec::new();
    for (index, c) in input.chars().enumerate() {
        let segment = if index % 2 == 0 {
            index / 2
        } else {
            usize::MAX
        };
        if let Some(digit) = c.to_digit(10) {
            for _ in 0..digit {
                result.push(segment);
            }
        }
    }
    result
}

fn rearrange_vector_part1(mut input: Vec<usize>) -> Vec<usize> {
    let mut start = 0;
    let mut end = input.len() - 1;

    while start < end {
        while start < end && input[start] != usize::MAX {
            start += 1;
        }

        while start < end && input[end] == usize::MAX {
            end -= 1;
        }

        if start < end {
            input.swap(start, end);
            start += 1;
            end -= 1;
        }
    }

    input.retain(|&x| x != usize::MAX);
    input
}

fn rearrange_vector_part2(mut input: Vec<usize>) -> Vec<usize> {
    let mut free_spans = Vec::new();
    let mut start = 0;

    while start < input.len() {
        if input[start] == usize::MAX {
            let mut end = start;
            while end < input.len() && input[end] == usize::MAX {
                end += 1;
            }
            free_spans.push((start, end - start));
            start = end;
        } else {
            start += 1;
        }
    }

    let mut end = input.len() - 1;
    while end > 0 {
        if input[end] != usize::MAX {
            let mut len = 1;
            while end - len > 0 && input[end - len] == input[end] {
                len += 1;
            }

            let mut remove_index = usize::MAX;
            for span in free_spans.iter_mut() {
                if end <= span.0 {
                    break;
                }

                if span.1 >= len {
                    for i in 0..len {
                        input.swap(span.0 + i, end - i);
                    }

                    span.1 -= len;

                    if span.1 == 0 {
                        remove_index = span.0;
                    } else {
                        span.0 += len;
                    }
                    break;
                }
            }

            if remove_index != usize::MAX {
                free_spans.retain(|&(s, _)| s != remove_index);
            }
            end -= len;
        } else {
            end -= 1;
        }
    }

    input
}

fn calculate_checksum(input: Vec<usize>) -> usize {
    let mut checksum = 0;
    for (index, &value) in input.iter().enumerate() {
        if value == usize::MAX {
            continue;
        }
        checksum += index * value;
    }
    checksum
}

fn read_file(filename: &str) -> io::Result<String> {
    let file = File::open(filename);

    let mut file = match file {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to open file: {}", e);
            return Err(e);
        }
    };

    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Ok(_) => Ok(content),
        Err(e) => {
            eprintln!("Failed to read file: {}", e);
            Err(e)
        }
    }
}
