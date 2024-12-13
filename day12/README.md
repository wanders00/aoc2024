# Day 12

Link to the problem on the Advent of Code website - [click me](https://adventofcode.com/2024/day/12)

## Description

First day not being on time with the solution :-(

Why does life come in the way of advent of code???

Anyways, part1, easy, fun, and quick.

Part2, not so much. I have to admit that I needed to look up quite a bit of hints on how to figure out a solution. It was just hard for me to come up with an algorithm that could tackle the problem.

I ended up using a very simple algorithm, and it is very intuitive when you think about it. But I just couldn't come up with it myself.

Basically, find all perimeters and store them according with their direction. Afterwards, loop through all, and find if there exists a perimeter at `y+1` or `x+1` with the same direction. If so, remove one. Then afterwards `perimeter.len() = cost`.

### Time elapsed

- Loading File: 0,0005535s

- Part 1:       0,032982s

- Part 2:       0,0571142s

## How to run

### Prerequisites

- Rust / Cargo installed

### Run the code

```bash
1. Clone the repo               |   git clone
2. Navigate into todays folder  |   cd day<x>
3. Compile & run the code       |   cargo run
```

