// Owner: PascalElixir / axolrs (GitHub org)
// File: axol-analyzer (Gills) - the LSP server for Axolotl, forked from rust-analyzer concepts.

pub mod analysis;
pub mod line_index;
pub mod scope;
pub mod server;

pub use analysis::{analyze, definition_in_docs, references_in_docs, rename_in_docs, Analysis};
pub use scope::{Container, Def, DefKind};

/// The Gills server name reported in the `initialize` handshake.
pub const SERVER_NAME: &str = "Gills";

/// The Gills server version reported in the `initialize` handshake.
pub const SERVER_VERSION: &str = env!("CARGO_PKG_VERSION");
