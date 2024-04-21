use clap::{ArgGroup, Parser};
use std::path::Path;
use std::process::{exit, Command};

/// Wrapper around a Python script to convert CSG files to STEP format
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(group(ArgGroup::new("file_paths").required(true).args(&["input", "output"])))]
struct Cli {
    /// Input CSG file
    #[clap(value_parser)]
    input: String,

    /// Output STEP file
    #[clap(value_parser)]
    output: String,
}

fn main() {
    let cli = Cli::parse();

    // Check if the input file exists
    if !Path::new(&cli.input).exists() {
        eprintln!("Error: Input file '{}' does not exist.", cli.input);
        exit(1);
    }

    // Call the Python script
    match Command::new("python")
        .arg("script.py")
        .arg(cli.input)
        .arg(cli.output)
        .status()
    {
        Ok(status) if status.success() => (),
        Ok(_) => {
            eprintln!("Python script execution failed.");
            exit(1);
        }
        Err(e) => {
            eprintln!("Failed to execute python script: {}", e);
            exit(1);
        }
    }
}
