use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read, Write};
use std::rc::Rc;
use std::cell::RefCell;

use crate::node::Node;

pub fn calculate_frequencies(word: Vec<char>, nodes: &mut Vec<Rc<RefCell<Node>>>) {
    let mut frequencies = HashMap::new();

    for &c in &word {
        *frequencies.entry(c).or_insert(0) += 1;
    }

    for (ch, freq) in frequencies {
        nodes.push(Rc::new(RefCell::new(Node::new(Some(ch), freq))));
    }
}

pub fn build_huffman_tree(nodes: &mut Vec<Rc<RefCell<Node>>>) -> Rc<RefCell<Node>> {
    while nodes.len() > 1 {
        nodes.sort_by_key(|node| node.borrow().freq);

        // remove the node which has lowest frequency
        let left = nodes.remove(0);
        // left.freq < right.freq
        let right = nodes.remove(0);

        // parent freq
        let merged_freq = left.borrow().freq + right.borrow().freq;
        // parent Node
        let mut merged = Node::new(None, merged_freq);
        merged.left = Some(left.clone());
        merged.right = Some(right.clone());

        nodes.push(Rc::new(RefCell::new(merged)));
    }

    nodes[0].clone()
}

pub fn generate_huffman_codes(
    node: Option<Rc<RefCell<Node>>>,
    current_code: String,
    codes: &mut HashMap<char, String>,
) {
    if let Some(n) = node {
        let node_ref = n.borrow();
        if let Some(ch) = node_ref.char {
            codes.insert(ch, current_code);
        } else {
            generate_huffman_codes(node_ref.left.clone(), format!("{}0", current_code), codes);
            generate_huffman_codes(node_ref.right.clone(), format!("{}1", current_code), codes);
        }
    }
}

pub fn huffman_encoding(word: Vec<char>) -> (String, HashMap<char, String>) {
    let mut nodes = vec![];
    calculate_frequencies(word.clone(), &mut nodes);

    let root = build_huffman_tree(&mut nodes);
    let mut codes = HashMap::new();
    generate_huffman_codes(Some(root), String::new(), &mut codes);

    let encoded_word = word.iter().map(|c| codes[c].clone()).collect::<String>();

    (encoded_word, codes)
}

pub fn compress_file(input_file: &str, output_file: &str) -> io::Result<()> {
    let mut file = File::open(input_file)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let (encoded_data, codes) = huffman_encoding(contents.chars().collect());

    let mut encoded_bytes = Vec::new();
    let mut byte = 0u8;
    let mut bit_count = 0;

    for bit in encoded_data.chars() {
        byte <<= 1;
        if bit == '1' {
            byte |= 1;
        }
        bit_count += 1;

        if bit_count == 8 {
            encoded_bytes.push(byte);
            byte = 0;
            bit_count = 0;
        }
    }

    // 如果最后有未满8位的字节，记录填充的位数并添加到文件
    let padding_bits = if bit_count > 0 {
        byte <<= 8 - bit_count;
        encoded_bytes.push(byte);
        8 - bit_count
    } else {
        0
    };

    let mut output = File::create(output_file)?;

    // 写入填充位数信息（1字节）
    output.write_all(&[padding_bits as u8])?;

    let serialized_codes = serde_json::to_string(&codes)?;
    output.write_all(&(serialized_codes.len() as u32).to_be_bytes())?;
    output.write_all(serialized_codes.as_bytes())?;

    output.write_all(&encoded_bytes)?;

    Ok(())
}
