use clap::{ArgGroup, Parser, Subcommand, command};

#[derive(Debug, Parser)]
#[command(name = "veil")]
#[command(about = "A tool for hiding and extracting data in PNG images", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Check if there is hidden data in a file
    #[command(arg_required_else_help = true)]
    Check {
        #[arg(short = 'f', long = "file")]
        file_path: String,
    },
    /// Hide data inside a file
    #[command(arg_required_else_help = true)]
    #[command(group(
        ArgGroup::new("input")
            .args(["data_path", "message"])
    ))]
    Hide {
        #[arg(short = 'f', long = "file")]
        file_path: String,

        #[arg(short = 'd', long = "data", group = "input")]
        data_path: Option<String>,

        #[arg(short = 'm', long = "message", group = "input")]
        message: Option<String>,

        #[arg(short = 'o', long = "output")]
        output_path: Option<String>,
    },
    /// Extract all hidden data from a file
    #[command(arg_required_else_help = true)]
    Extract {
        #[arg(short = 'f', long = "file")]
        file_path: String,
        #[arg(short = 'o', long = "output")]
        output_dir: Option<String>,
    },
}
