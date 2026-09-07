// Owner: PascalElixir / axolrs (GitHub org)
// File: crates/bucket/src/lib.rs - the build orchestrator's library surface, shared by the CLI and integration tests.

pub mod bench;
pub mod build;
pub mod doc;
pub mod doctor;
pub mod fix;
pub mod fmt;
pub mod gills;
pub mod lint;
pub mod project;
pub mod publish;
pub mod run;
pub mod upgrade;

/// Re-export of the manifest module for downstream consumers.
pub use axolc_core::manifest;

/// Re-export of the manifest's Manifest struct.
pub use axolc_core::manifest::Manifest;
