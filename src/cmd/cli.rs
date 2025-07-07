use clap::{command, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "veil")]
#[command(about = "A tool for hiding messages in PNG images", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(arg_required_else_help = true)]
    Encode {
        file_path: String,
        chunk_type: String,
        message: String,
        output_path: Option<String>,
    },
    #[command(arg_required_else_help = true)]
    Decode {
        file_path: String,
        chunk_type: String,
    },
    #[command(arg_required_else_help = true)]
    Remove {
        file_path: String,
        chunk_type: String,
    },
    #[command(arg_required_else_help = true)]
    Print {
        file_path: String,
    },
}
