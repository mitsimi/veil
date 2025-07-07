use clap::Parser;
use veil::{Cli, Commands, encode_message, decode_message, remove_chunk, print_png_info};

fn main() -> veil::Result<()> {
    let args = Cli::parse();

    match args.command {
        Commands::Encode { file_path, chunk_type, message, output_path } => {
            encode_message(&file_path, &chunk_type, &message, output_path.as_deref())?;
        }
        Commands::Decode { file_path, chunk_type } => {
            let message = decode_message(&file_path, &chunk_type)?;
            println!("Message: {}", message);
        }
        Commands::Remove { file_path, chunk_type } => {
            remove_chunk(&file_path, &chunk_type)?;
        }
        Commands::Print { file_path } => {
            print_png_info(&file_path)?;
        }
    }

    Ok(())
}
