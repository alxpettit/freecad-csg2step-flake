use anyhow::{Context, Result};
use clap::{ArgGroup, Parser, ValueEnum};
use env_logger::Env;
use log::warn;
use std::{cmp::Ordering, fs, process::Command};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(group(ArgGroup::new("format").required(true).args(&["formats", "all_formats", "step"])))]
struct Args {
    #[clap(short, long, value_enum)]
    formats: Option<Vec<Format>>,
    #[clap(value_parser)]
    input_file: std::path::PathBuf,
    #[clap(short, long, value_parser)]
    output_path: Option<std::path::PathBuf>,
    /// Selects all available formats
    #[clap(short, long)]
    all_formats: bool,
    /// Shortcut for selecting Step format
    #[clap(short, long)]
    step: bool,
}

#[derive(Display, Debug, Clone, PartialEq, Eq, ValueEnum, EnumIter)]
enum Format {
    Stl,
    Off,
    Amf,
    #[clap(name = "3mf")]
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

impl PartialOrd for Format {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Format {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            // Ensures that Csg is sorted BEFORE Step, otherwise our code will break!
            (Format::Csg, Format::Step) => Ordering::Less,
            (Format::Step, Format::Csg) => Ordering::Greater,
            _ => self.to_string().cmp(&(other.to_string())),
        }
    }
}

fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("warn")).init();
    let args = Args::parse();

    if !args.input_file.exists() {
        anyhow::bail!("Error: File does not exist.");
    }

    let mut formats = args.formats.unwrap_or(if args.all_formats {
        Format::iter().collect()
    } else {
        use Format as F;
        vec![F::Csg, F::Step]
    });

    if formats.contains(&Format::Step) && !formats.contains(&Format::Csg) {
        warn!("You have requested Step format, but not Csg. Csg is a required intermediate format for our conversion process, and will be added.");
        formats.push(Format::Csg);
    }

    let out_dir = args
        .output_path
        .unwrap_or(args.input_file.with_file_name("converted"));
    fs::create_dir_all(&out_dir).with_context(|| "Failed to create output directory.")?;

    // IMPORTANT -- sort formats before using
    // the ordering of conversions matters because some conversions may have to occur BEFORE others!
    formats.sort();
    println!("Formats selected: {:?}", formats);
    for format in formats {
        let output_file = out_dir
            .join(args.input_file.file_stem().unwrap())
            .with_extension(format.to_string().to_lowercase());
        match format {
            Format::Step => {
                let step_output_file = out_dir
                    .join(args.input_file.file_stem().unwrap())
                    .with_extension("step");
                println!("Attempting to convert Csg to Step...");
                let output = Command::new("freecad-convert-shape-cli")
                    .arg(&output_file)
                    .arg(&step_output_file)
                    .output()
                    .with_context(|| "Failed to convert Csg to Step")?;

                if output.status.code() != Some(2) && !output.status.success() {
                    eprintln!("Failed to convert Csg to Step: {:?}", output.status);
                } else {
                    println!("Csg to Step conversion successful.");
                }
            }
            _ => {
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
                }
            }
        }
    }

    println!(
        "All done. Check your `{}` directory for the outputs.",
        out_dir.display()
    );
    Ok(())
}
