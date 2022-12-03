// Advent of Code 2022 - Day 2
// Calculate the score of a Rock Paper Scissors game

#[allow(clippy::identity_op)]
fn score1(opponent: char, player: char) -> u32 {
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

fn score2(opponent: char, player: char) -> u32 {
    match (opponent, player) {
        ('A', 'X') => 0 + 3,
        ('A', 'Y') => 3 + 1,
        ('A', 'Z') => 6 + 2,
        ('B', 'X') => 0 + 1,
        ('B', 'Y') => 3 + 2,
        ('B', 'Z') => 6 + 3,
        ('C', 'X') => 0 + 2,
        ('C', 'Y') => 3 + 3,
        ('C', 'Z') => 6 + 1,
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
            score2(opponent, player)
        })
        .sum::<u32>();
    println!("{}", total_score);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score() {
        let r1 = score1('A', 'Y');
        assert_eq!(r1, 8);
        let r2 = score1('B', 'X');
        assert_eq!(r2, 1);
        let r3 = score1('C', 'Z');
        assert_eq!(r3, 6);
        assert_eq!(r1 + r2 + r3, 15);
    }

    #[test]
    fn test_score2() {
        let r1 = score2('A', 'Y');
        assert_eq!(r1, 4);
        let r2 = score2('B', 'X');
        assert_eq!(r2, 1);
        let r3 = score2('C', 'Z');
        assert_eq!(r3, 7);
        assert_eq!(r1 + r2 + r3, 12);
    }
}
