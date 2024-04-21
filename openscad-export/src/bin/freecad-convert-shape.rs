use anyhow::{Context, Result};
use clap::{ArgGroup, Parser, ValueEnum};
use env_logger::Env;
use log::info;
use std::path::{Path, PathBuf};
use std::process::{exit, Command};
use std::{env, fs};
use strum_macros::Display;

#[derive(Display, Debug, Clone, PartialEq, Eq, ValueEnum)]
enum Format {
    Step,
    Dae,
    Wrl,
    Iges,
}

/// Wrapper around a Python script to convert CSG files to STEP format
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Input CSG file
    #[clap(value_parser)]
    input_path: PathBuf,

    /// Format of file
    #[clap(short, long)]
    format: Format,

    /// Output STEP file
    #[clap(short, long, value_parser)]
    output_path: Option<PathBuf>,
}

fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("warn")).init();
    let cli = Cli::parse();

    // Check if the input file exists
    if !Path::new(&cli.input_path).exists() {
        eprintln!(
            "Error: Input file `{}` does not exist.",
            cli.input_path.display()
        );
        exit(1);
    }

    let out_path = match cli.output_path {
        Some(v) => v,
        None => {
            let mut dir_path = cli.input_path.with_file_name("converted");
            fs::create_dir_all(&dir_path).with_context(|| "Failed to create output directory.")?;
            dir_path.push(
                cli.input_path
                    .with_extension(&cli.format.to_string().to_lowercase())
                    .file_name()
                    .with_context(|| "Failed to get file name")?,
            );
            dir_path
        }
    };

    let freecad_dynamic = env::var("FREECAD_DYNAMIC").unwrap_or(String::from("false"));

    let freecad_convert_cmd = if freecad_dynamic == "true" {
        "freecad-convert-shape-dynamic"
    } else {
        "freecad-convert-shape-cli"
    };

    println!("Destination file path: {}", &out_path.display());

    info!("Calling child process: {}", &freecad_convert_cmd);

    match Command::new(freecad_convert_cmd)
        .arg(cli.input_path)
        .arg(out_path)
        .status()
    {
        Ok(status) if status.success() => (),
        Ok(status) => {
            eprintln!("Python script execution failed: {}", status);
            exit(1);
        }
        Err(e) => {
            eprintln!("Failed to execute python script: {}", e);
            exit(1);
        }
    }
    Ok(())
}
