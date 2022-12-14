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
use take_until::TakeUntilExt;

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

fn find_maxima(matrix: &[Vec<u32>]) -> HashSet<(usize, usize)> {
    let cols = matrix[0].len();
    let rows = matrix.len();
    let mut maxima = HashSet::new();

    for row in 0..rows {
        maxima.extend(scan_line(matrix, row, 0, (0, 1)));
        maxima.extend(scan_line(matrix, row, cols - 1, (0, -1)));
    }
    for col in 0..cols {
        maxima.extend(scan_line(matrix, 0, col, (1, 0)));
        maxima.extend(scan_line(matrix, rows - 1, col, (-1, 0)));
    }

    maxima
}

fn tree_score(matrix: &[Vec<u32>], row: usize, col: usize) -> u32 {
    let height = matrix[row][col];
    let rows = matrix.len();
    let cols = matrix[0].len();

    let t = (0..row)
        .rev()
        .take_until(|&r| matrix[r][col] >= height)
        .inspect(|&r| println!("Up: {} {}", r, col))
        .count();

    let b = (row + 1..rows)
        .take_until(|&r| matrix[r][col] >= height)
        .inspect(|&r| println!("Down: {} {}", r, col))
        .count();

    let l = (0..col)
        .rev()
        .take_until(|&c| matrix[row][c] >= height)
        .inspect(|&c| println!("Left: {} {}", row, c))
        .count();

    let r = (col + 1..cols)
        .take_until(|&c| matrix[row][c] >= height)
        .inspect(|&c| println!("Right: {} {}", row, c))
        .count();

    println!("For ({},{}): t={} b={} l={} r={}", row, col, t, b, l, r);

    (t * b * l * r) as u32
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
    let matrix = parse_input("data/input.txt");
    let maxima = find_maxima(&matrix);
    println!("Number of maxima: {}", maxima.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_line() {
        let matrix = parse_input("data/test.txt");
        let maxima = scan_line(&matrix, 0, 0, (0, 1));
        assert_eq!(maxima.len(), 2);
        assert!(maxima.contains(&(0, 0)));
        assert!(maxima.contains(&(0, 3)));
        let maxima = scan_line(&matrix, 0, 1, (1, 0));
        assert_eq!(maxima.len(), 2);
        assert!(maxima.contains(&(0, 1)));
        assert!(maxima.contains(&(1, 1)));
        let maxima = scan_line(&matrix, 3, 4, (0, -1));
        assert_eq!(maxima.len(), 1);
        assert!(maxima.contains(&(3, 4)));
        let maxima = scan_line(&matrix, 4, 2, (-1, 0));
        assert_eq!(maxima.len(), 2);
        assert!(maxima.contains(&(4, 2)));
        assert!(maxima.contains(&(3, 2)));
    }

    #[test]
    fn test_find_highest() {
        let matrix = parse_input("data/test.txt");
        let maxima = find_maxima(&matrix);
        assert!(maxima.contains(&(1, 1)));
        assert!(maxima.contains(&(1, 2)));
        assert!(!maxima.contains(&(1, 3)));
        assert!(maxima.contains(&(2, 1)));
        assert!(!maxima.contains(&(2, 2)));
        assert!(maxima.contains(&(2, 3)));
        assert!(maxima.contains(&(3, 2)));
        assert!(!maxima.contains(&(3, 1)));
        assert!(!maxima.contains(&(3, 3)));
        assert_eq!(maxima.len(), 21);
    }

    #[test]
    fn test_tree_score() {
        let matrix = parse_input("data/test.txt");
        assert_eq!(tree_score(&matrix, 1, 2), 4);
        assert_eq!(tree_score(&matrix, 3, 2), 8);
    }
}
