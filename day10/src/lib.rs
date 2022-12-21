// Advent of Code 2022 CPU Library

use std::collections::VecDeque;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum OpCode {
    Noop,
    Addx(i32),
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OpCode::Noop => write!(f, "NOP"),
            OpCode::Addx(x) => write!(f, "ADDX {}", x),
        }
    }
}

fn arity(op: &OpCode) -> usize {
    match op {
        OpCode::Noop => 1,
        OpCode::Addx(_) => 2,
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct State {
    x: i32,
    t: usize,
    current_op: Option<OpCode>,
    op_dt: usize,
}

struct Cpu {
    program: VecDeque<OpCode>,
    state: State,
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "CPU[{}] => x: {}, op: {}, dt: {}",
            self.t,
            self.x,
            match self.current_op {
                Some(op) => op.to_string(),
                None => "None".to_string(),
            },
            self.op_dt
        )
    }
}

impl Cpu {
    fn new() -> Cpu {
        Cpu {
            state: State {
                x: 1,
                t: 0,
                current_op: None,
                op_dt: 0,
            },
            program: VecDeque::new(),
        }
    }

    fn load(&mut self, filename: &str) {
        let reader = BufReader::new(File::open(filename).expect("Could not open file"));
        let lines = reader.lines();

        for line in lines {
            let line = line.unwrap();
            let params = line.split_whitespace().collect::<Vec<&str>>();
            let op = match params[0] {
                "noop" => OpCode::Noop,
                "addx" => OpCode::Addx(params[1].parse::<i32>().unwrap()),
                _ => panic!("Unknown opcode"),
            };
            self.program.push_back(op);
        }
    }

    fn tick(&mut self) -> State {
        // Increase the clock counter
        self.state.t += 1;

        // At the start of the clock cycle, if the current operation is None, then
        // we need to fetch the next operation from the program queue.
        if self.state.current_op.is_none() {
            self.state.current_op = self.program.pop_front();

            // An operation with arity 1 will complete at the end of this cycle.
            self.state.op_dt = arity(&self.state.current_op.unwrap());
        }

        // Reduce the time to completion of the current operation
        self.state.op_dt -= 1;

        // The value of the register(s) during the current clock cycle
        let state = self.state.clone();

        // At the end of the cycle, execute the current operation if `dt` has reached zero. This
        // resets the current operation to `None`.
        if self.state.op_dt == 0 {
            match &self.state.current_op {
                Some(op) => match op {
                    OpCode::Noop => {}
                    OpCode::Addx(n) => {
                        self.state.x += n;
                    }
                },
                None => {}
            }
            self.state.current_op = None;
        }

        state
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load() {
        let mut cpu = Cpu::new();
        cpu.load("data/test.txt");
        assert_eq!(cpu.program.len(), 146);
        assert_eq!(cpu.program[0], OpCode::Addx(15));
        assert_eq!(cpu.program[1], OpCode::Addx(-11));
        assert_eq!(cpu.program[9], OpCode::Noop);
    }

    #[test]
    fn test_tick() {
        let mut cpu = Cpu::new();
        cpu.program.push_back(OpCode::Noop);
        cpu.program.push_back(OpCode::Addx(3));
        cpu.program.push_back(OpCode::Addx(-5));
        cpu.program.push_back(OpCode::Noop);
        let state = cpu.tick();
        assert_eq!(state.t, 1);
        assert_eq!(state.x, 1);
        println!("{:?}", state);
        let state = cpu.tick();
        assert_eq!(state.t, 2);
        assert_eq!(state.x, 1);
        println!("{:?}", state);
        let state = cpu.tick();
        assert_eq!(state.t, 3);
        assert_eq!(state.x, 1);
        println!("{:?}", state);
        let state = cpu.tick();
        assert_eq!(state.t, 4);
        assert_eq!(state.x, 4);
        println!("{:?}", state);
        let state = cpu.tick();
        assert_eq!(state.t, 5);
        assert_eq!(state.x, 4);
        println!("{:?}", state);
        let state = cpu.tick();
        assert_eq!(state.t, 6);
        assert_eq!(state.x, -1);
    }
}
