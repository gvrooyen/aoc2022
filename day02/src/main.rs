// Advent of Code 2022 - Day 2
// Calculate the score of a Rock Paper Scissors game

#[allow(clippy::identity_op)]
fn score(opponent: char, player: char) -> u32 {
    match (opponent, player) {
        ('A', 'X') => 1 + 3,
        ('A', 'Y') => 2 + 6,
        ('A', 'Z') => 3 + 0,
        ('B', 'X') => 1 + 0,
        ('B', 'Y') => 2 + 3,
        ('B', 'Z') => 3 + 6,
        ('C', 'X') => 1 + 6,
        ('C', 'Y') => 2 + 0,
        ('C', 'Z') => 3 + 3,
        _ => 0,
    }
}

fn main() {
    // Read the input data from `data/input.txt`, and pass the plays to the `score` function
    // to accumulate the total score.
    let total_score = std::fs::read_to_string("data/input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let mut iter = line.split_whitespace();
            let opponent = iter.next().unwrap().chars().next().unwrap();
            let player = iter.next().unwrap().chars().next().unwrap();
            score(opponent, player)
        })
        .sum::<u32>();
    println!("{}", total_score);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score() {
        let r1 = score('A', 'Y');
        assert_eq!(r1, 8);
        let r2 = score('B', 'X');
        assert_eq!(r2, 1);
        let r3 = score('C', 'Z');
        assert_eq!(r3, 6);
        assert_eq!(r1 + r2 + r3, 15);
    }
}
