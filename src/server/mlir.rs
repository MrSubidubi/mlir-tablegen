use super::LanguageServer;

/// Language-server integration for `mlir-lsp-server`.
pub struct MlirServer;

impl MlirServer {
    /// Zed server ID and default LLVM binary name.
    pub const SERVER_ID: &'static str = "mlir-lsp-server";
}

impl LanguageServer for MlirServer {
    fn id(&self) -> &'static str {
        Self::SERVER_ID
    }

    fn default_binary(&self) -> &'static str {
        Self::SERVER_ID
    }
}
