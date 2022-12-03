// Advent of Code 2022 - Day 3
// Find duplicate items in rucksacks

use std::collections::HashSet;

// Given a string representing the content of two backpacks, find the character that appears in
// both the first and the second half of the string. If there is no such character, return None.
fn find_duplicate(rucksack: &str) -> Option<char> {
    let mut seen = HashSet::new();

    // Split the input string into two halves
    let (first, second) = rucksack.split_at(rucksack.len() / 2);

    // Change the first half into a HashSet
    for c in first.chars() {
        seen.insert(c);
    }

    // Check if any of the characters in the second half are in the HashSet
    for c in second.chars() {
        if seen.contains(&c) {
            return Some(c);
        }
    }

    None
}

// The priority of a char is 1 to 27 for 'a' to 'z', and 28 to 54 for 'A' to 'Z'.
fn priority(c: char) -> u32 {
    if c.is_ascii_lowercase() {
        c as u32 - 'a' as u32 + 1
    } else {
        c as u32 - 'A' as u32 + 27
    }
}

fn main() {
    // Read the input from `data/input.txt`, and calculate the priority of the duplicate item in
    // each line. Accumulate the sum of the results.
    let sum: u32 = std::fs::read_to_string("data/input.txt")
        .expect("Failed to read input file")
        .lines()
        .map(|line| find_duplicate(line).expect("No duplicate found"))
        .map(priority)
        .sum();
    println!("{}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    // Create a static vector that represents the test rucksacks and their duplicate items.
    const TEST_DATA: &[(&str, char)] = &[
        ("vJrwpWtwJgWrhcsFMMfFFhFp", 'p'),
        ("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL", 'L'),
        ("PmmdzqPrVvPwwTWBwg", 'P'),
        ("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn", 'v'),
        ("ttgJtRGJQctTZtZT", 't'),
        ("CrZsJsPPZsGzwwsLwLmpwMDw", 's'),
    ];

    #[test]
    fn test_find_duplicate() {
        for (rucksack, duplicate) in TEST_DATA {
            assert_eq!(find_duplicate(rucksack), Some(*duplicate));
        }
    }

    #[test]
    fn test_priority() {
        assert_eq!(priority('a'), 1);
        assert_eq!(priority('z'), 26);
        assert_eq!(priority('A'), 27);
        assert_eq!(priority('Z'), 52);
    }
}
