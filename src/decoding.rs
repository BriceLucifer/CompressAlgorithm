use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read, Write};
use serde_json;

pub fn decompress_file(input_file: &str, output_file: &str) -> io::Result<()> {
    let mut file = File::open(input_file)?;

    // 读取填充位数信息
    let mut padding_bits_buf = [0u8; 1];
    file.read_exact(&mut padding_bits_buf)?;
    let padding_bits = padding_bits_buf[0] as usize;

    let mut len_bytes = [0u8; 4];
    file.read_exact(&mut len_bytes)?;
    let codes_len = u32::from_be_bytes(len_bytes) as usize;

    let mut serialized_codes = vec![0u8; codes_len];
    file.read_exact(&mut serialized_codes)?;
    let serialized_codes_str = String::from_utf8(serialized_codes)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    let codes: HashMap<char, String> = serde_json::from_str(&serialized_codes_str)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    struct DecodingNode {
        char: Option<char>,
        left: Option<Box<DecodingNode>>,
        right: Option<Box<DecodingNode>>,
    }

    impl DecodingNode {
        fn new() -> Self {
            DecodingNode { char: None, left: None, right: None }
        }

        fn insert(&mut self, code: &str, character: char) {
            let mut current = self;
            for bit in code.chars() {
                match bit {
                    '0' => {
                        if current.left.is_none() {
                            current.left = Some(Box::new(DecodingNode::new()));
                        }
                        current = current.left.as_mut().unwrap();
                    },
                    '1' => {
                        if current.right.is_none() {
                            current.right = Some(Box::new(DecodingNode::new()));
                        }
                        current = current.right.as_mut().unwrap();
                    },
                    _ => panic!("Invalid bit in code: {}", bit),
                }
            }
            current.char = Some(character);
        }
    }

    let mut root = DecodingNode::new();
    for (ch, code) in &codes {
        root.insert(code, *ch);
    }

    let mut encoded_bytes = Vec::new();
    file.read_to_end(&mut encoded_bytes)?;

    let mut bit_string = String::new();
    for (i, byte) in encoded_bytes.iter().enumerate() {
        for j in (0..8).rev() {
            if i == encoded_bytes.len() - 1 && j < padding_bits {
                break; // 跳过最后一个字节的填充位
            }
            bit_string.push(if (byte >> j) & 1 == 1 { '1' } else { '0' });
        }
    }

    let mut decoded_chars = Vec::new();
    let mut current_node = &root;
    for bit in bit_string.chars() {
        current_node = match bit {
            '0' => current_node.left.as_deref().unwrap(),
            '1' => current_node.right.as_deref().unwrap(),
            _ => unreachable!(),
        };

        if let Some(ch) = current_node.char {
            decoded_chars.push(ch);
            current_node = &root;
        }
    }

    let decoded_string: String = decoded_chars.iter().collect();
    let mut output = File::create(output_file)?;
    output.write_all(decoded_string.as_bytes())?;

    Ok(())
}

