// Advent of Code 2022 - Day 8
// Building a treehouse

// The strategy is to scan the matrix in all four directions, keeping track of the maxima in each
// row and column, for each direction. Each time a new maximum is found, the coordinates are added
// to the HashSet.
//
// An early stop on each scan can be done if a `9` is hit.

use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn scan_line(
    matrix: &[Vec<u32>],
    row: usize,
    col: usize,
    dir: (i32, i32),
) -> HashSet<(usize, usize)> {
    // The starting tree is always visible, and is the first maximum.
    let mut max = matrix[row][col];
    let mut maxima = HashSet::new();
    maxima.insert((row, col));
    let mut r = row as i32;
    let mut c = col as i32;
    loop {
        r += dir.0;
        c += dir.1;
        if r < 0 || r >= matrix.len() as i32 || c < 0 || c >= matrix[0].len() as i32 {
            break;
        }
        let v = matrix[r as usize][c as usize];
        if v > max {
            max = v;
            maxima.insert((r as usize, c as usize));
        }
        if v == 9 {
            break;
        }
    }
    maxima
}

fn find_highest(matrix: &[Vec<u32>]) -> HashSet<(usize, usize)> {
    let mut highest = HashSet::new();

    highest
}

fn parse_input(filename: &str) -> Vec<Vec<u32>> {
    let reader = BufReader::new(File::open(filename).expect("Could not open file"));
    let lines = reader.lines();
    lines
        .map(|l| {
            l.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_line() {
        let matrix = parse_input("data/test.txt");
        let mut maxima = scan_line(&matrix, 0, 0, (0, 1));
        assert_eq!(maxima.len(), 2);
        assert!(maxima.contains(&(0, 0)));
        assert!(maxima.contains(&(0, 3)));
    }

    #[test]
    fn test_find_highest() {
        let matrix = parse_input("data/test.txt");
        let highest = find_highest(&matrix);
        assert!(highest.contains(&(1, 1)));
        assert!(highest.contains(&(1, 2)));
        assert!(!highest.contains(&(1, 3)));
        assert!(highest.contains(&(2, 1)));
        assert!(!highest.contains(&(2, 2)));
        assert!(highest.contains(&(2, 3)));
        assert!(highest.contains(&(3, 2)));
        assert!(!highest.contains(&(3, 1)));
        assert!(!highest.contains(&(3, 3)));
    }
}
