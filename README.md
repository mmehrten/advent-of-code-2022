# advent-of-code-2022
Solutions to [Advent of Code 2022](adventofcode.com).

To run:

```sh
$ cargo install --path .
$ aoc <day number>
```

For example, to run the day 1 solution:

```sh
$ aoc 1
Example: ...
Challenge: ...
```

## Adding new solutions

1. Create `Example-<Day>.txt` and `Challenge-<Day>.txt` in the `inputs` folder with the test data
2. Implement solution in `src/day<Day>.rs`
3. Import and call your solution in `src/main.rs`