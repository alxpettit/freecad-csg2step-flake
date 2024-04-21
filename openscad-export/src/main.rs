use anyhow::{Context, Result};
use clap::{Parser, ValueEnum};
use std::{fs, process::Command};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, value_enum)]
    formats: Option<Vec<Format>>,
    #[clap(value_parser)]
    input_file: std::path::PathBuf,
    #[clap(
        help = "Overrides --formats, forcing all formats to be selected. Otherwise a default set of formats will be selected"
    )]
    #[clap(short, long)]
    all_formats: bool,
}

#[derive(Display, Debug, Clone, PartialEq, Eq, ValueEnum, EnumIter)]
enum Format {
    Stl,
    Off,
    Amf,
    Threemf,
    Csg,
    Step,
    Dxf,
    Svg,
    Pdf,
    Png,
    Echo,
    Ast,
    Term,
    Nef3,
    Nefdbg,
}

fn main() -> Result<()> {
    let args = Args::parse();

    if !args.input_file.exists() {
        anyhow::bail!("Error: File does not exist.");
    }

    let formats = args.formats.unwrap_or(if args.all_formats {
        Format::iter().collect()
    } else {
        use Format as F;
        vec![F::Csg, F::Step]
    });

    let out_dir = args.input_file.with_file_name("converted");
    fs::create_dir_all(&out_dir).with_context(|| "Failed to create output directory.")?;

    for format in formats {
        let output_file = out_dir
            .join(args.input_file.file_stem().unwrap())
            .with_extension(format.to_string().to_lowercase());
        println!("Exporting to {:?}...", format);
        let output = Command::new("openscad")
            .arg("-o")
            .arg(&output_file)
            .arg("-D")
            .arg(format!("output_format=\"{}\"", format))
            .arg(&args.input_file)
            .output()
            .with_context(|| format!("Failed to export {:?}", format))?;

        if !output.status.success() {
            eprintln!("Failed to export {:?}", format);
        } else {
            println!("{:?} export completed successfully.", format);

            if format == Format::Csg {
                let step_output_file = out_dir
                    .join(args.input_file.file_stem().unwrap())
                    .with_extension("step");
                println!("Attempting to convert CSG to STEP...");
                let output = Command::new("freecad-convert-shape-cli")
                    .arg(&output_file)
                    .arg(&step_output_file)
                    .output()
                    .with_context(|| "Failed to convert CSG to STEP")?;

                if !(output.status.code() == Some(2) || output.status.success()) {
                    eprintln!("Failed to convert CSG to STEP");
                } else {
                    println!("CSG to STEP conversion successful (probably).");
                }
            }
        }
    }

    println!(
        "All done. Check your {:?} directory for the outputs.",
        out_dir
    );
    Ok(())
}
