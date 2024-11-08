use std::{io::stdin, process::exit};


fn main() -> std::io::Result<()> {
    let mut buffer = String::new();
    loop {
        stdin().read_line(&mut buffer)?;
        if buffer.trim() == "exit".to_string() {
            exit(0)
        }
        print!("Word: {}",&buffer);
        let (encoded_word, codes) = huffman::huffman_encoding(buffer.trim().chars().collect());
        println!("Huffman code: {}", encoded_word);
        println!("Conversion table: {:?}", codes);
        buffer.clear();
    }
}
