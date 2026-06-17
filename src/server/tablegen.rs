use super::LanguageServer;

/// Language-server integration for `tblgen-lsp-server`.
pub struct TablegenServer;

impl TablegenServer {
    /// Zed server ID and default LLVM binary name.
    pub const SERVER_ID: &'static str = "tblgen-lsp-server";
}

impl LanguageServer for TablegenServer {
    fn id(&self) -> &'static str {
        Self::SERVER_ID
    }

    fn default_binary(&self) -> &'static str {
        Self::SERVER_ID
    }

    fn compilation_db_flag(&self) -> Option<&'static str> {
        Some("--tablegen-compilation-database")
    }

    fn compilation_db_filename(&self) -> Option<&'static str> {
        Some("tablegen_compile_commands.yml")
    }

    fn extra_dir_flag(&self) -> Option<&'static str> {
        Some("--tablegen-extra-dir")
    }
}
