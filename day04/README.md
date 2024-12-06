# Day 4

Link to the problem on the Advent of Code website - [click me](https://adventofcode.com/2024/day/4)

## Description

Today we use crab lang 🦀 and recursion.

I am bad at rust, do not look too much into the code...

However, I am still quite happy with my solutions. At least for part 1. Part 2 had a bit more icky solution, but it works and I am happy enough with it.

### Part 1

#### Textual explanation:

```
1. Check each character, check if it matches with the first letter of the word ('XMAS').
2. If it does, check in every direction, whether the next character is the next letter in the word.
3. Recursively check the next character in the word, until the word is found. 
4. Count the number of times the word is found.
```

- Time elapsed: 4.0463ms 

### Part 2

#### Textual explanation of my solution:
```
1. Check each character, check if it matches with the middle letter ('A') of the word ('MAS').
2. If it does, check in every diagonal direction, whether the next character is the first or last letter in the word.
3. Check if the found first and last letters match up with any of the correct formations.
4. Count the number of times the cross-shape of the word is found.
```

- Time elapsed: 5.7305ms 

## How to run

### Prerequisites

- Rust / Cargo installed

### Run the code

```bash
1. Clone the repo               |   git clone
2. Navigate into todays folder  |   cd day<x>
3. Compile & run the code       |   cargo run
```

