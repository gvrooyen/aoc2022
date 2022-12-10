// Advent of Code 2022 - Day 7
// kdirstat

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

enum NodeType {
    Dir,
    File,
}

enum ModeType {
    Command,
    Ls,
}

struct Node {
    size: u32,
    node_type: NodeType,
    parent: Option<String>,
    children: HashMap<String, Node>,
}

impl Node {
    fn root() -> Node {
        Node {
            size: 0,
            node_type: NodeType::Dir,
            parent: None,
            children: HashMap::new(),
        }
    }
}

struct Context {
    nodes: HashMap<String, Node>,
    pwd: String,
    mode: ModeType,
}

impl Context {
    fn new() -> Context {
        Context {
            nodes: HashMap::new(),
            pwd: String::from("/"),
            mode: ModeType::Command,
        }
    }
}

fn parse_line(context: &mut Context, line: &str) {
    let mut w = line.split_whitespace();
    match w.next() {
        Some("$") => match w.next() {
            Some("cd") => {
                // Change directory.
                context.pwd = w.next().unwrap().to_string();
            }
            Some("ls") => {}
            Some(unknown) => {}
            None => {}
        },
        Some("dir") => {}
        Some(size) => {}
        None => {}
    }
}

fn parse_input(filename: &str) -> Context {
    let reader = BufReader::new(File::open(filename).expect("Could not open file"));
    let lines = reader.lines();
    let mut context = Context::new();

    for line in lines {
        let l = line.unwrap();
        parse_line(&mut context, &l);
    }

    context
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let mut context = Context::new();
        parse_line(&mut context, "$ cd /");
        assert_eq!(context.pwd, "/");
        parse_line(&mut context, "$ ls");
        parse_line(&mut context, "14848514 b.txt");
        assert_eq!(context.nodes["b.txt"].size, 14848514);
    }
}
