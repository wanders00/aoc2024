use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::time::Instant;
use std::vec;

struct Position {
    row: isize,
    col: isize,
}

fn main() -> io::Result<()> {
    let filename = "src/input.txt";
    let start = Instant::now();
    
    match read_file(filename) {
        Ok(lines) => {
            println!("Result: {}", word_occurrence(lines, "XMAS"));
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    let duration = start.elapsed();
    println!("Time elapsed - Part 1: {:?}", duration);

    let start = Instant::now();

    match read_file(filename) {
        Ok(lines) => {
            println!("Cross Result: {}", cross_word_occurrence(lines, "MAS"));
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    let duration = start.elapsed();
    println!("Time elapsed - Part 2: {:?}", duration);

    Ok(())
}

/// Counts the number of occurrences of the given string in various directions within the given 2D vector of characters.
///
/// The function checks for the string in the following directions:
/// - Left to right
/// - Right to left
/// - Up to down
/// - Down to up
/// - Diagonally (4 directions)
///
/// ### Arguments
///
/// * `lines` - A vector of vectors of characters representing the contents of the file.
/// * `word` - The string to search for in the 2D vector of characters.
///
/// ### Returns
///
/// * `usize` - The number of occurrences of the string in the given 2D vector of characters.
fn word_occurrence(lines: Vec<Vec<char>>, word: &str) -> usize {
    let directions: Vec<Position> = vec![
        Position { row: 0, col: 1 },   // Left to right
        Position { row: 0, col: -1 },  // Right to left
        Position { row: 1, col: 0 },   // Up to down
        Position { row: -1, col: 0 },  // Down to up
        Position { row: 1, col: 1 },   // Diagonally top-left to bottom-right
        Position { row: -1, col: -1 }, // Diagonally bottom-right to top-left
        Position { row: 1, col: -1 },  // Diagonally top-right to bottom-left
        Position { row: -1, col: 1 },  // Diagonally bottom-left to top-right
    ];

    let mut count = 0;

    for i in 0..lines.len() {
        let line = &lines[i];
        for j in 0..line.len() {
            let ch = line[j];
            if let Some(char) = word.chars().nth(0) {
                if ch == char {
                    for direction in &directions {
                        let start = Position {
                            row: i as isize,
                            col: j as isize,
                        };
                        count = count + move_in_direction(&lines, start, &direction, word, 1)
                    }
                }
            }
        }
    }

    return count;
}

/// Moves in the specified direction within the 2D vector of characters to check for the occurrence of the given string.
///
/// This function is called recursively to check each character in the specified direction until the entire string is found or the search fails.
///
/// ### Arguments
///
/// * `lines` - A reference to a vector of vectors of characters representing the contents of the file.
/// * `start` - The starting position (row and column) for the search.
/// * `direction` - The direction to move in for the search (row and column offsets).
/// * `word` - The string to search for in the 2D vector of characters.
/// * `chr_index` - The current index of the character in the string being checked.
///
/// ### Returns
///
/// * `usize` - Returns 1 if the string is found in the specified direction, otherwise returns 0.
fn move_in_direction(
    lines: &Vec<Vec<char>>,
    start: Position,
    direction: &Position,
    word: &str,
    chr_index: usize,
) -> usize {
    let new_row = start.row + direction.row;
    let new_col = start.col + direction.col;
    if let Some(next) = word.chars().nth(chr_index) {
        if let Some(row) = lines.get(new_row as usize) {
            if let Some(&ch) = row.get(new_col as usize) {
                if ch == next {
                    if chr_index == word.len() - 1 {
                        return 1;
                    } else {
                        return move_in_direction(
                            lines,
                            Position {
                                row: new_row,
                                col: new_col,
                            },
                            direction,
                            word,
                            chr_index + 1,
                        );
                    }
                }
            }
        }
    }

    return 0;
}

/// Counts the number of occurrences of the given string in a cross within the given 2D vector of characters.
/// String.len must equal 3
///
/// ### Arguments
///
/// * `lines` - A vector of vectors of characters representing the contents of the file.
/// * `word` - The string to search for in the 2D vector of characters.
///
/// ### Returns
///
/// * `usize` - The number of occurrences a cross shaped form of the string in the given 2D vector of characters.
fn cross_word_occurrence(lines: Vec<Vec<char>>, word: &str) -> usize {
    let directions: Vec<Position> = vec![
        Position { row: 1, col: 1 },   // Diagonally top-left to bottom-right
        Position { row: -1, col: -1 }, // Diagonally bottom-right to top-left
        Position { row: 1, col: -1 },  // Diagonally top-right to bottom-left
        Position { row: -1, col: 1 },  // Diagonally bottom-left to top-right
    ];

    let mut count = 0;

    let first_char = match word.chars().nth(0) {
        Some(c) => c,
        None => return 0,
    };

    let last_char = match word.chars().nth(2) {
        Some(c) => c,
        None => return 0,
    };

    let solution_1: Vec<Vec<usize>> = vec![vec![1, 0, 1, 0], vec![0, 1, 0, 1]];
    let solution_2: Vec<Vec<usize>> = vec![vec![0, 1, 1, 0], vec![1, 0, 0, 1]];
    let solution_3: Vec<Vec<usize>> = vec![vec![0, 1, 0, 1], vec![1, 0, 1, 0]];
    let solution_4: Vec<Vec<usize>> = vec![vec![1, 0, 0, 1], vec![0, 1, 1, 0]];

    for i in 0..lines.len() {
        let line = &lines[i];
        for j in 0..line.len() {
            let mut first_char_count: Vec<usize> = vec![0; 4];
            let mut last_char_count: Vec<usize> = vec![0; 4];
            let ch = line[j];
            if let Some(char) = word.chars().nth(1) {
                if ch == char {
                    for (index, direction) in directions.iter().enumerate() {
                        first_char_count[index] = find_next_char_in_direction(
                            &lines,
                            Position {
                                row: i as isize,
                                col: j as isize,
                            },
                            &direction,
                            first_char,
                        );
                        last_char_count[index] = find_next_char_in_direction(
                            &lines,
                            Position {
                                row: i as isize,
                                col: j as isize,
                            },
                            &direction,
                            last_char,
                        );
                    }

                    if first_char_count == solution_1[0] && last_char_count == solution_1[1] {
                        count += 1;
                    } else if first_char_count == solution_2[0] && last_char_count == solution_2[1]
                    {
                        count += 1;
                    } else if first_char_count == solution_3[0] && last_char_count == solution_3[1]
                    {
                        count += 1;
                    } else if first_char_count == solution_4[0] && last_char_count == solution_4[1]
                    {
                        count += 1;
                    }
                }
            }
        }
    }

    return count;
}

/// Finds the next character in the specified direction within the 2D vector of characters.
///
/// ### Arguments
///
/// * `lines` - A reference to a vector of vectors of characters representing the contents of the file.
/// * `start` - The starting position (row and column) for the search.
/// * `direction` - The direction to move in for the search (row and column offsets).
/// * `next` - The character to search for in the specified direction.
///
/// ### Returns
///
/// * `usize` - Returns 1 if the char is found in the specified direction, otherwise returns 0.
fn find_next_char_in_direction(
    lines: &Vec<Vec<char>>,
    start: Position,
    direction: &Position,
    next: char,
) -> usize {
    let new_row = start.row + direction.row;
    let new_col = start.col + direction.col;

    if let Some(row) = lines.get(new_row as usize) {
        if let Some(&ch) = row.get(new_col as usize) {
            if ch == next {
                return 1;
            }
        }
    }

    return 0;
}

/// Reads the file and returns its contents as a vector of vectors of characters.
///
/// ### Arguments
///
/// * `filename` - The name of the file to read.
///
/// ### Returns
///
/// * `io::Result<Vec<Vec<char>>>` - The result of the file reading operation.
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
