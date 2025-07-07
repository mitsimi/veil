use clap::Parser;
use veil::{Cli, Commands, auto_detect_hidden_data, extract_all_hidden_data, hide_data};
use std::str::FromStr;

fn main() -> veil::Result<()> {
    let args = Cli::parse();

    match args.command {
        Commands::Check { file_path } => {
            let hidden_data = auto_detect_hidden_data(&file_path)?;
            if hidden_data.is_empty() {
                println!("No hidden data found in {}", file_path);
            } else {
                println!("Found {} hidden data items in {}:", hidden_data.len(), file_path);
                for (i, data) in hidden_data.iter().enumerate() {
                    println!("  {}. Type: {:?}, Size: {} bytes", 
                            i + 1, data.data_type, data.size);
                }
            }
        }
        Commands::Hide { file_path, data_path, message, output_path } => {
            let (data, chunk_type, source_desc) = if let Some(data_path_val) = &data_path {
                // Hide a file
                let data = std::fs::read(data_path_val)?;
                let chunk_type = get_chunk_type_from_extension(data_path_val);
                (data, chunk_type, format!("file from {}", data_path_val))
            } else if let Some(message_text) = &message {
                // Hide a text message
                let data = message_text.as_bytes().to_vec();
                let chunk_type = veil::ChunkType::from_str("TeXt")?;
                (data, chunk_type, "message".to_string())
            } else {
                return Err("Either data_path or message must be provided".into());
            };
            
            // Save the result
            let output = output_path.unwrap_or_else(|| format!("hidden_{}", file_path));
            hide_data(&file_path, data, chunk_type, &output)?;
            
            println!("{} hidden in {}", source_desc, output);
        }
        Commands::Extract { file_path, output_dir } => {
            let extracted = extract_all_hidden_data(&file_path)?;
            if extracted.is_empty() {
                println!("No hidden data found in {}", file_path);
                return Ok(());
            }
            
            let output_dir = output_dir.unwrap_or_else(|| "extracted".to_string());
            std::fs::create_dir_all(&output_dir)?;
            
            println!("Extracting {} hidden data items from {} to '{}':", 
                    extracted.len(), file_path, output_dir);
            
            for (i, data) in extracted.iter().enumerate() {
                let filename = format!("{}/hidden_data_{}", output_dir, i + 1);
                match &data.content {
                    veil::Content::Text(text) => {
                        std::fs::write(format!("{}.txt", filename), text)?;
                        println!("  {}. Text data saved to {}.txt", i + 1, filename);
                    }
                    veil::Content::Json(json) => {
                        std::fs::write(format!("{}.json", filename), json)?;
                        println!("  {}. JSON data saved to {}.json", i + 1, filename);
                    }
                    veil::Content::Image(data) | veil::Content::Png(data) => {
                        std::fs::write(format!("{}.png", filename), data)?;
                        println!("  {}. PNG image saved to {}.png", i + 1, filename);
                    }
                    veil::Content::Jpeg(data) => {
                        std::fs::write(format!("{}.jpg", filename), data)?;
                        println!("  {}. JPEG image saved to {}.jpg", i + 1, filename);
                    }
                    veil::Content::Gif(data) => {
                        std::fs::write(format!("{}.gif", filename), data)?;
                        println!("  {}. GIF image saved to {}.gif", i + 1, filename);
                    }
                    veil::Content::Bmp(data) => {
                        std::fs::write(format!("{}.bmp", filename), data)?;
                        println!("  {}. BMP image saved to {}.bmp", i + 1, filename);
                    }
                    veil::Content::Gzip(data) => {
                        std::fs::write(format!("{}.gz", filename), data)?;
                        println!("  {}. Gzip data saved to {}.gz", i + 1, filename);
                    }
                    veil::Content::Zlib(data) => {
                        std::fs::write(format!("{}.zlib", filename), data)?;
                        println!("  {}. Zlib data saved to {}.zlib", i + 1, filename);
                    }
                    veil::Content::Binary(data) => {
                        std::fs::write(format!("{}.bin", filename), data)?;
                        println!("  {}. Binary data saved to {}.bin", i + 1, filename);
                    }
                }
            }
        }
    }

    Ok(())
}

/// Generate a chunk type based on file extension
fn get_chunk_type_from_extension(file_path: &str) -> veil::ChunkType {
    use std::str::FromStr;
    
    let extension = std::path::Path::new(file_path)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("bin")
        .to_lowercase();
    
    let chunk_type = match extension.as_str() {
        "txt" | "text" => "TeXt",
        "json" => "JSON",
        "png" => "PNG_",
        "jpg" | "jpeg" => "JPEG",
        "gif" => "GIF_",
        "bmp" => "BMP_",
        "zip" | "gz" => "ZIP_",
        "pdf" => "PDF_",
        "doc" | "docx" => "DOC_",
        "xls" | "xlsx" => "XLS_",
        "ppt" | "pptx" => "PPT_",
        _ => "DATA",
    };
    
    veil::ChunkType::from_str(chunk_type).unwrap_or_else(|_| {
        // Fallback to a safe chunk type
        veil::ChunkType::from_str("DATA").unwrap()
    })
}
