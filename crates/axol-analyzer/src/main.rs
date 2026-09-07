// Owner: PascalElixir / axolrs (GitHub org)
// File: axol-analyzer (Gills) - the LSP server entry point: stdio serving, info, and check.

use std::path::PathBuf;

use clap::{Parser as ClapParser, Subcommand};

use axol_analyzer::analysis::analyze;
use axol_analyzer::{SERVER_NAME, SERVER_VERSION};

#[derive(ClapParser)]
#[command(name = "axol-analyzer", version, about = "Gills - the Axolotl LSP")]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Run the LSP server over framed stdio (the default).
    Stdio,
    /// Show LSP server info.
    Info,
    /// Print diagnostics for a file to stderr; exit code 1 on errors.
    Check {
        /// The .axol file to check.
        file: PathBuf,
    },
}

/// The entry point of the axol-analyzer CLI.
#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    match cli.command.unwrap_or(Command::Stdio) {
        Command::Stdio => {
            if let Err(e) = axol_analyzer::server::run_stdio().await {
                eprintln!("Gills LSP error: {e}");
                std::process::exit(1);
            }
        }
        Command::Info => {
            println!("{SERVER_NAME} - the Axolotl LSP, version {SERVER_VERSION}");
            println!("Owners: PascalElixir / axolrs");
            println!("A fork of rust-analyzer concepts, extended for mixed .axol + .rs projects.");
        }
        Command::Check { file } => {
            let path = file.to_string_lossy().to_string();
            let Ok(src) = std::fs::read_to_string(&file) else {
                eprintln!("Gills check: cannot read {path}");
                std::process::exit(2);
            };
            let report = analyze(&src).report_lines(&path);
            for line in &report {
                eprintln!("{line}");
            }
            if analyze(&src).has_errors() {
                std::process::exit(1);
            }
        }
    }
}
