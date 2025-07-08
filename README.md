# Veil - Multi-Format Steganography Tool

A Rust library and command-line tool for hiding and extracting data in various file formats using steganography techniques. Features automatic format detection and a clean abstraction for multiple file types.

## Features

- **Check**: Detect if there is hidden data in a file
- **Hide**: Hide text messages or files inside other files
- **Extract**: Extract all hidden data from files
- **Multi-Format Support**: Currently PNG (extensible design for more formats)
- **Clean API**: Simple trait-based design for easy library usage

### Planned:

- **More File Formats**: Like JPEG or PDF
- **Better Detection**: Use a more sophisticated approach to type detection (e.g.: headers)
- **Multiple chunks**: Support splitting large data across multiple chunks
- **Encryption**: Add optional encryption of hidden data
- **Compression**: Compress data before hiding

## Installation

```bash
git clone https://github.com/mitsimi/veil.git
cd veil
cargo build --release
```

## Usage

### Command Line Interface

```bash
# Check if there is hidden data
veil check -f image.png

# Hide a text message inside an image
veil hide -f image.png -m "Secret message" -o hidden_image.png

# Hide a file inside an image
veil hide -f image.png -d secret.txt -o hidden_image.png

# Hide piped data
echo "Secret message" | veil hide -f image.png -o hidden_image.png

# Extract hidden data
veil extract -f hidden_image.png -o extracted/
```

### As a Library

The library provides a clean, extensible API through the `Steganography` trait:

```rust
use veil::{SteganographyFile, Steganography};

// Load any supported file format (automatic detection)
let mut file = SteganographyFile::from_file("image.png")?;

// Check if file contains hidden data
if file.has_hidden_data() {
    println!("Hidden data detected!");
}

// Hide data (text or binary)
let secret_message = b"This is my secret message";
file.hide_data(secret_message)?;

// Save the modified file
file.save_to_file("output.png")?;

// Extract hidden data
let hidden_data = file.extract_data()?;
let message = String::from_utf8(hidden_data)?;
println!("Secret: {}", message);
```

#### Working with specific formats

```rust
use veil::png::{Png, Chunk, ChunkType};
use std::str::FromStr;

// Direct PNG manipulation
let mut png = Png::from_file("image.png")?;

// Create custom chunk
let chunk_type = ChunkType::from_str("tEXt")?;
let chunk = Chunk::new(chunk_type, b"metadata".to_vec());
png.append_chunk(chunk);

// Save modified PNG
png.to_file("output.png")?;

// Extract custom chunks
let custom_chunks = png.custom_chunks();
for chunk in custom_chunks {
    println!("Found chunk: {}", chunk.chunk_type());
    println!("Data: {:?}", chunk.data_as_string());
}
```

#### Adding New File Formats

The library is designed for easy extension. To add a new format:

```rust
// 1. Add to the enum
pub enum SteganographyFile {
    Png(png::Png),
    Jpeg(jpeg::Jpeg),  // New format
}

// 2. Update the match statements in the Steganography impl
impl Steganography for SteganographyFile {
    fn hide_data(&mut self, data: &[u8]) -> Result<()> {
        match self {
            SteganographyFile::Png(png) => { /* PNG logic */ }
            SteganographyFile::Jpeg(jpeg) => { /* JPEG logic */ }
        }
    }
    // ... other methods
}

// 3. Add detection logic in from_file()
```

## Supported File Formats

### Currently Implemented

- **PNG**: Uses custom chunks with type "vEiL" to store hidden data
  - Leverages PNG's built-in chunk system
  - Preserves image integrity and compatibility
  - Supports any binary data

### Planned Formats

- **JPEG**: LSB (Least Significant Bit) manipulation in pixel data
- **PDF**: Metadata insertion and hidden streams

## Design Philosophy

- **Simplicity**: Clean API with minimal complexity
- **Extensibility**: Easy to add new file formats
- **Safety**: Leverages Rust's type system for correctness
- **Performance**: Zero-copy operations where possible
- **Compatibility**: Generated files work with standard tools
