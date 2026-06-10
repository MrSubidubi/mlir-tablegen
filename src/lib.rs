mod server;

use zed_extension_api::{self as zed, LanguageServerId, Result, Worktree};

struct MlirSuiteExtension;

impl zed::Extension for MlirSuiteExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<zed::Command> {
        let server = server::from_id(id.as_ref())?;
        server.resolve_command(worktree)
    }
}

zed::register_extension!(MlirSuiteExtension);
