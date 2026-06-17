use super::LanguageServer;

/// Language-server integration for `mlir-pdll-lsp-server`.
pub struct PdllServer;

impl PdllServer {
    /// Zed server ID and default LLVM binary name.
    pub const SERVER_ID: &'static str = "mlir-pdll-lsp-server";
}

impl LanguageServer for PdllServer {
    fn id(&self) -> &'static str {
        Self::SERVER_ID
    }

    fn default_binary(&self) -> &'static str {
        Self::SERVER_ID
    }

    fn compilation_db_flag(&self) -> Option<&'static str> {
        Some("--pdll-compilation-database")
    }

    fn compilation_db_filename(&self) -> Option<&'static str> {
        Some("pdll_compile_commands.yml")
    }

    fn extra_dir_flag(&self) -> Option<&'static str> {
        Some("--pdll-extra-dir")
    }
}
