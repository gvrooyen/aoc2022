// Advent of Code 2022 - Day 7
// kdirstat

// TODO: The current implementation assumes unique node names. Refactor so that the main data
// structure is a proper tree.

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
    // The name of the node itself, without the full path
    name: String,
    parent: Option<String>,
    children: Vec<String>,
}

impl Node {
    fn root() -> Node {
        Node {
            size: 0,
            node_type: NodeType::Dir,
            parent: None,
            name: String::from("/"),
            children: Vec::new(),
        }
    }
}

struct Context {
    // Nodes are indexed by their full path
    nodes: HashMap<String, Node>,
    // The full path of the current node, e.g. /a/d/d.log
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
            name: name.to_string(),
            node_type,
            parent: Some((*self.pwd).to_string()),
            children: Vec::new(),
        };
        self.nodes.insert(format!("{}/{}", self.pwd, name), node);
        let parent_node_key = Some(self.pwd.clone());
        let parent_node = self.nodes.get_mut(&parent_node_key.unwrap()).unwrap();
        parent_node.children.push(name.to_string());
    }

    fn parent(&mut self) -> Option<String> {
        self.nodes.get(&self.pwd).unwrap().parent.clone()
    }

    // Caculate the total size of a node, by adding its own size to the size of its children and
    // theirs, recursively.
    fn node_size(&self, pwd: &str) -> u32 {
        println!("node_size: {}", pwd);
        let node = self.nodes.get(pwd).unwrap();
        let mut size = node.size;
        for child in &node.children {
            size += self.node_size(format!("{}/{}", pwd, child).as_str());
        }
        size
    }

    // Calculate the total size of all directories of at most the given size.
    fn total_size(&self, size: u32) -> u32 {
        let mut total = 0;
        for (name, node) in &self.nodes {
            if node.node_type == NodeType::Dir && self.node_size(name.as_str()) <= size {
                total += self.node_size(name.as_str());
            }
        }
        total
    }

    fn parse_line(&mut self, line: &str) {
        let mut w = line.split_whitespace();
        match w.next() {
            Some("$") => match w.next() {
                Some("cd") => {
                    // Change directory.
                    match w.next() {
                        Some("..") => self.pwd = self.parent().unwrap(),
                        Some("/") => self.pwd = String::from("/"),
                        Some(name) => self.pwd = format!("{}/{}", self.pwd, name.to_string()),
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
            None => {
                !panic!("Unknown file type");
            }
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
    println!("{:?}", context.total_size(100000));
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
        assert_eq!(context.nodes["//a"].size, 0);
        context.parse_line("14848514 b.txt");
        assert_eq!(context.nodes["//b.txt"].size, 14848514);
        context.parse_line("8504156 c.dat");
        assert_eq!(context.nodes["//c.dat"].size, 8504156);
        context.parse_line("dir d");
        assert_eq!(context.nodes["//d"].size, 0);
        context.parse_line("$ cd a");
        assert_eq!(context.pwd, "//a");
        assert_eq!(context.nodes["//a"].parent, Some("/".to_string()));
        context.parse_line("$ cd ..");
        assert_eq!(context.pwd, "/");
    }

    #[test]
    fn test_parse_input() {
        let context = parse_input("data/test.txt");
        assert_eq!(context.node_size("//a/e"), 584);
        assert_eq!(context.node_size("//a"), 94853);
        assert_eq!(context.node_size("//d"), 24933642);
        assert_eq!(context.node_size("/"), 48381165);
    }

    #[test]
    fn test_total_size() {
        let context = parse_input("data/test.txt");
        assert_eq!(context.total_size(100000), 95437);
    }
}
