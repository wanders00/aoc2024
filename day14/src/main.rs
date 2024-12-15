use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

#[derive(Clone)]
struct Robot {
    pos: UPoint,
    vel: IPoint,
}

impl Robot {
    fn new(pos_x: u64, pos_y: u64, vel_x: i64, vel_y: i64) -> Robot {
        Robot {
            pos: UPoint::new(pos_x, pos_y),
            vel: IPoint::new(vel_x, vel_y),
        }
    }

    fn update_position_with_bounds(&mut self, max_width: usize, max_height: usize) {
        let new_x = (self.pos.x as i64 + self.vel.x).rem_euclid(max_width as i64);
        let new_y = (self.pos.y as i64 + self.vel.y).rem_euclid(max_height as i64);
        self.pos.x = new_x as u64;
        self.pos.y = new_y as u64;
    }
}

impl fmt::Display for Robot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Position: {}, Velocity: {}", self.pos, self.vel)
    }
}

#[derive(Clone)]
struct IPoint {
    x: i64,
    y: i64,
}

impl IPoint {
    fn new(x: i64, y: i64) -> IPoint {
        IPoint { x, y }
    }
}

impl fmt::Display for IPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Clone)]
struct UPoint {
    x: u64,
    y: u64,
}

impl UPoint {
    fn new(x: u64, y: u64) -> UPoint {
        UPoint { x, y }
    }
}

impl fmt::Display for UPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    let start = Instant::now();
    match read_file("input.txt") {
        Ok(mut robots) => {
            println!("Time Elapsed - Loading File: {:?}", start.elapsed());

            let start = Instant::now();
            solve(&mut robots.clone(), false);
            println!("Time Elapsed - Part1: {:?}", start.elapsed());
            let start = Instant::now();
            solve(&mut robots, true);
            println!("Time Elapsed - Part2: {:?}", start.elapsed());
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
        }
    }
}

fn solve(robots: &mut Vec<Robot>, part2: bool) {
    let width = 101;
    let height = 103;
    let mut iterations = 100;
    if part2 {
        iterations = iterations * 100;
    }
    iterations += 1;
    for i in 1..iterations {
        for r in robots.iter_mut() {
            r.update_position_with_bounds(width, height);
        }
        if part2 {
           if find_easter_egg(i, &robots, width, height) {
            break;
           }
        }
    }
    if part2 {
        return;
    }
    // top left, top right, bottom left, bottom right
    let mut quadrant = (0, 0, 0, 0);
    let vertical_line = width as u64 / 2;
    let horizontal_line = height as u64 / 2;
    for r in robots.iter() {
        if r.pos.x < vertical_line && r.pos.y < horizontal_line {
            quadrant.0 += 1;
        } else if r.pos.x > vertical_line && r.pos.y < horizontal_line {
            quadrant.1 += 1;
        } else if r.pos.x < vertical_line && r.pos.y > horizontal_line {
            quadrant.2 += 1;
        } else if r.pos.x > vertical_line && r.pos.y > horizontal_line {
            quadrant.3 += 1;
        }
    }
    let result = quadrant.0 * quadrant.1 * quadrant.2 * quadrant.3;
    println!("Result: {}", result);
}

fn find_easter_egg(iteration: usize, robots: &Vec<Robot>, width: usize, height: usize) -> bool {
    let mut grid = vec![vec!['.'; width]; height];

    for robot in robots {
        let x = robot.pos.x as usize;
        let y = robot.pos.y as usize;
        if x < width && y < height {
            grid[y][x] = 'R';
        }
    }

    let mut count = 0;
    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    for y in 0..height {
        for x in 0..width {
            if grid[y][x] == 'R' {
                for &(dx, dy) in &directions {
                    let nx = x as isize + dx;
                    let ny = y as isize + dy;
                    if nx >= 0 && nx < width as isize && ny >= 0 && ny < height as isize {
                        if grid[ny as usize][nx as usize] == 'R' {
                            count += 1;
                            break;
                        }
                    }
                }
            }
        }
    }

    if count > 300 {
        println!(
            "Iteration {}: {} 'R' cells have adjacent 'R' cells",
            iteration, count
        );

        // print grid
        for row in &grid {
            for c in row {
                print!("{}", c);
            }
            println!();
        }
        return true;
    }
    return false;
}

fn read_file(filename: &str) -> io::Result<Vec<Robot>> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);
    let mut robots = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            let pos = parts[0]
                .trim_start_matches("p=")
                .split(',')
                .collect::<Vec<&str>>();
            let vel = parts[1]
                .trim_start_matches("v=")
                .split(',')
                .collect::<Vec<&str>>();
            if pos.len() == 2 && vel.len() == 2 {
                let pos_x = pos[0].parse::<u64>().unwrap();
                let pos_y = pos[1].parse::<u64>().unwrap();
                let vel_x = vel[0].parse::<i64>().unwrap();
                let vel_y = vel[1].parse::<i64>().unwrap();
                let robot = Robot::new(pos_x, pos_y, vel_x, vel_y);
                robots.push(robot);
            }
        }
    }

    Ok(robots)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_position() {
        let mut r = Robot::new(2, 4, 2, -3);
        r.update_position_with_bounds(11, 7);
        assert_eq!((r.pos.x, r.pos.y), (4, 1));
        r.update_position_with_bounds(11, 7);
        assert_eq!((r.pos.x, r.pos.y), (6, 5));
        r.update_position_with_bounds(11, 7);
        assert_eq!((r.pos.x, r.pos.y), (8, 2));
        r.update_position_with_bounds(11, 7);
        assert_eq!((r.pos.x, r.pos.y), (10, 6));
        r.update_position_with_bounds(11, 7);
        assert_eq!((r.pos.x, r.pos.y), (1, 3));
    }
}
