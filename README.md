# Advent of Code 2023

My submissions for advent of code 2023 in Rust.

## Structure

Every day is a new executable crate named `./days/day-01` to `./days/day-31` (**IF** I complete all puzzles this year...).

Handling boilerplate for getting the puzzle-input and running two parts are done in `aoc-core` which fetches and caches inputs.
`aoc-core` also declares a macro (`aoc_main! {}`) which wires everything up, only requiring focus on the problems:

````rust
fn part_one(input: &str) -> String
{
    let solution = format!("This string is {} chars long!!", input.len());

    solution
}

fn part_two(input: &str) -> u8
{
    123u8
}

aoc_main! { 2023, 1 };
````

I will probably add support for timing the solutions and stuff like that when my lazy a** gets around to it.

## Try a solution with automatic input downloading

Grab your session-key from the [website](https://adventofcode.com) (F12 for devtools and grab the cookie `session`).

Store this key
````dotenv
# In ./.env
AOC_TOKEN=your_token
````

## Try a solution without automatic downloading

Just grab the whole input and paste it in `./input/{year}-{day}.txt`. Example `./input/2023-02.txt`.