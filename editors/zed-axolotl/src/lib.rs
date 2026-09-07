// Owner: PascalElixir / axolrs (GitHub org)
// File: editors/zed-axolotl/src/lib.rs - Zed extension for Axolotl: spawns the Gills LSP server.

use zed_extension_api::{self as zed, Command, Extension, LanguageServerId, Result};

const SERVER_PATH: &str = "axol-analyzer";

struct AxolotlExtension;

impl Extension for AxolotlExtension {
    /// Returns a new instance of the extension.
    fn new() -> Self {
        AxolotlExtension
    }

    /// Returns the command that starts the Gills LSP server over stdio.
    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<Command> {
        Ok(Command {
            command: SERVER_PATH.to_string(),
            args: vec!["stdio".to_string()],
            env: vec![],
        })
    }
}

zed_extension_api::register_extension!(AxolotlExtension);
