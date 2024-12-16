use std::fs::File;
use std::io::Read;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let contents = read_file("input.txt");
    let (grid, moves) = parse_data(&contents);

    println!("Time Elapsed - Loading File: {:?}", start.elapsed());

    let mut grid_p1 = grid.clone();

    let start = Instant::now();

    let mut x = 0;
    let mut y = 0;
    'outer: for (i, row) in grid_p1.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == '@' {
                x = j as i64;
                y = i as i64;
                break 'outer;
            }
        }
    }

    for m in moves.chars() {
        let mut moves_to_do: Vec<(i64, i64)> = Vec::new();
        let (dx, dy) = move_to_coords(m);
        moves_to_do.push((x, y));
        attempt_move(&grid_p1, &mut moves_to_do, x, y, dx, dy);
        if moves_to_do.len() > 0 {
            for i in (0..moves_to_do.len()).rev() {
                let (nx, ny) = moves_to_do[i];
                let char = grid_p1[ny as usize][nx as usize];
                grid_p1[(ny + dy) as usize][(nx + dx) as usize] = char;
            }
            grid_p1[y as usize][x as usize] = '.';
            x += dx;
            y += dy;
        }
    }

    let mut result = 0;
    for (i, row) in grid_p1.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == 'O' {
                result += 100 * i + j;
            }
        }
    }

    let duration = start.elapsed();
    println!("Result - Part1: {}", result);
    println!("Time Elapsed - Part1: {:?}", duration);


    let mut grid_p2 = transform_grid(&grid);
    let mut x = 0;
    let mut y = 0;
    'outer: for (i, row) in grid_p2.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == '@' {
                x = j as i64;
                y = i as i64;
                break 'outer;
            }
        }
    }

    for m in moves.chars() {
        let mut moves_to_do: Vec<(i64, i64)> = Vec::new();
        let (dx, dy) = move_to_coords(m);
        moves_to_do.push((x, y));
        attempt_move(&grid_p2, &mut moves_to_do, x, y, dx, dy);
        if moves_to_do.len() > 0 {
            match m {
                '^' => {
                    moves_to_do.sort_by(|a, b| a.1.cmp(&b.1));
                }
                'v' => {
                    moves_to_do.sort_by(|a, b| b.1.cmp(&a.1));
                }
                '>' => {
                    moves_to_do.sort_by(|a, b| b.0.cmp(&a.0));
                }
                '<' => {
                    moves_to_do.sort_by(|a, b| a.0.cmp(&b.0));
                }
                _ => {}
            }

            for i in 0..moves_to_do.len() {
                let (nx, ny) = moves_to_do[i];
                let temp = grid_p2[(ny + dy) as usize][(nx + dx) as usize];
                grid_p2[(ny + dy) as usize][(nx + dx) as usize] = grid_p2[ny as usize][nx as usize];
                grid_p2[ny as usize][nx as usize] = temp;
            }

            x += dx;
            y += dy;
        }
    }

    let mut result = 0;
    for (i, row) in grid_p2.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == '[' {
                result += 100 * i + j;
            }
        }
    }

    let duration = start.elapsed();
    println!("Result - Part2: {}", result);
    println!("Time Elapsed - Part2: {:?}", duration);
}

fn attempt_move(
    grid: &Vec<Vec<char>>,
    moves_to_do: &mut Vec<(i64, i64)>,
    x: i64,
    y: i64,
    dx: i64,
    dy: i64,
) {
    if moves_to_do.len() == 0 {
        return;
    }

    let (new_x, new_y) = (x + dx, y + dy);
    let next_char = grid[new_y as usize][new_x as usize];
    if next_char == '#' {
        moves_to_do.clear();
        return;
    }
    if next_char == 'O' {
        moves_to_do.push((new_x, new_y));
        attempt_move(grid, moves_to_do, new_x, new_y, dx, dy);
        return;
    }
    if next_char == '[' || next_char == ']' {
        if dy == 0 {
            moves_to_do.push((new_x, new_y));
            attempt_move(grid, moves_to_do, new_x, new_y, dx, dy);
        } else {
            attempt_move(grid, moves_to_do, new_x, new_y, dx, dy);

            if next_char == '[' {
                attempt_move(grid, moves_to_do, new_x + 1, new_y, dx, dy);
            } else {
                attempt_move(grid, moves_to_do, new_x - 1, new_y, dx, dy);
            }

            if moves_to_do.len() == 0 {
                return;
            }

            // idk brute force solution to stupid bug, but it works
            if !moves_to_do.contains(&(new_x, new_y)) {
                moves_to_do.push((new_x, new_y));
            }
            if next_char == '[' && !moves_to_do.contains(&(new_x + 1, new_y)) {
                moves_to_do.push((new_x + 1, new_y));
            } else if next_char == ']' && !moves_to_do.contains(&(new_x - 1, new_y)) {
                moves_to_do.push((new_x - 1, new_y));
            }
        }
        return;
    }
    if next_char == '.' {
        return;
    }
}

fn move_to_coords(m: char) -> (i64, i64) {
    match m {
        '^' => (0, -1),
        'v' => (0, 1),
        '<' => (-1, 0),
        '>' => (1, 0),
        _ => (0, 0),
    }
}

fn transform_grid(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_grid = vec![vec!['.'; grid[0].len() * 2]; grid.len()];

    for (i, row) in grid.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            match c {
                '#' => {
                    new_grid[i][j * 2] = '#';
                    new_grid[i][j * 2 + 1] = '#';
                }
                'O' => {
                    new_grid[i][j * 2] = '[';
                    new_grid[i][j * 2 + 1] = ']';
                }
                '.' => {
                    new_grid[i][j * 2] = '.';
                    new_grid[i][j * 2 + 1] = '.';
                }
                '@' => {
                    new_grid[i][j * 2] = '@';
                    new_grid[i][j * 2 + 1] = '.';
                }
                _ => {}
            }
        }
    }

    return new_grid;
}

fn parse_data(data: &str) -> (Vec<Vec<char>>, &str) {
    // split by empty line
    let mut parts = data.split("\r\n\r\n");
    // unwrap the first part and split by newline
    let part1: Vec<Vec<char>> = parts
        .next()
        .expect("No part1 found")
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    // unwrap the second part
    let part2 = parts.next().expect("No part2 found");
    (part1, part2.trim())
}

fn read_file(filename: &str) -> String {
    let mut file = File::open(filename).expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read file");
    contents
}
