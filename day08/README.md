# Day 8

Link to the problem on the Advent of Code website - [click me](https://adventofcode.com/2024/day/8)

## Description

Rust today as well, what a surprise! (not)

Todays problem was mostly straightforward, the only difference I did between part 1 & 2 is that part 1 runs once and part is a loop of the same logic(ish).

I saw a lot of people were talking about struggling with the "validity of antinode placements" but I didn't have any issue with that.
I just implemented the logic as described in the problem and it worked kind of first time.

The only problem I had for today was that I missed out on this part of the problem description:
"This means that some of the new antinodes will occur at the position of each antenna (unless that antenna is the only one of its frequency)."

Which made me have a result a bit smaller than I was supposed to. To be honest this took me quite a while to figure out what was wrong with my code :-)

### Time elapsed

- Parsing File: 0.0002644s

- Part 1:       0.0000744s

- Part 2:       0.0001203s

## How to run

### Prerequisites

- Rust / Cargo installed

### Run the code

```bash
1. Clone the repo               |   git clone
2. Navigate into todays folder  |   cd day<x>
3. Compile & run the code       |   cargo run
```

