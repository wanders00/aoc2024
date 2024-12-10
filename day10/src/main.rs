use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::time::Instant;

#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq)]
struct Position {
    y: isize,
    x: isize,
}

fn main() {
    let start = Instant::now();

    let filename = "src/input.txt";
    let content = match read_file(filename) {
        Ok(input) => input,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

    let duration = start.elapsed();
    println!("Time elapsed - Reading File: {:?}", duration);

    let start = Instant::now();

    let mut result = 0;
    for (y, line) in content.iter().enumerate() {
        for (x, &c) in line.iter().enumerate() {
            if c == '0' {
                result += dfs_part1(
                    &content,
                    Position {
                        y: y as isize,
                        x: x as isize,
                    },
                    '9',
                );
            }
        }
    }

    let duration = start.elapsed();
    println!("Result - Part 1: {}", result);
    println!("Time elapsed - Part 1: {:?}", duration);

    let start = Instant::now();

    let mut result = 0;
    for (y, line) in content.iter().enumerate() {
        for (x, &c) in line.iter().enumerate() {
            if c == '9' {
                result += dfs_part2(
                    &content,
                    Position {
                        y: y as isize,
                        x: x as isize,
                    },
                    '0',
                );
            }
        }
    }

    let duration = start.elapsed();
    println!("Result - Part 2: {}", result);
    println!("Time elapsed - Part 2: {:?}", duration);
}

// Check for visited and look for incrementing characters
fn dfs_part1(grid: &Vec<Vec<char>>, start: Position, goal: char) -> usize {
    let mut result = 0;
    let mut stack = vec![start];
    let mut visited = HashSet::new();
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    while let Some(pos) = stack.pop() {
        if let Some(value) = get_char_at_pos(grid, &pos) {
            if !visited.insert(pos) {
                continue;
            }

            if value == goal {
                result += 1;
                continue;
            }

            for &(dy, dx) in &directions {
                let new_pos = Position {
                    y: pos.y + dy,
                    x: pos.x + dx,
                };
                if let Some(new_value) = get_char_at_pos(grid, &new_pos) {
                    if let (Some(new_digit), Some(current_digit)) =
                        (new_value.to_digit(10), value.to_digit(10))
                    {
                        if new_digit == current_digit + 1 {
                            if !visited.contains(&new_pos) {
                                stack.push(new_pos);
                            }
                        }
                    }
                }
            }
        }
    }

    return result;
}

// Do NOT check visited and look for decrementing characters
fn dfs_part2(grid: &Vec<Vec<char>>, start: Position, goal: char) -> usize {
    let mut result = 0;
    let mut stack = vec![start];
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    while let Some(pos) = stack.pop() {
        if let Some(value) = get_char_at_pos(grid, &pos) {
            if value == goal {
                result += 1;
                continue;
            }

            for &(dy, dx) in &directions {
                let new_pos = Position {
                    y: pos.y + dy,
                    x: pos.x + dx,
                };
                if let Some(new_value) = get_char_at_pos(grid, &new_pos) {
                    if let (Some(new_digit), Some(current_digit)) =
                        (new_value.to_digit(10), value.to_digit(10))
                    {
                        if new_digit == current_digit - 1 {
                            stack.push(new_pos);
                        }
                    }
                }
            }
        }
    }

    return result;
}

fn get_char_at_pos(grid: &Vec<Vec<char>>, pos: &Position) -> Option<char> {
    if pos.y >= 0 && pos.x >= 0 {
        let y = pos.y as usize;
        let x = pos.x as usize;
        if y < grid.len() && x < grid[0].len() {
            return Some(grid[y][x]);
        }
    }
    None
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
