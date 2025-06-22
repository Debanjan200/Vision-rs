use clap::Parser;

/// Reads the path from the cli
#[derive(Parser, Debug)] // Derive the Parser and Debug traits for our struct
#[command(author, version, about, long_about = None)] // Add metadata for help message
pub struct Cli {
    /// The path to the input file to read
    #[arg(short, long, value_name = "FILE")] // Define a short (-f) and long (--file) flag for the argument
    pub file: String,
}
