use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(about = "A CLI to encode and decode information from and into PNG files. Made by following this project tutorial -> https://picklenerd.github.io/pngme_book/introduction.html", long_about = None)]
#[command(author = "S460")]
#[command(version = "1.0")]
pub struct CLI {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// encode message into file
    Encode {
        /// message to encode into the PNG file
        #[arg(short, long)]
        message: String,

        /// path to the PNG file
        #[arg(short, long)]
        file_path: std::path::PathBuf,

        /// type of the chunk to encode
        #[arg(short, long)]
        chunk_type: String,
        // add output file later on
    },
    /// decode message from file
    Decode {
        /// path to the PNG file
        #[arg(short, long)]
        file_path: std::path::PathBuf,

        /// type of the chunk to decode
        #[arg(short, long)]
        chunk_type: String,
    },

    /// remove chunk with specified chunk type
    Remove {
        /// path to the PNG file
        #[arg(short, long)]
        file_path: std::path::PathBuf,

        /// type of the chunk to remove
        #[arg(short, long)]
        chunk_type: String,
    },

    /// print the file
    Print {
        /// path to the PNG file to print
        #[arg(short, long)]
        file_path: std::path::PathBuf,
    },
    /// show an awesome banner
    Banner,
}
