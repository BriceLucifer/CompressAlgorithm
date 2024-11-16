use huffman::{compress_file, decompress_file, ui};
use std::fs;
use std::time::Instant;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    ui()
}

/// 比较两个文件的内容是否相同
fn compare_files(file1: &str, file2: &str) -> io::Result<bool> {
    let mut f1 = fs::File::open(file1)?;
    let mut f2 = fs::File::open(file2)?;

    let mut buf1 = Vec::new();
    let mut buf2 = Vec::new();

    f1.read_to_end(&mut buf1)?;
    f2.read_to_end(&mut buf2)?;

    Ok(buf1 == buf2)
}

pub fn compress_test() -> io::Result<()>{
    // 原文件和压缩文件路径
    let input_file = "./example/example.txt";
    let compressed_file = "./example/compressed.huff";
    let decompressed_file = "./example/decompressed.txt";

    // 获取原始文件大小（单位：字节），转换为 MB
    let original_size = fs::metadata(input_file)?.len() as f64 / (1024.0 * 1024.0); // 转换为 MB

    // 记录压缩开始时间
    let compress_start = Instant::now();
    compress_file(input_file, compressed_file)?; // 直接调用你已经实现的压缩函数
    let compress_duration = compress_start.elapsed();
    let compress_speed = (original_size * 1024.0 * 1024.0 / compress_duration.as_secs_f64()) / (1024.0 * 1024.0); // 转换为 MB/s

    // 记录解压开始时间
    let decompress_start = Instant::now();
    decompress_file(compressed_file, decompressed_file)?; // 直接调用你已经实现的解压函数
    let decompress_duration = decompress_start.elapsed();
    let decompress_speed = (original_size * 1024.0 * 1024.0 / decompress_duration.as_secs_f64()) / (1024.0 * 1024.0); // 转换为 MB/s

    // 获取文件大小，计算压缩率（单位：MB）
    let compressed_size = fs::metadata(compressed_file)?.len() as f64 / (1024.0 * 1024.0); // 转换为 MB
    let decompressed_size = fs::metadata(decompressed_file)?.len() as f64 / (1024.0 * 1024.0); // 转换为 MB

    let compression_ratio = compressed_size / original_size * 100.0;

    // 打印结果
    println!("Compression time: {:.2?}", compress_duration);
    println!("Decompression time: {:.2?}", decompress_duration);
    println!("Original file size: {:.2} MB", original_size);
    println!("Compressed file size: {:.2} MB", compressed_size);
    println!("Decompressed file size: {:.2} MB", decompressed_size);
    println!("Compression ratio: {:.2}%", compression_ratio);

    // 打印压缩和解压速度（以 MB/s 为单位）
    println!("Compression speed: {:.2} MB/s", compress_speed);
    println!("Decompression speed: {:.2} MB/s", decompress_speed);

    // 对比原文件和解压文件的内容
    if compare_files(input_file, decompressed_file)? {
        println!("The decompressed file is identical to the original.");
    } else {
        println!("The decompressed file is different from the original.");
    }

    Ok(())
}