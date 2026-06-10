use serde::Deserialize;
use zed_extension_api::{self as zed, serde_json, settings::LspSettings, Result, Worktree};

/// Structured settings that can be provided via the `"settings"` field of
/// `lsp.<server-id>` in Zed's `settings.json`.
///
/// ```jsonc
/// "lsp": {
///   "tblgen-lsp-server": {
///     "settings": {
///       "path": "/path/to/tblgen-lsp-server",
///       "compilation_database": "build/tablegen_compile_commands.yml",
///       "extra_dirs": ["include/"],
///       "log": "verbose",
///       "pretty": true
///     }
///   }
/// }
/// ```
///
/// All fields are optional. Fields that do not apply to a given server
/// (e.g. `compilation_database` for `mlir-lsp-server`) are silently ignored.
#[derive(Debug, Default, Deserialize)]
pub struct ServerSettings {
    /// Path to the server binary (alternative to `binary.path`).
    pub path: Option<String>,
    /// Path to the compilation-database YAML file (tblgen / pdll only).
    pub compilation_database: Option<String>,
    /// Extra include directories (tblgen / pdll only).
    #[serde(default)]
    pub extra_dirs: Vec<String>,
    /// Log verbosity: `"error"`, `"info"`, or `"verbose"`.
    pub log: Option<String>,
    /// Pretty-print JSON output from the server.
    pub pretty: Option<bool>,
}

mod mlir;
mod pdll;
mod tablegen;

pub use mlir::MlirServer;
pub use pdll::PdllServer;
pub use tablegen::TablegenServer;

/// Candidate paths to probe for compilation-database files, relative to the
/// worktree root. Listed from most-specific to least-specific.
const BUILD_DIR_CANDIDATES: &[&str] = &["build", "out"];

/// Trait defining the interface for a language server integration.
///
/// The default implementation of [`resolve_command`](LanguageServer::resolve_command)
/// handles the common boilerplate:
/// - reading per-worktree `LspSettings` from Zed's `settings.json`
/// - resolving the server binary from a user-provided path or `$PATH`
/// - assembling the final `zed::Command` with arguments and environment variables
/// - auto-detecting compilation-database files when the server declares candidates.
pub trait LanguageServer {
    /// The language-server ID used in `extension.toml` and `settings.json`.
    fn id(&self) -> &'static str;

    /// The default binary name looked up on `$PATH`.
    fn default_binary(&self) -> &'static str;

    /// The CLI flag prefix for the compilation database, *without* the `=`.
    ///
    /// Return `None` (the default) if this server does not support a
    /// compilation database flag.
    fn compilation_db_flag(&self) -> Option<&'static str> {
        None
    }

    /// The filename of the compilation-database file this server expects
    /// (e.g. `"tablegen_compile_commands.yml"`).
    ///
    /// The default `resolve_command` will probe
    /// `<build_dir>/<filename>` for each directory in
    /// [`BUILD_DIR_CANDIDATES`].
    fn compilation_db_filename(&self) -> Option<&'static str> {
        None
    }

    /// The CLI flag prefix for extra include directories, *without* the `=`.
    ///
    /// Return `None` (the default) if this server does not support extra dirs.
    fn extra_dir_flag(&self) -> Option<&'static str> {
        None
    }

    /// Resolve the full command used to start this language server.
    ///
    /// Flag resolution priority (highest → lowest):
    /// 1. Raw flags in `binary.arguments`
    /// 2. Structured fields in `settings`
    /// 3. Auto-detection from the worktree
    fn resolve_command(&self, worktree: &Worktree) -> Result<zed::Command> {
        let lsp_settings = LspSettings::for_worktree(self.id(), worktree).ok();

        let user_binary = lsp_settings.as_ref().and_then(|s| s.binary.as_ref());

        // Parse structured settings from `lsp.<id>.settings`.
        // A parse error is logged and downgraded to defaults so a malformed
        // settings entry never blocks the server from starting.
        let settings: ServerSettings = lsp_settings
            .as_ref()
            .and_then(|s| s.settings.clone())
            .map(|value| match serde_json::from_value(value) {
                Ok(parsed) => parsed,
                Err(err) => {
                    eprintln!(
                        "[mlir-suite] failed to parse lsp.{}.settings, \
                         using defaults: {err}",
                        self.id(),
                    );
                    ServerSettings::default()
                }
            })
            .unwrap_or_default();

        // --- binary path: binary.path > settings.path > $PATH ---
        let command = match user_binary.and_then(|b| b.path.clone()) {
            Some(path) => path,
            None => match settings.path {
                Some(ref path) => path.clone(),
                None => self.resolve_from_path(worktree)?,
            },
        };

        let mut args = user_binary
            .and_then(|b| b.arguments.clone())
            .unwrap_or_default();

        // --- compilation database ---
        if let Some(flag) = self.compilation_db_flag() {
            let already_set = args.iter().any(|a| a.starts_with(flag));
            if !already_set {
                let db_path = settings.compilation_database.or_else(|| {
                    self.compilation_db_filename()
                        .and_then(|f| detect_compilation_db(worktree, f))
                });
                if let Some(path) = db_path {
                    args.push(format!("{flag}={path}"));
                }
            }
        }

        // --- extra include directories ---
        if let Some(flag) = self.extra_dir_flag() {
            for dir in &settings.extra_dirs {
                args.push(format!("{flag}={dir}"));
            }
        }

        // --- common flags: log, pretty ---
        if let Some(ref level) = settings.log {
            if !args.iter().any(|a| a.starts_with("--log")) {
                args.push(format!("--log={level}"));
            }
        }
        if settings.pretty == Some(true) && !args.iter().any(|a| a == "--pretty") {
            args.push("--pretty".to_string());
        }

        // Start with the worktree's shell environment and overlay
        // user-specified env vars from `binary.env` on top.
        let mut env = worktree.shell_env();
        if let Some(user_env) = user_binary.and_then(|b| b.env.clone()) {
            for (k, v) in user_env {
                env.retain(|(ek, _)| ek != &k);
                env.push((k, v));
            }
        }

        Ok(zed::Command { command, args, env })
    }

    /// Look up the server binary on `$PATH`.
    fn resolve_from_path(&self, worktree: &Worktree) -> Result<String> {
        worktree.which(self.default_binary()).ok_or_else(|| {
            format!(
                "`{}` not found. Install it from the LLVM project \
                 (`cmake --build . --target {}`) and either add it to \
                 your $PATH or set `lsp.{}.binary.path` in settings.json.",
                self.default_binary(),
                self.default_binary(),
                self.id()
            )
        })
    }
}

/// Create a boxed [`LanguageServer`] from its Zed language-server ID.
pub fn from_id(id: &str) -> Result<Box<dyn LanguageServer>> {
    match id {
        "mlir-lsp-server" => Ok(Box::new(MlirServer)),
        "mlir-pdll-lsp-server" => Ok(Box::new(PdllServer)),
        "tblgen-lsp-server" => Ok(Box::new(TablegenServer)),
        other => Err(format!("unknown language server id: {other}")),
    }
}

/// Probe [`BUILD_DIR_CANDIDATES`] for a compilation-database file and return
/// its absolute path if found.
fn detect_compilation_db(worktree: &Worktree, filename: &str) -> Option<String> {
    let root = worktree.root_path();
    for dir in BUILD_DIR_CANDIDATES {
        let relative = format!("{dir}/{filename}");
        // `read_text_file` succeeds only when the file exists and is readable.
        if worktree.read_text_file(&relative).is_ok() {
            return Some(format!("{root}/{relative}"));
        }
    }
    None
}
