
fn main() {
    let word = "lossless".chars().collect::<Vec<char>>();
    let (encoded_word, codes) = huffman::huffman_encoding(word);

    println!("Word: lossless");
    println!("Huffman code: {}", encoded_word);
    println!("Conversion table: {:?}", codes);
}
