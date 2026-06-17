use serde::Deserialize;
use zed_extension_api::{self as zed, serde_json, settings::LspSettings, Result, Worktree};

/// Structured `lsp.<server-id>.settings` options shared by LLVM LSP servers.
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
/// Unknown-to-server fields are parsed but ignored by that server.
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

/// Compilation-database search roots, relative to the worktree root.
const BUILD_DIR_CANDIDATES: &[&str] = &["build", "out"];

/// Per-server differences used by the shared command resolver.
///
/// The default `resolve_command` handles settings lookup, binary resolution,
/// environment merging, and optional compilation-database discovery.
pub trait LanguageServer {
    /// Language-server ID used by Zed and `settings.json`.
    fn id(&self) -> &'static str;

    /// Binary name to probe on `$PATH` when settings do not override it.
    fn default_binary(&self) -> &'static str;

    /// Compilation-database flag prefix, without `=`.
    fn compilation_db_flag(&self) -> Option<&'static str> {
        None
    }

    /// Compilation-database filename to probe under `BUILD_DIR_CANDIDATES`.
    fn compilation_db_filename(&self) -> Option<&'static str> {
        None
    }

    /// Extra include-directory flag prefix, without `=`.
    fn extra_dir_flag(&self) -> Option<&'static str> {
        None
    }

    /// Resolve the command, with `binary.arguments` taking full flag ownership.
    fn resolve_command(&self, worktree: &Worktree) -> Result<zed::Command> {
        let lsp_settings = LspSettings::for_worktree(self.id(), worktree).ok();

        let user_binary = lsp_settings.as_ref().and_then(|s| s.binary.as_ref());

        // Downgrade malformed structured settings so a bad optional field does
        // not prevent the language server from starting.
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

        // Structured/auto-detected flags are skipped when binary.arguments
        // already owns that flag family.
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

        if let Some(flag) = self.extra_dir_flag() {
            for dir in &settings.extra_dirs {
                args.push(format!("{flag}={dir}"));
            }
        }

        if let Some(ref level) = settings.log {
            if !args.iter().any(|a| a.starts_with("--log")) {
                args.push(format!("--log={level}"));
            }
        }
        if settings.pretty == Some(true) && !args.iter().any(|a| a == "--pretty") {
            args.push("--pretty".to_string());
        }

        // Overlay explicit env settings without duplicating keys.
        let mut env = worktree.shell_env();
        if let Some(user_env) = user_binary.and_then(|b| b.env.clone()) {
            for (k, v) in user_env {
                env.retain(|(ek, _)| ek != &k);
                env.push((k, v));
            }
        }

        Ok(zed::Command { command, args, env })
    }

    /// Look up the default binary on `$PATH`, with an installation hint.
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

/// Instantiate the integration for a Zed language-server ID.
pub fn from_id(id: &str) -> Result<Box<dyn LanguageServer>> {
    match id {
        MlirServer::SERVER_ID => Ok(Box::new(MlirServer)),
        PdllServer::SERVER_ID => Ok(Box::new(PdllServer)),
        TablegenServer::SERVER_ID => Ok(Box::new(TablegenServer)),
        other => Err(format!("unknown language server id: {other}")),
    }
}

/// Return the first detected compilation database as an absolute path.
fn detect_compilation_db(worktree: &Worktree, filename: &str) -> Option<String> {
    let root = worktree.root_path();
    for dir in BUILD_DIR_CANDIDATES {
        let relative = format!("{dir}/{filename}");
        if worktree.read_text_file(&relative).is_ok() {
            return Some(format!("{root}/{relative}"));
        }
    }
    None
}
