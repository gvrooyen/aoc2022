// Advent of Code 2022 - Day 6
// Find start-of packet markers in a stream of characters.

use std::collections::{HashSet, VecDeque};
use std::fs;

fn find_marker(stream: Vec<char>, len: usize) -> Option<usize> {
    let mut win = VecDeque::new();
    win.extend(stream.iter().take(len));
    let mut i = len;
    while i < stream.len() {
        let chars: HashSet<char> = HashSet::from_iter(win.iter().cloned());
        if chars.len() == len {
            return Some(i);
        }
        win.pop_front();
        win.push_back(stream[i]);
        i += 1;
    }
    None
}

fn main() {
    // Assume that the input file has only one line, and read it.
    let stream = fs::read_to_string("data/input.txt").expect("Unable to read file");
    let stream: Vec<char> = stream.chars().collect();
    let pos = find_marker(stream, 14);
    println!("Found marker at position {}", pos.unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_marker() {
        assert_eq!(
            find_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb".chars().collect(), 4),
            Some(7)
        );
        assert_eq!(
            find_marker("bvwbjplbgvbhsrlpgdmjqwftvncz".chars().collect(), 4),
            Some(5)
        );
        assert_eq!(
            find_marker("nppdvjthqldpwncqszvftbrmjlhg".chars().collect(), 4),
            Some(6)
        );
        assert_eq!(
            find_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".chars().collect(), 4),
            Some(10)
        );
        assert_eq!(
            find_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".chars().collect(), 4),
            Some(11)
        );
    }
}
