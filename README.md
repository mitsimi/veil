# Veil - PNG Steganography Tool

A Rust library and command-line tool for encoding and decoding hidden messages in PNG files using custom chunks.

## Features

- **Encode**: Hide messages in PNG files using custom chunk types
- **Decode**: Extract hidden messages from PNG files
- **Remove**: Remove specific chunks from PNG files
- **Print**: Display PNG file structure and chunk information

## Installation

```bash
git clone <repository-url>
cd veil
cargo build --release
```

## Usage

### Command Line Interface

```bash
# Encode a message
cargo run -- encode -f input.png -t "RuSt" -m "Hello, World!" -o output.png

# Decode a message
cargo run -- decode -f input.png -t "RuSt"

# Remove a chunk
cargo run -- remove -f input.png -t "RuSt"

# Print PNG information
cargo run -- print -f input.png
```

### As a Library

```rust
use veil::{encode_message, decode_message};

// Encode a message
encode_message("input.png", "RuSt", "Secret message", Some("output.png"))?;

// Decode a message
let message = decode_message("input.png", "RuSt")?;
println!("Hidden message: {}", message);
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
│   └── cmd/            # Command-line interface
│       ├── mod.rs      # Module declarations
│       └── cli.rs      # CLI argument parsing
├── tests/              # Integration tests (future)
├── examples/           # Example code (future)
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

## License

[Add your license here] 