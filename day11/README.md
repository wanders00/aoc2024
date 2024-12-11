# Day 11

Link to the problem on the Advent of Code website - [click me](https://adventofcode.com/2024/day/11)

## Description

Oh boy, today was a fun one :-)

For part1 everything worked fine and dandy with just looping through the ordinary way, and I got my solution that way.

But for part2 I was greeted with this beauty:

```
memory allocation of 68719476736 bytes failed
error: process didn't exit successfully: target\debug\day11.exe (exit code: 0xc0000409, STATUS_STACK_BUFFER_OVERRUN)
```

Casual 64GB memory allocation on the 48th iteration...

So for part2 I had to redo the algorithm, I did look up some hints on how to design it, but it was very straight forward and it makes a lot of sense why it is so much better.

Basically just using a HashMap with `key: number, value: count` and then you can process every single duplicate of the number at once, instead of for an example doing a `0` or a `1` for the hundred millionth time.

### Time elapsed

- Loading File: 0,0001022s

- Part 1:       0,0022679s

- Part 2:       0,0946609s

## How to run

### Prerequisites

- Rust / Cargo installed

### Run the code

```bash
1. Clone the repo               |   git clone
2. Navigate into todays folder  |   cd day<x>
3. Compile & run the code       |   cargo run
```

