use clap::{Parser, ValueEnum};
use std::fs;
use std::path::PathBuf;

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
enum Format {
    Rgbds,
    Raw,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // Files to compile
    #[arg(required = true)]
    file: PathBuf,
    // Compiled output format
    #[arg(long)]
    format: Option<Format>,
}

fn main() {
    let args = Args::parse();

    let file = fs::read_to_string(&args.file);
    match file {
        Err(err) => println!("Unable to open {:?}: {}", args.file, err),
        Ok(contents) => {
            println!("Compiling {:?}", args.file);
            tugboat::compile(args.file.file_stem().unwrap().to_str().unwrap(), contents);
        }
    }
}
