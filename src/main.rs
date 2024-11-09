use huffman::{compress_file, decompress_file};

fn main() -> std::io::Result<()> {
    compress_file("example.txt", "compressed.huff")?;
    decompress_file("compressed.huff", "decompressed.txt")?;
    Ok(())
}
