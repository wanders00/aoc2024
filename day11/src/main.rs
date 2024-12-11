use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};
use std::time::Instant;

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
    let content2 = content.clone();
    println!("Time elapsed - Reading File: {:?}", start.elapsed());

    let start = Instant::now();
    let result = solve(content, 25);
    let duration = start.elapsed();
    println!("Result - Part 1: {}", result);
    println!("Time elapsed - Part 1: {:?}", duration);

    let start = Instant::now();
    let result = solve(content2, 75);
    let duration = start.elapsed();
    println!("Result - Part 2: {}", result);
    println!("Time elapsed - Part 2: {:?}", duration);
}

fn solve(mut content: HashMap<i64, isize>, iteration: usize) -> isize {
    for _ in 0..iteration {
        let mut edit = Vec::new();

        for (&k, &v) in &content {
            if k == 0 {
                edit.push((k, -v));
                edit.push((1, v));
                continue;
            }

            let k_str = k.to_string();
            if k_str.len() % 2 == 0 {
                let (first, second) = k_str.split_at(k_str.len() / 2);
                first.parse::<i64>().unwrap();
                edit.push((k, -v));
                edit.push((first.parse::<i64>().unwrap(), v));
                edit.push((second.parse::<i64>().unwrap(), v));
            } else {
                edit.push((k, -v));
                edit.push((k * 2024, v));
            }
        }

        for (ek, ev) in edit {
            let entry = content.entry(ek).or_insert(0);
            *entry += ev;
            if *entry <= 0 {
                content.remove(&ek);
            }
        }
    }

    return content.values().sum::<isize>();
}

fn read_file(filename: &str) -> io::Result<HashMap<i64, isize>> {
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
        Ok(_) => {
            let mut word_count = HashMap::new();
            for word in content.split_whitespace() {
                if let Ok(key) = word.parse::<i64>() {
                    let counter = word_count.entry(key).or_insert(0);
                    *counter += 1;
                } else {
                    eprintln!("Failed to parse word: {}", word);
                }
            }
            Ok(word_count)
        }
        Err(e) => {
            eprintln!("Failed to read file: {}", e);
            Err(e)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let mut input = HashMap::new();
        input.insert(125, 1);
        input.insert(17, 1);
        assert_eq!(solve(input.clone(), 6), 22);
        assert_eq!(solve(input, 25), 55312);
    }
}