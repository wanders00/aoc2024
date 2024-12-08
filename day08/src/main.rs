use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::time::Instant;

#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: usize,
    y: usize,
}

fn main() {
    let start = Instant::now();
    let filename = "src/input.txt";
    let mut char_map: HashMap<char, Vec<Position>> = HashMap::new();

    match read_file(filename) {
        Ok(input) => {
            let mut result = vec![vec!['0'; input[0].len()]; input.len()];
            for y in 0..input.len() {
                let line = &input[y];
                for x in 0..line.len() {
                    let c = line[x];
                    if c == '.' {
                        continue;
                    }
                    char_map
                        .entry(c)
                        .or_insert_with(Vec::new)
                        .push(Position { y, x });
                }
            }

            let duration = start.elapsed();
            println!("Time elapsed - Parsing File: {:?}", duration);

            // Part 1
            let start = Instant::now();
            for value in char_map.values() {
                let len = value.len();
                for i in 0..len {
                    let start = &value[i];
                    for j in 0..len {
                        if i == j {
                            continue;
                        }

                        let end = &value[j];

                        let y_diff: isize = end.y as isize - start.y as isize;
                        let x_diff: isize = end.x as isize - start.x as isize;

                        let y = end.y as isize + y_diff;
                        let x = end.x as isize + x_diff;
                        let y_u = y as usize;
                        let x_u = x as usize;

                        if y >= 0 && x >= 0 && y_u < input.len() && x_u < input[y_u].len() {
                            result[y_u][x_u] = '#';
                        }
                    }
                }
            }

            println!("Result - Part 1: {}", count_char_in_vec(&result, '#'));
            let duration = start.elapsed();
            println!("Time elapsed - Part 1: {:?}", duration);

            let start = Instant::now();
            let mut result = vec![vec!['0'; input[0].len()]; input.len()];

            // Part 2
            for value in char_map.values() {
                let len = value.len();
                for i in 0..len {
                    let start = &value[i];
                    for j in 0..len {
                        if i == j {
                            continue;
                        }

                        let end = &value[j];

                        let y_diff: isize = end.y as isize - start.y as isize;
                        let x_diff: isize = end.x as isize - start.x as isize;

                        let mut y = start.y as isize + y_diff;
                        let mut x = start.x as isize + x_diff;
                        let mut y_u = y as usize;
                        let mut x_u = x as usize;

                        while y >= 0 && x >= 0 && y_u < input.len() && x_u < input[y_u].len() {
                            result[y_u][x_u] = '#';

                            y += y_diff;
                            x += x_diff;
                            y_u = y as usize;
                            x_u = x as usize;
                        }
                    }
                }
            }

            println!("Result - Part 2: {}", count_char_in_vec(&result, '#'));
            let duration = start.elapsed();
            println!("Time elapsed - Part 2: {:?}", duration);
        }
        Err(e) => eprintln!("Failed to read file: {}", e),
    }
}

fn count_char_in_vec(input: &Vec<Vec<char>>, target: char) -> usize {
    input
        .iter()
        .flat_map(|row| row.iter())
        .filter(|&&c| c == target)
        .count()
}

fn read_file(filename: &str) -> io::Result<Vec<Vec<char>>> {
    let file = File::open(filename);

    let file = match file {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to open file: {}", e);
            return Err(e);
        }
    };

    let reader = BufReader::new(file);

    let mut lines: Vec<Vec<char>> = Vec::new();
    for line in reader.lines() {
        match line {
            Ok(line) => lines.push(line.chars().collect()),
            Err(e) => {
                eprintln!("Failed to read line: {}", e);
                return Err(e);
            }
        }
    }

    Ok(lines)
}
