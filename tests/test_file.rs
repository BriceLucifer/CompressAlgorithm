use std::fs::{self, File};
use std::io::{Read, Write};
use huffman::{compress_file,decompress_file};

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_file() -> std::io::Result<()> {
        let input_data = "this is a test of huffman encoding!";
        let input_file = "test_input.txt";
        let compressed_file = "test_compressed.huff";
        let decompressed_file = "test_decompressed.txt";
        
        // 将输入数据写入文件
        let mut input = File::create(input_file)?;
        input.write_all(input_data.as_bytes())?;
        
        // 压缩文件
        compress_file(input_file, compressed_file)?;
        
        // 解压文件
        decompress_file(compressed_file, decompressed_file)?;
        
        // 读取解压后的文件内容
        let mut decompressed_data = String::new();
        let mut file = File::open(decompressed_file)?;
        file.read_to_string(&mut decompressed_data)?;
        
        // 验证解压后的内容是否与原始输入数据一致
        assert_eq!(decompressed_data, input_data);
        
        // 清理测试文件
        fs::remove_file(input_file)?;
        fs::remove_file(compressed_file)?;
        fs::remove_file(decompressed_file)?;

        Ok(())
    }

    #[test]
    fn test_compress_and_decompress() -> std::io::Result<()> {
        let input_data = "tasdfasdfsadfsadfsadfsdfasdfasdf;;!";
        let input_file = "test_input2.txt";
        let compressed_file = "test_compressed2.huff";
        let decompressed_file = "test_decompressed2.txt";
        
        // 将输入数据写入文件
        let mut input = File::create(input_file)?;
        input.write_all(input_data.as_bytes())?;
        
        // 压缩文件
        compress_file(input_file, compressed_file)?;
        
        // 解压文件
        decompress_file(compressed_file, decompressed_file)?;
        
        // 读取解压后的文件内容
        let mut decompressed_data = String::new();
        let mut file = File::open(decompressed_file)?;
        file.read_to_string(&mut decompressed_data)?;
        
        // 验证解压后的内容是否与原始输入数据一致
        assert_eq!(decompressed_data, input_data);
        
        // 清理测试文件
        fs::remove_file(input_file)?;
        fs::remove_file(compressed_file)?;
        fs::remove_file(decompressed_file)?;

        Ok(())
    }
}
