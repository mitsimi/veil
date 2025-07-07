//! Veil - A steganography library
//! 
//! This library provides functionality for hiding and extracting data
//! in various file formats using custom chunks with automatic detection.

pub mod png;
pub mod cmd;
pub mod formats;

pub use png::{Chunk, ChunkType, Png, DataType, Content, HiddenData, ExtractedData};
pub use cmd::{Cli, Commands};
pub use formats::{FileFormat, SteganographyFormat};


pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

/// Automatically detect all hidden data in a file
pub fn auto_detect_hidden_data(file_path: &str) -> Result<Vec<HiddenData>> {
    let format = FileFormat::detect(file_path)?;
    format.auto_detect_hidden_data(file_path)
}

/// Extract all hidden data from a file
pub fn extract_all_hidden_data(file_path: &str) -> Result<Vec<ExtractedData>> {
    let format = FileFormat::detect(file_path)?;
    format.extract_all_hidden_data(file_path)
}

/// Get all custom chunks in a file
pub fn list_custom_chunks(file_path: &str) -> Result<Vec<String>> {
    let format = FileFormat::detect(file_path)?;
    format.list_custom_chunks(file_path)
}

/// Hide data in a file
pub fn hide_data(file_path: &str, data: Vec<u8>, chunk_type: ChunkType, output_path: &str) -> Result<()> {
    let format = FileFormat::detect(file_path)?;
    format.hide_data(file_path, data, chunk_type, output_path)
} 