// Owner: PascalElixir / axolrs (GitHub org)
// File: crates/bucket/src/main.rs - the bucket CLI entry point: argument parsing and command dispatch.

use bucket::{bench, build, doc, doctor, fix, fmt, gills, lint, project, publish, run, upgrade};
use clap::{Parser as ClapParser, Subcommand};
use std::path::PathBuf;

#[derive(ClapParser)]
#[command(name = "bucket", version, about = "The Axolotl build orchestrator")]
struct Cli {
    #[command(subcommand)]
    command: Command,
    #[arg(long)]
    json: bool,
}

#[derive(Subcommand)]
enum Command {
    /// Create a new Axolotl project.
    New { name: String },
    /// Initialize a Bucket.jsonc in the current directory.
    Init,
    /// Add a dependency to the manifest.
    Add { name: String, version: Option<String> },
    /// Build the project (release mode by default with `--release`).
    Build { #[arg(long)] release: bool },
    /// Run the project (interprets in dev mode, runs the binary in release mode).
    Run { #[arg(long)] watch: bool, args: Vec<String> },
    /// Run the project's tests.
    Test,
    /// Format the project with Shed.
    Fmt,
    /// Lint the project with Neoten.
    Lint,
    /// Apply auto-fixes to the project with Regrow.
    Fix,
    /// Generate docs for the project with Ambystoma.
    Doc,
    /// Clean build artifacts.
    Clean,
    /// Run the doctor (health check).
    Doctor,
    /// Emit the generated Rust for the project.
    EmitRust { file: Option<PathBuf> },
    /// Run benchmarks with Salamander.
    Bench,
    /// Run a script defined in Bucket.jsonc.
    Script { name: String, args: Vec<String> },
    /// Publish to crates.io via Eggbox.
    Publish,
    /// Upgrade the toolchain or migrate the project with Molt.
    Upgrade,
    /// Start the Gills LSP server.
    Gills { #[arg(long)] stdio: bool },
}

/// The entry point of the bucket CLI.
fn main() {
    let cli = Cli::parse();
    match cli.command {
        Command::New { name } => project::new_project(&name),
        Command::Init => project::init_project(),
        Command::Add { name, version } => project::add_dep(&name, version),
        Command::Build { release } => build::build(release),
        Command::Run { watch, args } => run::run(watch, &args),
        Command::Test => run::test_cmd(),
        Command::Fmt => fmt::fmt_cmd(),
        Command::Lint => lint::lint_cmd(),
        Command::Fix => fix::fix_cmd(),
        Command::Doc => doc::doc_cmd(),
        Command::Clean => project::clean_cmd(),
        Command::Doctor => doctor::doctor(),
        Command::EmitRust { file } => build::emit_rust(file.as_deref()),
        Command::Bench => bench::bench(),
        Command::Script { name, args } => project::script(&name, &args),
        Command::Publish => publish::publish(),
        Command::Upgrade => upgrade::upgrade(),
        Command::Gills { stdio } => gills::gills(stdio),
    }
}
