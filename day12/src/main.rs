use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::time::Instant;

#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];

fn main() {
    let start = Instant::now();
    let filename = "src/test.txt";
    let content = match read_file(filename) {
        Ok(input) => input,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

    let duration = start.elapsed();
    println!("Time Elapsed - Loading File: {:?}", duration);

    let start = Instant::now();

    let part1_result = solve(&content, recursion_part1);
    let duration = start.elapsed();
    println!("Result - Part 1: {}", part1_result);
    println!("Time Elapsed - Part1: {:?}", duration);

    let start = Instant::now();

    let part2_result = solve(&content, recursion_part2);
    let duration = start.elapsed();
    println!("Result - Part 2: {}", part2_result);
    println!("Time Elapsed - Part2: {:?}", duration);
}

fn solve<F>(grid: &Vec<Vec<char>>, recursion_fn: F) -> usize
where
    F: Fn(char, usize, usize, &mut usize, &Vec<Vec<char>>, &mut HashSet<(usize, usize)>) -> usize,
{
    let mut total_cost = 0;
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    for (y, line) in grid.iter().enumerate() {
        for (x, &c) in line.iter().enumerate() {
            let mut size = 0;
            let cost = recursion_fn(c, y, x, &mut size, grid, &mut visited);
            total_cost += cost * size;
        }
    }
    return total_cost;
}

fn recursion_part1(
    c: char,
    y: usize,
    x: usize,
    size: &mut usize,
    grid: &Vec<Vec<char>>,
    visited: &mut HashSet<(usize, usize)>,
) -> usize {
    if visited.contains(&(y, x)) {
        return 0;
    }

    visited.insert((y, x));

    let mut cost = 0;
    *size += 1;

    for &d in &DIRECTIONS {
        let new_pos = move_position((y as isize, x as isize), d);

        if let Some(next_c) = get_char_at_position(&grid, new_pos) {
            if c == next_c {
                cost += recursion_part1(
                    c,
                    new_pos.0 as usize,
                    new_pos.1 as usize,
                    size,
                    grid,
                    visited,
                );
            } else {
                cost += 1;
            }
        } else {
            cost += 1;
        }
    }

    return cost;
}

fn recursion_part2(
    c: char,
    y: usize,
    x: usize,
    size: &mut usize,
    grid: &Vec<Vec<char>>,
    visited: &mut HashSet<(usize, usize)>,
) -> usize {
    if visited.contains(&(y, x)) {
        return 0;
    }

    visited.insert((y, x));

    let mut cost = 0;
    *size += 1;

    for &d in &DIRECTIONS {
        let new_pos = move_position((y as isize, x as isize), d);

        if let Some(next_c) = get_char_at_position(&grid, new_pos) {
            if c == next_c {
                cost += recursion_part2(
                    c,
                    new_pos.0 as usize,
                    new_pos.1 as usize,
                    size,
                    grid,
                    visited,
                );
            }
        }
    }

    return cost;
}

fn move_position(pos: (isize, isize), dir: Direction) -> (isize, isize) {
    match dir {
        Direction::Up => (pos.0 - 1, pos.1),
        Direction::Down => (pos.0 + 1, pos.1),
        Direction::Left => (pos.0, pos.1 - 1),
        Direction::Right => (pos.0, pos.1 + 1),
    }
}

fn get_char_at_position(input: &Vec<Vec<char>>, pos: (isize, isize)) -> Option<char> {
    if pos.0 >= 0
        && pos.0 < input.len() as isize
        && pos.1 >= 0
        && pos.1 < input[pos.0 as usize].len() as isize
    {
        Some(input[pos.0 as usize][pos.1 as usize])
    } else {
        None
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        let input: Vec<String> = vec![
            "RRRRIICCFF".to_string(),
            "RRRRIICCCF".to_string(),
            "VVRRRCCFFF".to_string(),
            "VVRCCCJFFF".to_string(),
            "VVVVCJJCFE".to_string(),
            "VVIVCCJJEE".to_string(),
            "VVIIICJJEE".to_string(),
            "MIIIIIJJEE".to_string(),
            "MIIISIJEEE".to_string(),
            "MMMISSJEEE".to_string(),
        ];

        let input: Vec<Vec<char>> = input.into_iter().map(|s| s.chars().collect()).collect();
        assert_eq!(solve(&input, recursion_part1), 1930);
    }

    #[test]
    fn test_solve_part2() {
        let input: Vec<String> = vec![
            "RRRRIICCFF".to_string(),
            "RRRRIICCCF".to_string(),
            "VVRRRCCFFF".to_string(),
            "VVRCCCJFFF".to_string(),
            "VVVVCJJCFE".to_string(),
            "VVIVCCJJEE".to_string(),
            "VVIIICJJEE".to_string(),
            "MIIIIIJJEE".to_string(),
            "MIIISIJEEE".to_string(),
            "MMMISSJEEE".to_string(),
        ];

        let input: Vec<Vec<char>> = input.into_iter().map(|s| s.chars().collect()).collect();
        assert_eq!(solve(&input, recursion_part2), 1206);
    }
}
