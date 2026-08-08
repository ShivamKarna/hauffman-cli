use crate::error::HauffmanError;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, Eq, PartialEq)]
pub struct Node {
    pub ch: Option<char>,
    pub freq: usize,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

impl Node {
    fn leaf(char: Option<char>, freq: usize) -> Self {
        Node {
            ch: char,
            freq,
            left: None,
            right: None,
        }
    }

    fn internal(freq: usize, left: Node, right: Node) -> Self {
        Node {
            ch: None,
            freq,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        }
    }
}

impl Ord for Node {
    fn cmp(self: &Self, other: &Self) -> Ordering {
        other.freq.cmp(&self.freq)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(self: &Self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

pub fn count_frequencies(text: &str) -> HashMap<char, usize> {
    let mut freqs = HashMap::new();
    for ch in text.chars() {
        *freqs.entry(ch).or_insert(0) += 1;
    }
    freqs
}

pub fn build_tree(freqs: &HashMap<char, usize>) -> Option<Node> {
    let mut heap = BinaryHeap::new();
    for (&char, &freq) in freqs {
        heap.push(Node::leaf(Some(char), freq));
    }
    if heap.is_empty() {
        return None;
    }
    while heap.len() > 1 {
        let left_node = heap.pop().expect("Heap must contain atleast 2 nodes.");
        let right_node = heap.pop().expect("Heap must contain atleast 2 nodes.");
        let parent_freq = left_node.freq + right_node.freq;
        let parent = Node::internal(parent_freq, left_node, right_node);
        heap.push(parent);
    }
    heap.pop()
}

pub fn generate_code(root: &Node) -> HashMap<char, String> {
    let mut codes = HashMap::new();
    let mut stack = Vec::new();
    stack.push((root, String::new()));
    while !stack.is_empty() {
        let (current_node, prefix) = stack.pop().unwrap();
        // Leaf Node
        if let Some(ch) = current_node.ch {
            if prefix.is_empty() {
                codes.insert(ch, "0".to_string());
            } else {
                codes.insert(ch, prefix);
            }
            continue;
        }
        // Right -> 1
        if let Some(ref right) = current_node.right {
            let mut right_prefix = prefix.clone();
            right_prefix.push('1');
            stack.push((right, right_prefix));
        }
        // Left -> 0
        if let Some(ref left) = current_node.left {
            let mut left_prefix = prefix.clone();
            left_prefix.push('0');
            stack.push((left, left_prefix));
        }
    }
    codes
}

pub fn read_file(path: &str) -> Result<String, HauffmanError> {
    let content = std::fs::read_to_string(path)?;

    if content.is_empty() {
        return Err(HauffmanError::EmptyFile);
    }

    Ok(content)
}
