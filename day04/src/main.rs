// Advent of Code 2022 - Day 4
// Find overlapping work assignments.

// Find the assignment indices where one range in the pair is completely contained in the other.
fn find_subsets(assignments: &Vec<((u32, u32), (u32, u32))>) -> Vec<usize> {
    let mut subsets = Vec::new();
    for (i, ((a, b), (c, d))) in assignments.iter().enumerate() {
        if ((a >= c) && (b <= d)) || ((c >= a) && (d <= b)) {
            subsets.push(i);
        }
    }
    subsets
}

// Find the assignment indices where the ranges overlap at all.
fn find_overlaps(assignments: &Vec<((u32, u32), (u32, u32))>) -> Vec<usize> {
    let mut overlaps = Vec::new();
    for (i, ((a, b), (c, d))) in assignments.iter().enumerate() {
        if ((a >= c) && (a <= d))
            || ((b >= c) && (b <= d))
            || ((c >= a) && (c <= b))
            || ((d >= a) && (d <= b))
        {
            overlaps.push(i);
        }
    }
    overlaps
}

// Parse a string of the form "10-19,3-11" into a tuple of the form ((10, 19), (3, 11)).
fn parse_assignment(assignment: &str) -> ((u32, u32), (u32, u32)) {
    let assignment: Vec<&str> = assignment.split(',').collect();
    let first: Vec<&str> = assignment[0].split('-').collect();
    let second: Vec<&str> = assignment[1].split('-').collect();
    (
        (first[0].parse().unwrap(), first[1].parse().unwrap()),
        (second[0].parse().unwrap(), second[1].parse().unwrap()),
    )
}

fn main() {
    // Read the input file `data/input.txt` into an vector of tuple pairs.
    let assignments = std::fs::read_to_string("data/input.txt")
        .expect("Failed to read input file")
        .lines()
        .map(|line| parse_assignment(line))
        .collect::<Vec<((u32, u32), (u32, u32))>>();
    let subsets = find_subsets(&assignments);
    println!("Found {} subsets", subsets.len());
    let overlaps = find_overlaps(&assignments);
    println!("Found {} overlaps", overlaps.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &[((u32, u32), (u32, u32))] = &[
        ((2, 4), (6, 8)),
        ((2, 3), (4, 5)),
        ((5, 7), (7, 9)),
        ((2, 8), (3, 7)),
        ((6, 6), (4, 6)),
        ((2, 6), (4, 8)),
    ];

    #[test]
    fn test_find_subsets() {
        assert_eq!(find_subsets(&TEST_INPUT.to_vec()), vec![3, 4]);
    }

    #[test]
    fn test_parse_assignment() {
        assert_eq!(parse_assignment("2-14,16-108"), ((2, 14), (16, 108)));
    }
}
