mod completion;
mod server;

use zed_extension_api::{self as zed, lsp::Completion, LanguageServerId, Result, Worktree};

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

    fn label_for_completion(
        &self,
        language_server_id: &LanguageServerId,
        completion: Completion,
    ) -> Option<zed::CodeLabel> {
        crate::completion::label_for(language_server_id.as_ref(), &completion)
    }
}

zed::register_extension!(MlirSuiteExtension);
