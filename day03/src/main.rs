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

// Given a vector of strings, find the character common to each string.
// If there is more than one such character, or no such character, return None.
fn get_badge(rucksacks: &[&str]) -> Option<char> {
    let mut items: Vec<HashSet<char>> = Vec::new();

    // For each line in the input string, turn the line into a HashSet and add it to the `items`
    // vector.
    for rucksack in rucksacks {
        let mut seen = HashSet::new();
        for c in rucksack.chars() {
            seen.insert(c);
        }
        items.push(seen);
    }

    // Calculate the intersection of all the HashSets in the `items` vector.
    let mut intersection = items[0].clone();
    for item in items.iter().skip(1) {
        intersection = intersection.intersection(item).cloned().collect();
    }

    // If there is exactly one character in the intersection, return it.
    // Otherwise, return None.
    if intersection.len() == 1 {
        Some(*intersection.iter().next().unwrap())
    } else {
        None
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
    println!("Part 1: {}", sum);

    // Read the input from `data/input.txt` 3 lines at a time, and pass it to the `get_badge`
    // function to find the common item ("badge"). For each badge, calculate the priority and
    // accumulate the sum of the results.
    let sum: u32 = std::fs::read_to_string("data/input.txt")
        .expect("Failed to read input file")
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|chunk| get_badge(chunk).expect("No badge found"))
        .map(priority)
        .sum();
    println!("Part 2: {}", sum);
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

    #[test]
    fn test_get_badges() {
        // Collect the first 3 lines of the test data into a vector, and get those lines' badge.
        let rucksacks: Vec<&str> = TEST_DATA
            .iter()
            .map(|(rucksack, _)| *rucksack)
            .take(3)
            .collect();
        assert_eq!(get_badge(&rucksacks), Some('r'));
        // Do the same for the next 3 lines.
        let rucksacks: Vec<&str> = TEST_DATA
            .iter()
            .map(|(rucksack, _)| *rucksack)
            .skip(3)
            .take(3)
            .collect();
        assert_eq!(get_badge(&rucksacks), Some('Z'));
    }
}
