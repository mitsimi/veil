# Veil - Multi-Format Steganography Tool

A Rust library and command-line tool for hiding and extracting data in various image formats using custom chunks. Features automatic detection and extraction of hidden data.

## Features

- **Check**: Detect if there is hidden data in a file
- **Hide**: Hide any file inside a file
- **Extract**: Automatically extract all hidden data with proper file extensions
- **Multi-Format Support**: PNG, (others planned)

## Installation

```bash
git clone <repository-url>
cd veil
cargo build --release
```

## Usage

### Command Line Interface

```bash
# Check if there is hidden data
veil check image.png
veil check image.jpg

# Hide a file inside an image
veil hide image.png secret.txt -o hidden_image.png
veil hide image.jpg secret.txt -o hidden_image.jpg

# Hide a text message
veil hide image.png -m "Secret message" -o hidden_image.png

# Extract all hidden data
veil extract image.png -o extracted/
veil extract image.jpg -o extracted/
```

### Supported File Formats

Currently implemented:
- **PNG**: Full support with custom chunks

### Automatic Detection Features

The tool can automatically detect and extract various types of hidden data:

- **Text**: Plain text messages (.txt files)
- **JSON**: Structured data (.json files)
- **Images**: PNG, JPEG, GIF, BMP files
- **Documents**: PDF, DOC, XLS, PPT files
- **Compressed Data**: ZIP, Gzip compressed content
- **Binary**: Any other binary data

### As a Library

```rust
use veil::{auto_detect_hidden_data, extract_all_hidden_data, hide_data};

// Check for hidden data (works with any supported format)
let hidden_data = auto_detect_hidden_data("image.png")?;
for data in hidden_data {
    println!("Found {:?} data, {} bytes", data.data_type, data.size);
}

// Extract all hidden data
let extracted = extract_all_hidden_data("image.jpg")?;
for data in extracted {
    match data.content {
        veil::Content::Text(text) => println!("Text: {}", text),
        veil::Content::Image(img_data) => println!("Image: {} bytes", img_data.len()),
        // ... handle other types
    }
}

// Hide data in any supported format
let data = "Secret message".as_bytes().to_vec();
let chunk_type = veil::ChunkType::from_str("TeXt")?;
hide_data("image.png", data, chunk_type, "output.png")?;
```

## Project Structure

```
veil/
├── src/
│   ├── lib.rs          # Library entry point and public API
│   ├── main.rs         # Binary entry point (CLI)
│   ├── png/            # PNG handling module
│   │   ├── mod.rs      # Module declarations
│   │   ├── png.rs      # PNG struct and implementation
│   │   ├── chunk.rs    # Chunk handling
│   │   └── chunk_type.rs # Chunk type validation
│   ├── formats/        # File format support
│   │   └── mod.rs      # Format detection and traits
│   └── cmd/            # Command-line interface
│       ├── mod.rs      # Module declarations
│       └── cli.rs      # CLI argument parsing
└── README.md           # This file
```

## Development

### Running Tests

```bash
cargo test
```

### Building Documentation

```bash
cargo doc --open
```

### Adding New File Formats

To add support for a new file format:

1. Add the format to the `FileFormat` enum in `src/formats/mod.rs`
2. Implement the detection logic in `FileFormat::detect()`
3. Implement the `SteganographyFormat` trait methods for the new format
4. Add tests for the new format