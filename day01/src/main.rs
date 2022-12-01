// Advent of Code 2022 - Day 1
// Read batches of numbers from the input file, and find the batch with the largest sum.

use std::fs::File;
use std::io::{BufRead, BufReader};

fn argmax_calories<R: BufRead>(reader: &mut R) -> i32 {
    let mut max = 0;
    let mut sum = 0;

    reader.lines().for_each(|line| {
        // If the line is empty, we've reached the end of a batch. Check whether the
        // sum is larger than the previous maximum, save it if it is, and reset the sum.
        if line.as_ref().unwrap().is_empty() {
            if sum > max {
                max = sum;
            }
            sum = 0;
        } else {
            // Otherwise, add the number to the sum.
            sum += line.unwrap().parse::<i32>().unwrap();
        }
    });

    // If we've reached to end of the file, we still need to check the last batch.
    if sum > max {
        max = sum;
    }

    max
}

fn main() {
    let file = File::open("data/input.txt").unwrap();
    let mut reader = BufReader::new(file);
    let result = argmax_calories(&mut reader);
    println!("Part 1: {:?}", result);
}
