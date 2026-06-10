# zed-mlir-suite

[![EN](https://img.shields.io/badge/lang-EN-blue?style=flat-square)](README.md)
[![中文](https://img.shields.io/badge/lang-中文-lightgrey?style=flat-square)](docs/README_ZH.md)

[![CI](https://img.shields.io/github/actions/workflow/status/felixtensor/zed-mlir-suite/ci.yml?style=flat-square&logo=githubactions&logoColor=white&label=CI)](https://github.com/felixtensor/zed-mlir-suite/actions/workflows/ci.yml)
[![Version](https://img.shields.io/github/v/tag/felixtensor/zed-mlir-suite?style=flat-square&logo=github&label=version)](https://github.com/felixtensor/zed-mlir-suite/tags)
[![License](https://img.shields.io/badge/license-Apache%202.0%20with%20LLVM%20Exceptions-blue?style=flat-square&logo=apache&logoColor=white)](LICENSE)
[![Stars](https://img.shields.io/github/stars/felixtensor/zed-mlir-suite?style=flat-square&logo=github)](https://github.com/felixtensor/zed-mlir-suite/stargazers)

[MLIR](https://mlir.llvm.org), TableGen, and PDLL support for the [Zed](https://zed.dev) editor.

## Features

- **Tree-sitter grammars** for MLIR (`.mlir`), TableGen (`.td`), and PDLL (`.pdll`) — 100% pass rate on 403 official MLIR test files across 11 core dialects, meaning virtually all valid MLIR syntax is highlighted correctly out of the box.
- **MLIR inside C++ raw strings** — embedded MLIR is highlighted inside C++ `R"mlir(…)mlir"` strings when Zed's C++ raw-string injection support is available.
- **First-class custom dialect support** — user-defined or out-of-tree `dialect.op` forms are recognized and highlighted correctly, so your project's own dialects just work.
- **Symbol outline** — navigate symbols in MLIR, TableGen, and PDLL from the outline panel.
- **Language Server integration** for all three upstream LLVM servers:
  - `mlir-lsp-server` for `.mlir`
  - `mlir-pdll-lsp-server` for `.pdll`
  - `tblgen-lsp-server` for `.td`
- **Editing ergonomics** — bracket matching, auto-close pairs, and indentation tuned for each language.

## Prerequisites

- [Zed](https://zed.dev) editor
- (Optional) LLVM language servers for LSP features — see [Language Server Setup](#language-server-setup).

## Installation

This extension is installed as a Zed **dev extension**: clone the repository, then point Zed at the directory. Zed compiles the extension to WebAssembly on first install, so a local Rust toolchain is required.

### 1. Install the Rust toolchain

Install Rust via [rustup](https://rustup.rs) (stable). On macOS / Linux:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

On Windows, download and run `rustup-init.exe` from [rustup.rs](https://rustup.rs).

Then add the WebAssembly target Zed extensions compile against:

```bash
rustup target add wasm32-wasip2
```

Verify the toolchain is reachable from the same shell Zed is launched from (Zed inherits its `PATH`):

```bash
cargo --version
rustup target list --installed
```

`wasm32-wasip2` must appear in the installed-targets list.

### 2. Clone the repository

```bash
git clone https://github.com/felixtensor/zed-mlir-suite.git
```

### 3. Install as a dev extension

In Zed, open the command palette (`Cmd+Shift+P` on macOS, `Ctrl+Shift+P` on Linux/Windows) and run **`zed: install dev extension`** — or open **Extensions** (`Cmd+Shift+X` / `Ctrl+Shift+X`) and click **Install Dev Extension**. Select the cloned directory.

Zed builds the extension on install; the first build fetches dependencies and may take a minute or two. If the build fails with a missing-target error, re-run `rustup target add wasm32-wasip2` and reinstall.

## Language Server Setup

### Building the servers

The three servers live in the `llvm-project` monorepo under `mlir/tools/`. Follow the [official MLIR Getting Started guide](https://mlir.llvm.org/getting_started/) to build them; a typical Unix-like flow is:

```bash
git clone https://github.com/llvm/llvm-project.git
mkdir llvm-project/build && cd llvm-project/build

cmake -G Ninja ../llvm \
  -DLLVM_ENABLE_PROJECTS=mlir \
  -DLLVM_TARGETS_TO_BUILD="Native" \
  -DCMAKE_BUILD_TYPE=Release \
  -DLLVM_ENABLE_ASSERTIONS=ON

cmake --build . --target mlir-lsp-server mlir-pdll-lsp-server tblgen-lsp-server
```

After a successful build, the binaries land in `llvm-project/build/bin/`. Either add that directory to your `$PATH`, or point each server at its absolute path via `settings.json` (see below).

> Tip: if `mlir` is listed in `LLVM_ENABLE_PROJECTS` and you build the default `all` target, the three LSP servers are produced along with the rest of MLIR — no separate invocation needed.

### Configuration

Each server is configured under `lsp.<server-id>.settings` in Zed's `settings.json`. The extension looks for the server binary via `settings.path`, then falls back to `$PATH`.

| Language | Server id |
|---|---|
| MLIR | `mlir-lsp-server` |
| PDLL | `mlir-pdll-lsp-server` |
| TableGen | `tblgen-lsp-server` |

#### Available settings

| Field | Type | Applies to | Description |
|---|---|---|---|
| `path` | `string` | all | Path to the server binary |
| `compilation_database` | `string` | tblgen, pdll | Path to the compilation-database YAML |
| `extra_dirs` | `string[]` | tblgen, pdll | Extra include directories |
| `log` | `string` | all | Log verbosity: `"error"`, `"info"`, or `"verbose"` |
| `pretty` | `bool` | all | Pretty-print JSON output |

All fields are optional. The extension auto-detects compilation-database files in your workspace; if detection succeeds, no manual configuration is needed.

#### SSH remote development

When using Zed's SSH remote development, language servers run on the remote server. Configure MLIR Suite server binary paths with `zed: open server settings`, not `zed: open settings file`; the latter edits the local machine's settings. See [SSH remote development](docs/REMOTE_DEVELOPMENT.md) for the recommended settings layout.

#### Example

```jsonc
{
  "lsp": {
    "mlir-lsp-server": {
      "settings": {
        "path": "/path/to/mlir-lsp-server",
        "log": "verbose"
      }
    },
    "tblgen-lsp-server": {
      "settings": {
        "path": "/path/to/tblgen-lsp-server",
        "compilation_database": "/path/to/build/tablegen_compile_commands.yml",
        "extra_dirs": [
          "/path/to/llvm-project/llvm/include",
          "/path/to/llvm-project/mlir/include"
        ]
      }
    },
    "mlir-pdll-lsp-server": {
      "settings": {
        "path": "/path/to/mlir-pdll-lsp-server",
        "compilation_database": "/path/to/build/pdll_compile_commands.yml",
        "extra_dirs": [
          "/path/to/llvm-project/mlir/include"
        ]
      }
    }
  }
}
```

> Zed's native `binary.path` and `binary.arguments` fields still work and take precedence when set.

After changing settings, open the command palette (`Cmd+Shift+P` on macOS, `Ctrl+Shift+P` on Linux/Windows) and run `zed: restart language server` to apply them.

## Screenshots

### Go to Definition

![Go to Definition](https://raw.githubusercontent.com/felixtensor/zed-mlir-suite/assets/screenshots/go-to-definition.gif)

### Find References

![Find References](https://raw.githubusercontent.com/felixtensor/zed-mlir-suite/assets/screenshots/find-references.gif)

### Hover / Signature

![Hover / Signature](https://raw.githubusercontent.com/felixtensor/zed-mlir-suite/assets/screenshots/hover.gif)

### Completion

![Completion](https://raw.githubusercontent.com/felixtensor/zed-mlir-suite/assets/screenshots/completion.gif)

### Diagnostics

![Diagnostics](https://raw.githubusercontent.com/felixtensor/zed-mlir-suite/assets/screenshots/diagnostics.gif)

### Symbol Outline

![Symbol Outline](https://raw.githubusercontent.com/felixtensor/zed-mlir-suite/assets/screenshots/outline.gif)

## Acknowledgements

This extension builds on:

- [MLIR](https://mlir.llvm.org) — the multi-level intermediate representation framework from the LLVM project.
- [tree-sitter-mlir](https://github.com/felixtensor/tree-sitter-mlir) — Tree-sitter grammar for MLIR.
- [tree-sitter-tablegen](https://github.com/felixtensor/tree-sitter-tablegen) — Tree-sitter grammar for TableGen.
- [tree-sitter-pdll](https://github.com/felixtensor/tree-sitter-pdll) — Tree-sitter grammar for PDLL.
- The three LSP servers (`mlir-lsp-server`, `mlir-pdll-lsp-server`, `tblgen-lsp-server`) are part of the [LLVM project](https://github.com/llvm/llvm-project).

For MLIR tooling in other editors, see:

- [vscode-mlir](https://github.com/llvm/vscode-mlir) — official VS Code extension for MLIR, PDLL, and TableGen.
- [mlir-mode](https://github.com/llvm/llvm-project/tree/main/mlir/utils/emacs) — Emacs major mode and LSP client, shipped in the LLVM monorepo.

## Feedback & Contributions

This extension is actively developed. See the [roadmap](docs/ROADMAP.md) for what's planned next. Contributions of all sizes are welcome — bug reports, feature requests, and pull requests:

- [Open an issue](https://github.com/felixtensor/zed-mlir-suite/issues)
- Submit a pull request with your improvements

## License

Apache License 2.0 with LLVM Exceptions.
