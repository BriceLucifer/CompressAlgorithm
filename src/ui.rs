use clap::{Parser, Subcommand};
use std::{fs, io, path::PathBuf};

use crate::{compress_file, decompress_file};

#[derive(Parser)]
#[command(about, long_about = None)]
struct Cli {
    /// Compress a Huffman file
    #[arg(short, long, value_name = "FILE", conflicts_with("decompress"))]
    compress: Option<PathBuf>,

    /// Decompress a Huffman file
    #[arg(short, long, value_name = "FILE", conflicts_with("compress"))]
    decompress: Option<PathBuf>,

    /// Output file name
    #[arg(short, long, value_name = "FILE")]
    output: Option<PathBuf>,
}

pub fn ui() -> Result<(), io::Error> {
    let cli = Cli::parse();

    if let Some(compress_path) = &cli.compress {
        if let Some(output_path) = &cli.output {
            handle_compress(compress_path, output_path)?;
        } else {
            eprintln!("Error: Output file is required for compression.");
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Output file is required for compression",
            ));
        }
    }

    if let Some(decompress_path) = &cli.decompress {
        if let Some(output_path) = &cli.output {
            handle_decompress(decompress_path, output_path)?;
        } else {
            eprintln!("Error: Output file is required for decompression.");
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Output file is required for decompression",
            ));
        }
    }

    if cli.compress.is_none() && cli.decompress.is_none() {
        eprintln!("Error: Either --compress or --decompress must be specified.");
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Either --compress or --decompress must be specified",
        ));
    }

    Ok(())
}

fn handle_compress(compress_path: &PathBuf, output_path: &PathBuf) -> Result<(), io::Error> {
    println!("Compressing file: {}", compress_path.display());
    let compress_path_str = compress_path
        .to_str()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid compress path"))?;
    let output_path_str = output_path
        .to_str()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid output path"))?;
    compress_file(compress_path_str, output_path_str)?;
    let compression_ratio = compression_ratio(compress_path_str, output_path_str)?;
    println!("Compression ratio: {:.2}%", compression_ratio);
    Ok(())
}

fn handle_decompress(decompress_path: &PathBuf, output_path: &PathBuf) -> Result<(), io::Error> {
    println!("Decompressing file: {}", decompress_path.display());
    let decompress_path_str = decompress_path
        .to_str()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid decompress path"))?;
    let output_path_str = output_path
        .to_str()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid output path"))?;
    decompress_file(decompress_path_str, output_path_str)?;
    Ok(())
}

pub fn compression_ratio(original: &str, compressed: &str) -> Result<f64, io::Error> {
    let original_size = fs::metadata(original)?.len() as f64 / (1024.0 * 1024.0); // 转换为 MB
    let compressed_size = fs::metadata(compressed)?.len() as f64 / (1024.0 * 1024.0);

    let compression_ratio = compressed_size / original_size * 100.0;
    Ok(compression_ratio)
}
