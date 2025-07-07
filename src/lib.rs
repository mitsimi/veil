//! Veil - A steganography library
//!
//! This library provides functionality for hiding and extracting data
//! in various file formats using custom chunks with automatic detection.

use std::str::FromStr;

pub mod cmd;
pub mod png;

pub use cmd::{Cli, Commands};

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

/// Core trait for steganography operations across different file formats
pub trait Steganography {
    /// Hide data within this file format
    fn hide_data(&mut self, data: &[u8]) -> Result<()>;

    /// Extract all hidden data from this file
    fn extract_data(&self) -> Result<Vec<u8>>;

    /// Check if this file contains any hidden data
    fn has_hidden_data(&self) -> bool;

    /// Save the file (with any modifications) to the specified path
    fn save_to_file<P: AsRef<std::path::Path>>(&self, path: P) -> Result<()>;
}

/// Enum representing different file formats that support steganography
#[derive(Debug)]
pub enum SteganographyFile {
    Png(png::Png),
    // Future formats will be added here:
    // Jpeg(jpeg::Jpeg),
    // Pdf(pdf::Pdf),
}

impl SteganographyFile {
    /// Detect file format and load the appropriate type from a file path
    pub fn from_file<P: AsRef<std::path::Path>>(path: P) -> Result<Self> {
        let path_ref = path.as_ref();

        // TODO: examine file headers for more robust detection
        if let Some(extension) = path_ref.extension() {
            match extension.to_string_lossy().to_lowercase().as_str() {
                "png" => {
                    let png = png::Png::from_file(path)?;
                    Ok(SteganographyFile::Png(png))
                }
                _ => Err(format!("Unsupported file format: {:?}", extension).into()),
            }
        } else {
            Err("Could not determine file format - no extension found".into())
        }
    }
}

impl Steganography for SteganographyFile {
    fn hide_data(&mut self, data: &[u8]) -> Result<()> {
        match self {
            SteganographyFile::Png(png) => {
                let chunk_type = png::ChunkType::from_str("vEiL")?;
                let chunk = png::Chunk::new(chunk_type, data.to_vec());

                // Add the chunk to the PNG
                png.append_chunk(chunk);
                Ok(())
            }
        }
    }

    fn extract_data(&self) -> Result<Vec<u8>> {
        match self {
            SteganographyFile::Png(png) => {
                // Look for our custom "vEiL" chunks and extract their data
                let custom_chunks = png.custom_chunks();

                // Filter for our specific chunk type
                let veil_chunks: Vec<_> = custom_chunks
                    .into_iter()
                    .filter(|chunk| chunk.chunk_type().to_string() == "vEiL")
                    .collect();

                if veil_chunks.is_empty() {
                    return Err("No hidden data found".into());
                }

                // For now, extract data from the first vEiL chunk
                // In the future, we could concatenate multiple chunks
                let data = veil_chunks[0].data().to_vec();
                Ok(data)
            }
        }
    }

    fn has_hidden_data(&self) -> bool {
        match self {
            SteganographyFile::Png(png) => {
                // Check if there are any of our "vEiL" chunks specifically
                png.custom_chunks()
                    .iter()
                    .any(|chunk| chunk.chunk_type().to_string() == "vEiL")
            }
        }
    }

    fn save_to_file<P: AsRef<std::path::Path>>(&self, path: P) -> Result<()> {
        match self {
            SteganographyFile::Png(png) => png.to_file(path),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_steganography_end_to_end() {
        // Create a simple PNG with test chunks
        let chunks = vec![
            test_chunk("IHDR", b"fake header data"),
            test_chunk("IDAT", b"fake image data"),
            test_chunk("IEND", b""),
        ];
        let png = png::Png::from_chunks(chunks);
        let mut stego_file = SteganographyFile::Png(png);

        // Test data to hide
        let secret_message = b"This is a secret message!";

        // Initially should have no hidden data
        assert!(!stego_file.has_hidden_data());

        // Hide the data
        stego_file.hide_data(secret_message).unwrap();

        // Now should detect hidden data
        assert!(stego_file.has_hidden_data());

        // Extract and verify the data
        let extracted = stego_file.extract_data().unwrap();
        assert_eq!(extracted, secret_message);
    }

    #[test]
    fn test_no_hidden_data_error() {
        // Create a PNG without any hidden data
        let chunks = vec![test_chunk("IHDR", b"fake header data")];
        let png = png::Png::from_chunks(chunks);
        let stego_file = SteganographyFile::Png(png);

        // Should return error when trying to extract from file without hidden data
        assert!(stego_file.extract_data().is_err());
        assert!(!stego_file.has_hidden_data());
    }

    fn test_chunk(chunk_type: &str, data: &[u8]) -> png::Chunk {
        use std::str::FromStr;
        let chunk_type = png::ChunkType::from_str(chunk_type).unwrap();
        png::Chunk::new(chunk_type, data.to_vec())
    }
}
