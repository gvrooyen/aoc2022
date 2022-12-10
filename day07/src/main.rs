// Advent of Code 2022 - Day 7
// kdirstat

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

enum NodeType {
    Dir,
    File,
}

#[derive(Debug, PartialEq)]
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
        let mut c = Context {
            nodes: HashMap::new(),
            pwd: String::from("/"),
            mode: ModeType::Command,
        };
        c.nodes.insert(String::from("/"), Node::root());

        c
    }

    fn add_node(&mut self, node_type: NodeType, name: &str, size: u32) {
        let node = Node {
            size,
            node_type,
            parent: Some((*self.pwd).to_string()),
            children: HashMap::new(),
        };
        self.nodes.insert(String::from(name), node);
    }

    fn parse_line(&mut self, line: &str) {
        let mut w = line.split_whitespace();
        match w.next() {
            Some("$") => match w.next() {
                Some("cd") => {
                    // Change directory.
                    self.pwd = w.next().unwrap().to_string();
                    self.mode = ModeType::Command;
                }
                Some("ls") => {
                    // List directory.
                    self.mode = ModeType::Ls;
                }
                Some(unknown) => {}
                None => {}
            },
            Some("dir") => {
                assert_eq!(self.mode, ModeType::Ls);
                let dir = w.next().unwrap();
                self.add_node(NodeType::Dir, dir, 0);
            }
            Some(size) => {
                assert_eq!(self.mode, ModeType::Ls);
                let size = size.parse::<u32>().unwrap();
                let file = w.next().unwrap();
                self.add_node(NodeType::File, file, size);
            }
            None => {}
        }
    }
}

fn parse_input(filename: &str) -> Context {
    let reader = BufReader::new(File::open(filename).expect("Could not open file"));
    let lines = reader.lines();
    let mut context = Context::new();

    for line in lines {
        let l = line.unwrap();
        context.parse_line(&l);
    }

    context
}

fn main() {
    let context = parse_input("data/input.txt");
    println!("{:?}", context.pwd);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let mut context = Context::new();
        assert_eq!(context.pwd, "/");
        assert_eq!(context.mode, ModeType::Command);
        assert_eq!(context.nodes.len(), 1);
        assert_eq!(context.nodes.contains_key("/"), true);
        context.parse_line("$ cd /");
        assert_eq!(context.pwd, "/");
        context.parse_line("$ ls");
        context.parse_line("14848514 b.txt");
        assert_eq!(context.nodes["b.txt"].size, 14848514);
        context.parse_line("8504156 c.dat");
        assert_eq!(context.nodes["c.dat"].size, 8504156);
        context.parse_line("dir d");
        assert_eq!(context.nodes["d"].size, 0);
        context.parse_line("$ cd a");
        assert_eq!(context.pwd, "a");
        context.parse_line("$ cd ..");
        assert_eq!(context.pwd, "/");
    }

    #[test]
    fn test_parse_input() {}
}
