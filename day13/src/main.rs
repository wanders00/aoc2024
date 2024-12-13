use regex::Regex;
use std::fs;
use std::time::Instant;

fn extract_numbers(input: &str) -> Vec<Vec<i64>> {
    let re = Regex::new(r"\d+").unwrap();
    let numbers: Vec<i64> = re
        .find_iter(input)
        .map(|mat| mat.as_str().parse::<i64>().unwrap())
        .collect();

    numbers.chunks(6).map(|chunk| chunk.to_vec()).collect()
}

fn can_combine(input: (i64, i64, i64, i64, i64, i64), part2: bool) -> i64 {
    let (x1, x2, y1, y2, mut z1, mut z2) = input;
    z1 += 10000000000000 * part2 as i64;
    z2 += 10000000000000 * part2 as i64;
    let b = (z2 * x1 - z1 * x2) / (y2 * x1 - y1 * x2);
    let a = (z1 - b * y1) / x1;
    ((x1 * a + y1 * b, x2 * a + y2 * b) == (z1, z2)) as i64 * (a * 3 + b)
}

fn main() {
    let start = Instant::now();
    let input = fs::read_to_string("src/input.txt").expect("Failed to read file");
    let numbers = extract_numbers(&input);

    let duration = start.elapsed();
    println!("Time Elapsed - Loading File: {:?}", duration);

    let start = Instant::now();

    let mut result = 0;
    for number in &numbers {
        result += can_combine(
            (
                number[0], number[1], number[2], number[3], number[4], number[5],
            ),
            false,
        );
    }

    let duration = start.elapsed();
    println!("{}", result);
    println!("Time Elapsed - Part1: {:?}", duration);

    let start = Instant::now();

    let mut result = 0;
    for number in &numbers {
        result += can_combine(
            (
                number[0], number[1], number[2], number[3], number[4], number[5],
            ),
            true,
        );
    }

    let duration = start.elapsed();
    println!("{}", result);
    println!("Time Elapsed - Part2: {:?}", duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        let input = "Button A: X+94, Y+34
    Button B: X+22, Y+67
    Prize: X=8400, Y=5400
    
    Button A: X+26, Y+66
    Button B: X+67, Y+21
    Prize: X=12748, Y=12176
    
    Button A: X+17, Y+86
    Button B: X+84, Y+37
    Prize: X=7870, Y=6450
    
    Button A: X+69, Y+23
    Button B: X+27, Y+71
    Prize: X=18641, Y=10279";

        let numbers = extract_numbers(input);
        let mut result = 0;
        for number in &numbers {
            result += can_combine(
                (
                    number[0], number[1], number[2], number[3], number[4], number[5],
                ),
                false,
            );
        }
        assert_eq!(result, 480);
    }
}
