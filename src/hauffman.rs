use crate::error::HauffmanError;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fs::{self, File};
use std::io::Write;

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
        other
            .freq
            .cmp(&self.freq)
            // if frequencies are equal then compare their ascii values , i.e. cmp them alphabetically
            .then_with(|| self.ch.cmp(&other.ch))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(self: &Self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

pub fn count_frequencies(text: &str) -> HashMap<char, usize> {
    let mut char_counts = HashMap::new();
    for ch in text.chars() {
        *char_counts.entry(ch).or_insert(0) += 1;
    }
    char_counts
}

pub fn build_tree(char_counts: &HashMap<char, usize>) -> Option<Node> {
    if char_counts.is_empty() {
        return None;
    }

    let mut sorted_char_counts: Vec<(char, usize)> = char_counts
        .iter()
        .map(|(&character, &count)| (character, count))
        .collect();
    sorted_char_counts.sort_by(|a, b| a.1.cmp(&b.1).then_with(|| a.0.cmp(&b.0)));
    // then_with() only runs if sort_by() returns Ordeing::Equal

    let mut heap = BinaryHeap::new();

    for (ch, freq) in sorted_char_counts {
        heap.push(Node::leaf(Some(ch), freq));
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

pub fn encode_to_file(content: &str, output_path: &str) -> Result<(), HauffmanError> {
    let char_count = count_frequencies(content);

    let root = match build_tree(&char_count) {
        Some(r) => r,
        None => return Err(HauffmanError::EmptyFile),
    };

    let codes = generate_code(&root);

    let mut bits_string = String::new();

    for ch in content.chars() {
        bits_string.push_str(&codes[&ch]);
    }

    let mut file = File::create(output_path)?;

    for (ch, freq) in &char_count {
        let display_ch = match ch {
            '\n' => "\\n".to_string(),
            '\r' => "\\r".to_string(),
            _ => ch.to_string(),
        };

        writeln!(file, "{}:{}", display_ch, freq)?;
    }
    writeln!(file, "---------------")?;
    // char : freq and below it will be the bits_string, so in between them this will act as a seperator.

    writeln!(file, "{}", bits_string)?;

    Ok(())
}

// To decode,
// 1. build a hashmap from the given header
// 2. iter over and decode the given encoded string seperated by the seperator
// 3. write to file

fn parse_header(header_str: &str) -> Result<HashMap<char, usize>, HauffmanError> {
    let mut char_counts = HashMap::new();

    for line in header_str.lines() {
        if let Some((char_part, count_part)) = line.rsplit_once(':') {
            if let (Some(ch), Ok(count)) = (char_part.chars().next(), count_part.parse::<usize>()) {
                char_counts.insert(ch, count);
            }
        }
    }

    Ok(char_counts)
}

fn decode_string(bit_string: &str, root: &Node) -> Result<String, HauffmanError> {
    let mut decoded = String::new();
    let mut curr = root;

    for ch in bit_string.trim().chars() {
        match ch {
            '0' => {
                if let Some(ref left) = curr.left {
                    curr = left;
                }
            }
            '1' => {
                if let Some(ref right) = curr.right {
                    curr = right;
                }
            }
            _ => {
                continue;
            }
        }

        if let Some(leaf_ch) = curr.ch {
            decoded.push(leaf_ch);
            curr = root;
        }
    }

    Ok(decoded)
}

pub fn decode_to_file(input_path: &str, output_path: &str) -> Result<(), HauffmanError> {
    let raw_content = fs::read_to_string(input_path)?;

    let parts: Vec<&str> = raw_content.split("---------------").collect();

    if parts.len() < 2 {
        return Err(HauffmanError::InvalidHeader);
    }

    // parse header and build the tree
    let char_counts = parse_header(parts[0])?;
    let root = build_tree(&char_counts).ok_or(HauffmanError::CorruptedData)?;

    // decode content and write output
    let original_text = decode_string(parts[1], &root)?;
    let mut file = File::create(output_path)?;
    file.write_all(original_text.as_bytes())?;

    Ok(())
}
