// Owner: PascalElixir / axolrs (GitHub org)
// File: axol-hot-runner - the Axolotl interpreter, JIT, and hot-reload CLI for dev mode.

use axol_hot_runner::{jit, reload};
use clap::{Parser as ClapParser, Subcommand};
use std::fs;
use std::path::PathBuf;

#[derive(ClapParser)]
#[command(name = "axol-hot-runner", version, about = "The Axolotl hot interpreter + JIT")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Interpret an Axolotl source file.
    Run {
        #[arg(value_name = "FILE")]
        file: PathBuf,
    },
    /// Watch a file and re-interpret on change, preserving top-level state.
    Watch {
        #[arg(value_name = "FILE")]
        file: PathBuf,
        /// Run one cycle without entering the watch loop.
        #[arg(long)]
        once: bool,
    },
    /// Run an Axolotl source file through the Cranelift JIT with interpreter fallback.
    Jit {
        #[arg(value_name = "FILE")]
        file: PathBuf,
        /// Print which functions were JIT-compiled vs interpreted to stderr.
        #[arg(long = "trace-jit")]
        trace_jit: bool,
    },
    /// Drop into the interactive REPL.
    Repl,
}

/// The entry point of the axol-hot-runner CLI.
fn main() {
    let cli = Cli::parse();
    match cli.command {
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
        Command::Watch { file, once } => {
            if let Err(e) = reload::watch(&file, once) {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        }
        Command::Jit { file, trace_jit } => {
            let src = fs::read_to_string(&file).expect("read file");
            match jit::run_traced(&src, trace_jit) {
                Ok(out) => print!("{}", out),
                Err(errs) => {
                    for e in errs {
                        eprintln!("{}", e);
                    }
                    std::process::exit(1);
                }
            }
        }
        Command::Repl => {
            println!("Axolotl REPL - Ctrl-D to exit");
            let stdin = std::io::stdin();
            let mut line = String::new();
            loop {
                line.clear();
                print!("axol> ");
                use std::io::Write;
                let _ = std::io::stdout().flush();
                if stdin.read_line(&mut line).unwrap_or(0) == 0 {
                    break;
                }
                if line.is_empty() {
                    break;
                }
                let (out, diags) = axolc_core::interpret(&line, 0);
                print!("{}", out);
                for d in &diags.items {
                    if d.severity == axolc_core::diag::Severity::Error {
                        eprintln!("{}", d);
                    }
                }
            }
        }
    }
}
