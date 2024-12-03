# Day 3

Link to the problem on the Advent of Code website - [click me](https://adventofcode.com/2024/day/3)

## Description

Todays problems was mostly solved with the magic of regex 😎

I quite enjoyed it. I was struggling a bit with the second part, but was very fulfilling when I got it to work. The problem was that I was setting a 'size_t' variable to -1, which is not a valid value for that type without realizing it. Then afterwards my loop was getting duplicates, but once I fixed that I got the correct answer.

### Reading the input

- Time elapsed: 0.0007557 

### Part 1

#### Textual explanation:

```
1. Use Regex to find matches.
2. Manipulate the string into a multiplication of the two numbers.
3. Sum the product of all the matches.
```

- Time elapsed: 0.0042147 seconds  

### Part 2

#### Textual explanation of my solution:
```
4. Reuse result from step 1.
5. Use regex to find "do()" and "don't()" matches. 
6. Find all pairs of "do()"s and "don't()"s.
7. Erase the substring between the two matches from the text (remove the invalid characters).
8. Manipulate the string into a multiplication of the two numbers.
9. Sum the product of all the matches.
```

- Time elapsed: 0.0079364 seconds 

## How to run

### Prerequisites

- C++ compiler

### Run the code

```bash
1. Clone the repo               |   git clone
2. Navigate into todays folder  |   cd day<x>/src
3. Compile & run the code       |   g++ main.cpp -o main && main.exe # Or whatever compiler you use
```

