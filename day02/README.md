# Day 2

Link to the problem on the Advent of Code website - [click me](https://adventofcode.com/2024/day/2)

## Description

Kept going with C++ for this one as well. Was quite nice to work with something im familiar with as I have barely coded all semester, due to that I've just been having mostly math courses.

The solution to the problems first part was kind of similar to the day 1 problem. Therefore, I started off todays problem with just copying what I had done from yesterday and then just modifying it to fit the new problem.

However, once I got to part 2 I saw that I had to change up a lot more than I had done for part 1. Thus, the code ended up being quite different from the day 1 code.

### Reading the input

#### Textual explanation:

```
1. Read the 'input.txt' line by line
2. Transform each line into a vector of integers.
```

#### Complexity analysis:
Where `l` is the amount of lines in the input, and `n` is the amount of characters in each line.
```
Read the input:         O(l)
String to integers:     O(l * n)
Transform to vector:    O(l * n)
Pushing vector to list: O(l)
```

#### 

- Time elapsed: 0.0045335 seconds
- Complexity:   O(n)

### Part 1

#### Textual explanation:

```
3. Call the function 'is_safe' for each line in the input.
4. Count the amount of lines that are safe.
```

#### Complexity analysis:
Where `l` is the amount of lines in the input.
```
Check line safety:      O(n)
```

- Time elapsed: 0.0010164 seconds
- Complexity:   O(n)

### Part 2

#### Textual explanation of my solution:
```
5. Call the function 'is_safe_remove_one' for each line in the input.
    5.1. Call the function 'is_safe' removing one character at a time.
```

#### Complexity analysis:
Where `l` is the amount of lines in the input.
```
Check line safety removing one character at a time: O(l * n)
```

- Time elapsed: 0.0023553
- Complexity:   O(l * n)

## How to run

### Prerequisites

- C++ compiler

### Run the code

```bash
1. Clone the repo               |   git clone
2. Navigate into todays folder  |   cd day<x>/src
3. Compile & run the code       |   g++ main.cpp -o main && main.exe # Or whatever compiler you use
```

