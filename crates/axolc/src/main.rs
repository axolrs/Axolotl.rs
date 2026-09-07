// Owner: PascalElixir / axolrs (GitHub org)
// File: axolc - the Axolotl to Rust compiler CLI driver.

use clap::{Parser as ClapParser, Subcommand};
use std::path::PathBuf;
use std::fs;

#[derive(ClapParser)]
#[command(name = "axolc", version, about = "The Axolotl to Rust compiler")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Compile an Axolotl source file to Rust.
    Compile {
        #[arg(value_name = "FILE")]
        file: PathBuf,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    /// Emit the generated Rust for an Axolotl source file.
    EmitRust {
        #[arg(value_name = "FILE")]
        file: PathBuf,
    },
    /// Tokenize an Axolotl source file and print the token stream.
    Tokenize {
        #[arg(value_name = "FILE")]
        file: PathBuf,
    },
    /// Parse an Axolotl source file and report any errors.
    Parse {
        #[arg(value_name = "FILE")]
        file: PathBuf,
    },
    /// Interpret an Axolotl source file (dev mode).
    Run {
        #[arg(value_name = "FILE")]
        file: PathBuf,
    },
    /// Print version and toolchain info.
    Version,
}

/// The entry point of the axolc CLI.
fn main() {
    let cli = Cli::parse();
    match cli.command {
        Command::Compile { file, out } => {
            let src = fs::read_to_string(&file).expect("read file");
            let (rust, diags) = axolc_core::compile_to_rust(&src, 0);
            if diags.has_errors() {
                for d in &diags.items {
                    if d.severity == axolc_core::diag::Severity::Error {
                        eprintln!("{}", d);
                    }
                }
                std::process::exit(1);
            }
            match out {
                Some(p) => fs::write(&p, rust).expect("write output"),
                None => print!("{}", rust),
            }
        }
        Command::EmitRust { file } => {
            let src = fs::read_to_string(&file).expect("read file");
            let (rust, _diags) = axolc_core::compile_to_rust(&src, 0);
            print!("{}", rust);
        }
        Command::Tokenize { file } => {
            let src = fs::read_to_string(&file).expect("read file");
            let (tokens, diags) = axolc_core::tokenize(&src, 0);
            for t in &tokens {
                println!("{:?}: {:?}", t.kind, t.text);
            }
            for d in &diags.items {
                println!("{}", d);
            }
        }
        Command::Parse { file } => {
            let src = fs::read_to_string(&file).expect("read file");
            let (module, diags) = axolc_core::parse(&src, 0);
            println!("parsed {} items", module.items.len());
            for d in &diags.items {
                println!("{}", d);
            }
        }
        Command::Run { file } => {
            let src = fs::read_to_string(&file).expect("read file");
            let (out, diags) = axolc_core::interpret(&src, 0);
            print!("{}", out);
            for d in &diags.items {
                if d.severity == axolc_core::diag::Severity::Error {
                    eprintln!("{}", d);
                }
            }
        }
        Command::Version => {
            println!("axolc {} (axolotl programming language)", env!("CARGO_PKG_VERSION"));
            println!("owners: PascalElixir / axolrs");
        }
    }
}
