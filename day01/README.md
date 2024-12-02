# Day 1

Link to the problem on the Advent of Code website - [click me](https://adventofcode.com/2024/day/1)

## Description

We so back 🎄

Decided to do the first challenge with C++ to keep things simple.

Overall, quite easy first challenge.

### Part 1

#### Textual explanation of my solution:

```
1. Read the 'input.txt' line by line
    1.1 Decipher the input to left and right list respectively.
2. Sort left and right list.
3. Compare distances from smallest -> largest.
4. Return final total distance.
```

#### Complexity analysis:
Where `n` is the amount of lines in the input.
```
Read the input:         O(n)
Sort the lists:         O(n * log n)
Compare the distances:  O(n)
```

- Time elapsed: 0.0009023 seconds
- Complexity:   O(n * log n)

### Part 2

#### Textual explanation of my solution:
Continuing from part 1.
```
5. Index the right list accordingly to the problem (count occurrences of each element).
6. Calculate the total similarity between the two lists.
7. Return the total similarity.
```

#### Complexity analysis:
Where `n` is the amount of lines in the input.
```
Index the right list:   O(n)
Calculate similarity:   O(n)
```

- Time elapsed: 0.0004224 seconds
- Complexity: O(n)

## How to run

### Prerequisites

- C++ compiler

### Run the code

```bash
1. Clone the repo               |   git clone
2. Navigate into todays folder  |   cd day<x>/src
3. Compile & run the code       |   g++ main.cpp -o main && main.exe # Or whatever compiler you use
```

