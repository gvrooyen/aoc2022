// Advent of Code 2022 - Day 5
// Shuffling crates

use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Stack = Vec<Vec<char>>;

#[derive(Debug)]
struct Move {
    qty: u32,
    src: usize,
    dst: usize,
}

// Parse the specified input file, and return a `Stack` containter with the initial state of the
// stacks, and a `Vec` of `Move`s that describe the shuffling process.
fn parse_input(filename: &str) -> (Stack, Vec<Move>) {
    // This 2D vector of characters initially represents the first part of the input file, which
    // describes the initial state of the stacks.
    let mut raw_stack: Vec<Vec<char>> = Vec::new();
    // This will eventually store the moves between stacks.
    let mut moves = Vec::new();
    let mut num_stacks = 0;

    let reader = BufReader::new(File::open(filename).expect("Could not open file"));
    let lines = reader.lines();

    let mut file_part = 0;

    for (i, line) in lines.enumerate() {
        let line = line.expect("Could not read line");
        if i == 0 {
            num_stacks = (line.len() + 1) / 4;
        }
        if line.is_empty() {
            file_part += 1;
            continue;
        }

        if file_part == 0 {
            // This is the first part of the input file, which describes the initial state of the
            // stacks.
            raw_stack.push(Vec::new());
            for (j, c) in line.chars().enumerate() {
                if j % 4 == 1 {
                    raw_stack[i].push(c);
                }
            }
        } else {
        }
    }

    println!("{:?}", raw_stack);

    // This will eventually store the initial state of the stacks.
    let mut stack: Stack = vec![Vec::new(); num_stacks];

    // Convert the `raw_stack` vector into a `Stack` container. This is done by iterating over
    // `raw_stack` from next-to-last to first, and pushing each non-space character to the
    // corresponding `stack` vector.
    for i in (0..num_stacks - 1).rev() {
        for (j, c) in raw_stack[i].iter().enumerate() {
            if *c != ' ' {
                stack[j].push(*c);
            }
        }
    }

    println!("{:?}", stack);

    (stack, moves)
}

fn main() {
    let (stack, moves) = parse_input("data/input.txt");
    println!("Stack length: {}", stack.len());
    println!("Number of moves: {}", moves.len());
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let (stacks, moves) = parse_input("data/test.txt");
        assert_eq!(stacks.len(), 3);
        assert_eq!(stacks[0].len(), 2);
        assert_eq!(stacks[1].len(), 3);
        assert_eq!(stacks[2].len(), 1);
        assert_eq!(moves.len(), 4);
    }
}
