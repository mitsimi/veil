use clap::Parser;
use std::path::Path;
use veil::{Cli, Commands, Steganography, SteganographyFile};

fn main() -> veil::Result<()> {
    let args = Cli::parse();

    match args.command {
        Commands::Check { file_path } => {
            // Load the file and check if it contains hidden data
            let file = SteganographyFile::from_file(&file_path)?;

            if file.has_hidden_data() {
                println!("✓ Hidden data found in {}", file_path);
            } else {
                println!("✗ No hidden data found in {}", file_path);
            }
        }

        Commands::Hide {
            file_path,
            data_path,
            message,
            output_path,
        } => {
            // Load the host file
            let mut file = SteganographyFile::from_file(&file_path)?;

            // Determine what data to hide (either from file or message)
            let data_to_hide = if let Some(data_path) = data_path {
                // Hide data from a file
                std::fs::read(&data_path)?
            } else if let Some(message) = message {
                // Hide a text message
                message.into_bytes()
            } else {
                return Err("Either --data-path or --message must be provided".into());
            };

            // Hide the data
            file.hide_data(&data_to_hide)?;

            // Determine output path (use input path with "_hidden" suffix if not provided)
            let output_file = match output_path {
                Some(path) => path,
                None => {
                    let input_path = Path::new(&file_path);
                    let stem = input_path.file_stem().unwrap().to_string_lossy();
                    let extension = input_path.extension().unwrap().to_string_lossy();
                    format!("{}_hidden.{}", stem, extension)
                }
            };

            // Save to output file
            file.save_to_file(&output_file)?;

            println!("✓ Data hidden successfully in {}", output_file);
            println!("  Hidden {} bytes", data_to_hide.len());
        }

        Commands::Extract {
            file_path,
            output_dir,
        } => {
            // Load the file
            let file = SteganographyFile::from_file(&file_path)?;

            // Extract the hidden data
            let hidden_data = file.extract_data()?;

            // Determine output directory (use current directory if not provided)
            let output_directory = output_dir.unwrap_or_else(|| ".".to_string());

            // Create output directory if it doesn't exist
            std::fs::create_dir_all(&output_directory)?;

            // Save extracted data to a file
            let output_file = Path::new(&output_directory).join("extracted_data.bin");
            std::fs::write(&output_file, &hidden_data)?;

            println!(
                "✓ Extracted {} bytes to {}",
                hidden_data.len(),
                output_file.display()
            );

            // Try to display as text if it's valid UTF-8
            match String::from_utf8(hidden_data.clone()) {
                Ok(text) => {
                    println!("  Content (as text): {}", text);
                }
                Err(_) => {
                    println!("  Content: Binary data (not valid UTF-8)");
                }
            }
        }
    }

    Ok(())
}
