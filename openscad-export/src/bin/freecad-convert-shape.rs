use clap::{ArgGroup, Parser};
use log::info;
use std::env;
use std::path::Path;
use std::process::{exit, Command};

/// Wrapper around a Python script to convert CSG files to STEP format
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Input CSG file
    #[clap(value_parser)]
    input: String,

    /// Output STEP file
    #[clap(value_parser)]
    output: String,
}

fn main() {
    env_logger::init();
    let cli = Cli::parse();

    // Check if the input file exists
    if !Path::new(&cli.input).exists() {
        eprintln!("Error: Input file '{}' does not exist.", cli.input);
        exit(1);
    }

    let freecad_dynamic = env::var("FREECAD_DYNAMIC").unwrap_or(String::from("false"));

    let freecad_convert_cmd = if freecad_dynamic == "true" {
        "freecad-convert-shape-dynamic"
    } else {
        "freecad-convert-shape-cli"
    };

    info!("Calling child process: {}", &freecad_convert_cmd);

    match Command::new(freecad_convert_cmd)
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
