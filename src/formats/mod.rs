use crate::Result;
use crate::png::{Png, ChunkType, HiddenData, ExtractedData};
use std::path::Path;

/// Trait for different file formats that support steganography
pub trait SteganographyFormat {
    fn auto_detect_hidden_data(&self, file_path: &str) -> Result<Vec<HiddenData>>;
    fn extract_all_hidden_data(&self, file_path: &str) -> Result<Vec<ExtractedData>>;
    fn list_custom_chunks(&self, file_path: &str) -> Result<Vec<String>>;
    fn hide_data(&self, file_path: &str, data: Vec<u8>, chunk_type: ChunkType, output_path: &str) -> Result<()>;
}

/// Supported file formats
pub enum FileFormat {
    Png,
    // Add more formats as needed
}

impl FileFormat {
    /// Detect file format based on file extension and magic bytes
    pub fn detect(file_path: &str) -> Result<Self> {
        let path = Path::new(file_path);
        let extension = path.extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("")
            .to_lowercase();
        
        // Read first few bytes to check magic numbers
        let mut file = std::fs::File::open(file_path)?;
        let mut buffer = [0; 8];
        std::io::Read::read_exact(&mut file, &mut buffer)?;
        
        match extension.as_str() {
            "png" => {
                if &buffer[0..8] == Png::STANDARD_HEADER {
                    Ok(FileFormat::Png)
                } else {
                    Err("Invalid PNG file".into())
                }
            }
            _ => Err(format!("Unsupported file format: {}", extension).into())
        }
    }
}

impl SteganographyFormat for FileFormat {
    fn auto_detect_hidden_data(&self, file_path: &str) -> Result<Vec<HiddenData>> {
        match self {
            FileFormat::Png => {
                let png = Png::from_file(file_path)?;
                Ok(png.auto_detect_hidden_data())
            }
        }
    }

    fn extract_all_hidden_data(&self, file_path: &str) -> Result<Vec<ExtractedData>> {
        match self {
            FileFormat::Png => {
                let png = Png::from_file(file_path)?;
                png.extract_all_hidden_data()
            }
        }
    }

    fn list_custom_chunks(&self, file_path: &str) -> Result<Vec<String>> {
        match self {
            FileFormat::Png => {
                let png = Png::from_file(file_path)?;
                Ok(png.custom_chunks().iter().map(|c| c.chunk_type().to_string()).collect())
            }
        }
    }

    fn hide_data(&self, file_path: &str, data: Vec<u8>, chunk_type: ChunkType, output_path: &str) -> Result<()> {
        match self {
            FileFormat::Png => {
                let mut png = Png::from_file(file_path)?;
                let chunk = crate::png::Chunk::new(chunk_type, data);
                png.append_chunk(chunk);
                png.to_file(output_path)
            }
        }
    }
} 