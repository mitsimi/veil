//! Veil - A PNG steganography library
//! 
//! This library provides functionality for encoding and decoding hidden messages
//! in PNG files using custom chunks.

pub mod png;
pub mod cmd;

pub use png::{Chunk, ChunkType, Png};
pub use cmd::{Cli, Commands};

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

/// Encode a message into a PNG file
pub fn encode_message(file_path: &str, chunk_type: &str, message: &str, output_path: Option<&str>) -> Result<()> {
    use std::str::FromStr;
    
    let mut png = Png::from_file(file_path)?;
    let chunk_type = ChunkType::from_str(chunk_type)?;
    let chunk = Chunk::new(chunk_type, message.as_bytes().to_vec());
    png.append_chunk(chunk);
    
    let output = output_path.unwrap_or(file_path);
    png.to_file(output)?;
    Ok(())
}

/// Decode a message from a PNG file
pub fn decode_message(file_path: &str, chunk_type: &str) -> Result<String> {
    let png = Png::from_file(file_path)?;
    let chunk = png.chunk_by_type(chunk_type).ok_or("Chunk not found")?;
    let message = String::from_utf8(chunk.data().to_vec())?;
    Ok(message)
}

/// Remove a chunk from a PNG file
pub fn remove_chunk(file_path: &str, chunk_type: &str) -> Result<()> {
    let mut png = Png::from_file(file_path)?;
    png.remove_first_chunk(chunk_type)?;
    png.to_file(file_path)?;
    Ok(())
}

/// Print PNG file information
pub fn print_png_info(file_path: &str) -> Result<()> {
    let png = Png::from_file(file_path)?;
    println!("{}", png);
    Ok(())
} 