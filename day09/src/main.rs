// Advent of Code 2022 - Day 9
// A pet on a leash in NetHack

use std::collections::HashSet;

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

    fn step(&mut self, dir: char) {
        let (x, y) = self.head;
        let (new_head, new_tail) = match dir {
            'U' => {
                let new_head = (x, y + 1);
                let mut new_tail = self.tail;
                if (new_head.1 - new_tail.1) > 1 {
                    new_tail = match new_head.0 {
                        x if x == self.tail.0 => (self.tail.0, self.tail.1 + 1),
                        x if x > self.tail.0 => (self.tail.0 + 1, self.tail.1 + 1),
                        _ => (self.tail.0 - 1, self.tail.1 + 1),
                    };
                }
                (new_head, new_tail)
            }
            'D' => {
                let new_head = (x, y - 1);
                let mut new_tail = self.tail;
                if (new_tail.1 - new_head.1) > 1 {
                    if new_head.0 == self.tail.0 {
                        new_tail = (self.tail.0, self.tail.1 - 1);
                    } else if new_head.0 > self.tail.0 {
                        new_tail = (self.tail.0 + 1, self.tail.1 - 1);
                    } else {
                        new_tail = (self.tail.0 - 1, self.tail.1 - 1);
                    }
                }
                (new_head, new_tail)
            }
            'R' => {
                let new_head = (x + 1, y);
                let mut new_tail = self.tail;
                if (new_head.0 - new_tail.0) > 1 {
                    if new_head.1 == self.tail.1 {
                        new_tail = (self.tail.0 + 1, self.tail.1);
                    } else if new_head.1 > self.tail.1 {
                        new_tail = (self.tail.0 + 1, self.tail.1 + 1);
                    } else {
                        new_tail = (self.tail.0 + 1, self.tail.1 - 1);
                    }
                }
                (new_head, new_tail)
            }
            'L' => {
                let new_head = (x - 1, y);
                let mut new_tail = self.tail;
                if (new_tail.0 - new_head.0) > 1 {
                    if new_head.1 == self.tail.1 {
                        new_tail = (self.tail.0 - 1, self.tail.1);
                    } else if new_head.1 > self.tail.1 {
                        new_tail = (self.tail.0 - 1, self.tail.1 + 1);
                    } else {
                        new_tail = (self.tail.0 - 1, self.tail.1 - 1);
                    }
                }
                (new_head, new_tail)
            }
            _ => panic!("Invalid direction"),
        };

        self.head = new_head;
        self.tail = new_tail;
    }

    fn nstep(&mut self, n: usize, dir: char) {
        for _ in 0..n {
            self.step(dir);
        }
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut state = State::new();
        assert_eq!(state.head, (0, 0));
        assert_eq!(state.tail, (0, 0));
        state.step('R');
        assert_eq!(state.head, (1, 0));
        assert_eq!(state.tail, (0, 0));
        state.step('R');
        assert_eq!(state.head, (2, 0));
        assert_eq!(state.tail, (1, 0));
        state.nstep(2, 'R');
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
}
