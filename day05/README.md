# Day 5

Link to the problem on the Advent of Code website - [click me](https://adventofcode.com/2024/day/5)

## Description

Today we use coffee lang ☕ and for-loops.

Quite a simple approach to this problem, its not the best complexity wise, but it works and I am happy with it.

In hindsight, I could've used the power of hashes even more to make the code have better complexity, and therefore also, better performance.

### Reading the input

#### Textual explanation:
    
```
1. Read the input.
2. Split the input by the one empty line. (Gets rules and updates separately)
3. Split the rules & updates by the newline character. 
4. Parse the rules into a hashmap, where 'key:value that must come before', 'value: list of values that must be after'.
5. Parse the updates into vectors of integers.
```

- Time elapsed: 0.0095428 seconds

### Part 1

#### Textual explanation:

```
1. Loop through each update.
    1.1 Create a HashSet of seen values.
    1.2 Loop through each value in the update.
        1.2.1 Check if the current value must come before any of the values in the seen set.
        1.2.2 If it does, break the loop and move to the next update. 
        1.2.3 Otherwise, add middle element to the counter.
```

- Time elapsed: 0.004751 seconds

### Part 2

#### Textual explanation of my solution:
```
1. Loop through each invalid_update.
    1.2 Loop through each value in the update.
        1.3. Loop through each value before the current one.
            1.4 Loop through all the rules.
                1.5 Check if the current rule is the one that must come before the current value.
                1.6 If it is, re-insert current element to before the first appearing element that violated a rule.
```

- Time elapsed: 0.0061482 seconds

## How to run

### Prerequisites

- Java installed

### Run the code

```bash
1. Clone the repo               |   git clone
2. Navigate into todays folder  |   cd day<x>
3. Compile & run the code       |   javac day05.java && java day05
```

