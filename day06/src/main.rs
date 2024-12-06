use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::time::Instant;

#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq)]
struct Position {
    y: isize,
    x: isize,
}

#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

static mut START_POS: Position = Position { y: -1, x: -1 };

fn run_maze(input: &mut Vec<Vec<char>>, find_loop: bool) -> usize {
    unsafe {
        set_char_at_position(input, START_POS, 'X');
    }
    let mut pos;
    unsafe {
        pos = START_POS;
    }
    let mut dir = Direction::Up;
    let mut next_pos = move_position(pos, dir);
    let mut visited_positions: HashSet<(Position, Direction)> = HashSet::new();

    while pos.y >= 0
        && pos.y < input.len() as isize
        && pos.x >= 0
        && pos.x < input[pos.y as usize].len() as isize
    {
        if let Some(c) = get_char_at_position(&input, next_pos) {
            if c == '#' {
                if find_loop {
                    let tuple = (pos, dir);
                    if visited_positions.contains(&tuple) {
                        return 1;
                    }
                    visited_positions.insert(tuple);
                }
                dir = turn_right(dir);
                next_pos = move_position(pos, dir);
            } else {
                pos = next_pos;
                set_char_at_position(input, pos, 'X');
                next_pos = move_position(pos, dir);
            }
        } else {
            break;
        }
    }

    if find_loop {
        return 0;
    }

    let result = count_char_in_vec(&input, 'X');
    println!("Result: {}", result);
    return result;
}

fn part_2(input: &mut Vec<Vec<char>>) {
    let mut count = 0;
    for y in 0..input.len() {
        for x in 0..input[y].len() {
            let pos = Position {
                y: y as isize,
                x: x as isize,
            };
            if get_char_at_position(input, pos) == Some('X') {
                let mut temp = input.clone();
                set_char_at_position(&mut temp, pos, '#');
                count += run_maze(&mut temp, true);
            }
        }
    }

    println!("Result p2: {}", count);
}

fn main() {
    let filename = "src/input.txt";
    let start = Instant::now();
    match read_file(filename) {
        Ok(mut input) => {
            unsafe {
                START_POS = find_start_position(&input);
            }
            run_maze(&mut input, false);
            let duration = start.elapsed();
            println!("Time elapsed - Part 1: {:?}", duration);

            let start = Instant::now();
            part_2(&mut input);
            let duration = start.elapsed();
            println!("Time elapsed - Part 2: {:?}", duration);
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn count_char_in_vec(input: &Vec<Vec<char>>, target: char) -> usize {
    input
        .iter()
        .flat_map(|row| row.iter())
        .filter(|&&c| c == target)
        .count()
}

fn get_char_at_position(input: &Vec<Vec<char>>, pos: Position) -> Option<char> {
    if pos.y >= 0 && pos.y < input.len() as isize && pos.x >= 0 && pos.x < input[pos.y as usize].len() as isize {
        Some(input[pos.y as usize][pos.x as usize])
    } else {
        None
    }
}

fn set_char_at_position(input: &mut Vec<Vec<char>>, pos: Position, c: char) {
    if pos.y >= 0 && pos.y < input.len() as isize && pos.x >= 0 && pos.x < input[pos.y as usize].len() as isize {
        input[pos.y as usize][pos.x as usize] = c;
    }
}

fn turn_right(dir: Direction) -> Direction {
    match dir {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

fn move_position(pos: Position, dir: Direction) -> Position {
    match dir {
        Direction::Up => Position {
            y: pos.y - 1,
            x: pos.x,
        },
        Direction::Down => Position {
            y: pos.y + 1,
            x: pos.x,
        },
        Direction::Left => Position {
            y: pos.y,
            x: pos.x - 1,
        },
        Direction::Right => Position {
            y: pos.y,
            x: pos.x + 1,
        },
    }
}

fn find_start_position(input: &Vec<Vec<char>>) -> Position {
    for y in 0..input.len() {
        for x in 0..input[y].len() {
            if input[y][x] == '^' {
                return Position {
                    y: y as isize,
                    x: x as isize,
                };
            }
        }
    }

    Position { y: -1, x: -1 }
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
