// Advent of Code 2022 - Day 9
// A pet on a leash in NetHack

use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct State {
    head: (i32, i32),
    tail: (i32, i32),
    tail_visits: HashSet<(i32, i32)>,
}

impl State {
    fn new() -> Self {
        let mut s = State {
            head: (0, 0),
            tail: (0, 0),
            tail_visits: HashSet::new(),
        };
        s.tail_visits.insert((0, 0));
        s
    }
    fn pull_tail(&mut self) {
        let dx = self.head.0 - self.tail.0;
        let dy = self.head.1 - self.tail.1;

        // If the tail touches the head, no processing is needed.
        if (dx.abs() <= 1) && (dy.abs() <= 1) {
            return;
        }

        // We have eight cases to handle: the four quadrants in which the tail can be, and the four
        // axes. We start with the head directly north of the tail, and cover the cases clockwise.
        if (dx == 0) && (dy > 0) {
            self.tail.1 += 1;
        } else if (dx > 0) && (dy > 0) {
            self.tail.0 += 1;
            self.tail.1 += 1;
        } else if (dx > 0) && (dy == 0) {
            self.tail.0 += 1;
        } else if (dx > 0) && (dy < 0) {
            self.tail.0 += 1;
            self.tail.1 -= 1;
        } else if (dx == 0) && (dy < 0) {
            self.tail.1 -= 1;
        } else if (dx < 0) && (dy < 0) {
            self.tail.0 -= 1;
            self.tail.1 -= 1;
        } else if (dx < 0) && (dy == 0) {
            self.tail.0 -= 1;
        } else if (dx < 0) && (dy > 0) {
            self.tail.0 -= 1;
            self.tail.1 += 1;
        } else {
            // The tail is below the head; do nothing.
        }
        self.tail_visits.insert(self.tail);
    }

    fn step(&mut self, dir: char) {
        let (x, y) = self.head;
        match dir {
            'U' => {
                self.head = (x, y + 1);
            }
            'D' => {
                self.head = (x, y - 1);
            }
            'R' => {
                self.head = (x + 1, y);
            }
            'L' => {
                self.head = (x - 1, y);
            }
            _ => panic!("Invalid direction"),
        };

        self.pull_tail();
    }

    fn nstep(&mut self, n: usize, dir: char) {
        for _ in 0..n {
            self.step(dir);
        }
    }

    fn process(&mut self, filename: &str) {
        let reader = BufReader::new(File::open(filename).expect("Could not open file"));
        let lines = reader.lines();

        // For each line, read a direction character and the number of steps to take.
        for line in lines {
            let l = line.unwrap();
            let mut c = l.chars();
            let dir = c.next().unwrap();
            let n = c.skip(1).collect::<String>().parse::<usize>().unwrap();
            self.nstep(n, dir);
        }
    }
}

struct NKnots {
    knots: Vec<State>,
}

impl NKnots {
    fn new(n: usize) -> Self {
        let mut s = NKnots { knots: Vec::new() };
        for _ in 0..n {
            s.knots.push(State::new());
        }
        s
    }

    fn step(&mut self, dir: char) {
        self.knots[0].step(dir);
        for k in 1..self.knots.len() {
            self.knots[k].head = self.knots[k - 1].tail;
            self.knots[k].pull_tail();
        }
    }

    fn nstep(&mut self, n: usize, dir: char) {
        for _ in 0..n {
            self.step(dir);
        }
    }

    fn process(&mut self, filename: &str) {
        let reader = BufReader::new(File::open(filename).expect("Could not open file"));
        let lines = reader.lines();

        // For each line, read a direction character and the number of steps to take.
        for line in lines {
            let l = line.unwrap();
            let mut c = l.chars();
            let dir = c.next().unwrap();
            let n = c.skip(1).collect::<String>().parse::<usize>().unwrap();
            self.nstep(n, dir);
        }
    }
}

fn main() {
    let mut state = State::new();
    state.process("data/input.txt");
    println!("Tail visits: {}", state.tail_visits.len());

    let mut rope = NKnots::new(10);
    rope.process("data/input.txt");
    println!("Rope tail visits: {}", rope.knots[8].tail_visits.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step() {
        let mut state = State::new();
        assert_eq!(state.head, (0, 0));
        assert_eq!(state.tail, (0, 0));
        state.step('R');
        assert_eq!(state.head, (1, 0));
        assert_eq!(state.tail, (0, 0));
        state.step('R');
        assert_eq!(state.head, (2, 0));
        assert_eq!(state.tail, (1, 0));
        state.step('R');
        assert_eq!(state.head, (3, 0));
        assert_eq!(state.tail, (2, 0));
        state.step('R');
        assert_eq!(state.head, (4, 0));
        assert_eq!(state.tail, (3, 0));
        state.step('U');
        assert_eq!(state.head, (4, 1));
        assert_eq!(state.tail, (3, 0));
        state.nstep(3, 'U');
        assert_eq!(state.head, (4, 4));
        assert_eq!(state.tail, (4, 3));
        state.step('L');
        assert_eq!(state.head, (3, 4));
        assert_eq!(state.tail, (4, 3));
        state.step('L');
        assert_eq!(state.head, (2, 4));
        assert_eq!(state.tail, (3, 4));
        state.step('L');
        assert_eq!(state.head, (1, 4));
        assert_eq!(state.tail, (2, 4));
        state.step('D');
        assert_eq!(state.head, (1, 3));
        assert_eq!(state.tail, (2, 4));
        state.nstep(4, 'R');
        assert_eq!(state.head, (5, 3));
        assert_eq!(state.tail, (4, 3));
        state.step('D');
        assert_eq!(state.head, (5, 2));
        assert_eq!(state.tail, (4, 3));
        state.nstep(5, 'L');
        assert_eq!(state.head, (0, 2));
        assert_eq!(state.tail, (1, 2));
        state.step('R');
        assert_eq!(state.head, (1, 2));
        assert_eq!(state.tail, (1, 2));
        state.step('R');
        assert_eq!(state.head, (2, 2));
        assert_eq!(state.tail, (1, 2));
    }

    #[test]
    fn test_process_1() {
        let mut state = State::new();
        state.process("data/test.txt");
        assert_eq!(state.tail_visits.len(), 13);
    }

    #[test]
    fn test_nknots() {
        let mut rope = NKnots::new(10);
        assert_eq!(rope.knots.len(), 10);
        assert_eq!(rope.knots[0].head, (0, 0));
        assert_eq!(rope.knots[0].tail, (0, 0));
        assert_eq!(rope.knots[9].head, (0, 0));
        assert_eq!(rope.knots[9].tail, (0, 0));
        rope.step('R');
        assert_eq!(rope.knots[0].head, (1, 0));
        assert_eq!(rope.knots[0].tail, (0, 0));
        assert_eq!(rope.knots[1].head, (0, 0));
    }

    #[test]
    fn test_process_2() {
        let mut rope = NKnots::new(10);
        rope.process("data/test2.txt");
        assert_eq!(rope.knots[8].tail_visits.len(), 36);
    }
}
