// Advent of Code 2022 - Day 7
// kdirstat

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq)]
enum NodeType {
    Dir,
    File,
}

#[derive(Debug, PartialEq)]
enum ModeType {
    Command,
    Ls,
}

#[derive(Debug, PartialEq)]
struct Node {
    size: u32,
    node_type: NodeType,
    parent: Option<String>,
    children: Vec<String>,
}

impl Node {
    fn root() -> Node {
        Node {
            size: 0,
            node_type: NodeType::Dir,
            parent: None,
            children: Vec::new(),
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
            children: Vec::new(),
        };
        self.nodes.insert(String::from(name), node);
        let parent_node = self.nodes.get_mut(&self.pwd).unwrap();
        parent_node.children.push(String::from(name));
    }

    fn parent(&mut self) -> Option<String> {
        self.nodes.get(&self.pwd).unwrap().parent.clone()
    }

    // Caculate the total size of a node, by adding its own size to the size of its children and
    // theirs, recursively.
    fn node_size(&self, name: &str) -> u32 {
        let node = self.nodes.get(name).unwrap();
        let mut size = node.size;
        println!("{:?}", node.children);
        for child in &node.children {
            size += self.node_size(child.as_str());
        }
        size
    }

    fn parse_line(&mut self, line: &str) {
        let mut w = line.split_whitespace();
        match w.next() {
            Some("$") => match w.next() {
                Some("cd") => {
                    // Change directory.
                    match w.next() {
                        Some("..") => self.pwd = self.parent().unwrap(),
                        Some(name) => self.pwd = name.to_string(),
                        None => panic!("cd: missing operand"),
                    }
                    self.mode = ModeType::Command;
                }
                Some("ls") => {
                    // List directory.
                    self.mode = ModeType::Ls;
                }
                Some(unknown) => {
                    !panic!("Unknown command: {}", unknown);
                }
                None => {
                    !panic!("Empty command");
                }
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
        context.parse_line("dir a");
        assert_eq!(context.nodes["a"].size, 0);
        context.parse_line("14848514 b.txt");
        assert_eq!(context.nodes["b.txt"].size, 14848514);
        context.parse_line("8504156 c.dat");
        assert_eq!(context.nodes["c.dat"].size, 8504156);
        context.parse_line("dir d");
        assert_eq!(context.nodes["d"].size, 0);
        context.parse_line("$ cd a");
        assert_eq!(context.pwd, "a");
        assert_eq!(context.nodes["a"].parent, Some("/".to_string()));
        context.parse_line("$ cd ..");
        assert_eq!(context.pwd, "/");
    }

    #[test]
    fn test_parse_input() {
        let context = parse_input("data/test.txt");
        println!("{:?}", context.nodes);
        assert_eq!(context.node_size("e"), 584);
    }
}
