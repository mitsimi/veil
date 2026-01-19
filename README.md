# Veil - PNG Steganography Tool

A Rust library and command-line tool for hiding and extracting data in PNG files using steganography. This project was created as a learning exercise to explore Rust's type system, error handling, and working with binary file formats.

## Features

- **Check**: Detect if there is hidden data in a PNG file
- **Hide**: Hide text messages or files inside PNG images
- **Extract**: Extract hidden data from PNG files
- **Clean API**: Simple trait-based design for library usage
- **PNG Chunks**: Uses custom "vEiL" chunks to store hidden data

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

// Load a PNG file
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

#### Working with PNG directly

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

## How It Works

Veil uses PNG's chunk-based structure to hide data. PNG files are composed of chunks, each with a type and data. The tool creates custom chunks with the type "vEiL" to store hidden data.

- **Leverages PNG's built-in chunk system** - No pixel manipulation required
- **Preserves image integrity** - The image displays normally in any PNG viewer
- **Supports any binary data** - Hide text, files, or any other data
- **Standard compliant** - Generated files work with all PNG-compatible tools

## License

MIT
