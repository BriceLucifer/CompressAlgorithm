use std::collections::HashMap;
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
