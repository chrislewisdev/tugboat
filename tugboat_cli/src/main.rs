use clap::Parser;
use std::fs;
use std::path::PathBuf;
use tugboat::CompilationError;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(required = true)]
    file: PathBuf,
    #[arg(short, long)]
    output: Option<PathBuf>,
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();

    let output = match args.output {
        Some(path) => path,
        None => {
            let mut renamed = args.file.clone();
            renamed.set_extension("asm");
            renamed
        }
    };

    let file = fs::read_to_string(&args.file);
    match file {
        Err(err) => println!("Unable to open {:?}: {}", args.file, err),
        Ok(contents) => {
            compile(contents, output, args.verbose);
        }
    }
}

fn compile(contents: String, output: PathBuf, verbose: bool) {
    let result = tugboat::compile(contents);
    match result {
        Ok(asm) => {
            if verbose {
                println!("{}", asm);
            }
            let write_result = fs::write(output.clone(), asm);
            if let Err(error) = write_result {
                println!("Failed to write {:?}: {}", output, error);
            }
        }
        Err(errors) => {
            report(errors);
        }
    }
}

fn report(errors: Vec<CompilationError>) {
    for err in errors {
        println!("[line {}] error: {}", err.line, err.msg);
    }
}
