// Advent of Code 2022 - Day 1
// Read batches of numbers from the input file, and find the batch with the largest sum.

use std::fs::File;
use std::io::{BufRead, BufReader, Seek};

fn argmax_calories<R: BufRead>(reader: &mut R, top_n: usize) -> i32 {
    let mut max = vec![0; top_n];
    let mut min_idx = 0;
    let mut min = 0;
    let mut sum = 0;

    reader.lines().for_each(|line| {
        // If the line is empty, we've reached the end of a batch. Check whether the
        // sum is larger than the previous maximum, save it if it is, and reset the sum.
        if line.as_ref().unwrap().is_empty() {
            if sum > min {
                max[min_idx] = sum;
                min_idx = max.iter().enumerate().min_by_key(|(_, &v)| v).unwrap().0;
                min = max[min_idx];
            }
            sum = 0;
        } else {
            // Otherwise, add the number to the sum.
            sum += line.unwrap().parse::<i32>().unwrap();
        }
    });

    // If we've reached to end of the file, we still need to check the last batch.
    if sum > min {
        max[min_idx] = sum;
    }

    println!("Final: {:?}", max);
    max.iter().sum()
}

fn main() {
    let file = File::open("data/input.txt").unwrap();
    let mut reader = BufReader::new(file);
    let result = argmax_calories(&mut reader, 1);
    println!("Part 1: {:?}", result);

    reader.rewind().unwrap();
    let result = argmax_calories(&mut reader, 3);
    println!("Part 2: {:?}", result);
}
