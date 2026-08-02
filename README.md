# zed-mlir-suite

[![EN](https://img.shields.io/badge/lang-EN-blue?style=flat-square)](README.md)
[![中文](https://img.shields.io/badge/lang-中文-lightgrey?style=flat-square)](docs/README_ZH.md)

[![CI](https://img.shields.io/github/actions/workflow/status/felixtensor/zed-mlir-suite/ci.yml?style=flat-square&logo=githubactions&logoColor=white&label=CI)](https://github.com/felixtensor/zed-mlir-suite/actions/workflows/ci.yml)
[![Version](https://img.shields.io/github/v/tag/felixtensor/zed-mlir-suite?style=flat-square&logo=github&label=version)](https://github.com/felixtensor/zed-mlir-suite/tags)
[![License](https://img.shields.io/badge/license-Apache%202.0%20with%20LLVM%20Exceptions-blue?style=flat-square&logo=apache&logoColor=white)](LICENSE)
[![Stars](https://img.shields.io/github/stars/felixtensor/zed-mlir-suite?style=flat-square&logo=github)](https://github.com/felixtensor/zed-mlir-suite/stargazers)

[MLIR](https://mlir.llvm.org), [TableGen](https://llvm.org/docs/TableGen/), and [PDLL](https://mlir.llvm.org/docs/PDLL/) support for the [Zed](https://zed.dev) editor.

## Features

- **Tree-sitter grammars** for MLIR (`.mlir`), TableGen (`.td`), and PDLL (`.pdll`) — the pinned MLIR parser parses a [corpus of 600 official MLIR test files across 24 dialect directories](https://github.com/felixtensor/tree-sitter-mlir/blob/06a0f9237dd3166e2021090e6d30ca08fb13c8e3/examples/README.md) without `ERROR` nodes; MLIR Suite's Zed queries provide highlighting on top of those syntax trees.
- **MLIR inside C++ raw strings** — when Zed's bundled C++ grammar injects `raw_string_content` by delimiter, MLIR inside `R"mlir(…)mlir"` strings is highlighted using this extension's MLIR grammar.
- **First-class custom dialect support** — user-defined or out-of-tree `dialect.op` forms are recognized and highlighted correctly, so your project's own dialects just work.
- **Symbol outline** — navigate symbols in MLIR, TableGen, and PDLL from the outline panel.
- **Language Server integration** for all three upstream LLVM servers:
  - `mlir-lsp-server` for `.mlir`
  - `mlir-pdll-lsp-server` for `.pdll`
  - `tblgen-lsp-server` for `.td`
- **Rich completion labels** — completion items from the MLIR and PDLL language servers are classified by their LSP kind and detail, so values, blocks, dialects, operations, types, attributes, constraints, and include paths are colored consistently.
- **Editing ergonomics** — bracket matching, auto-close pairs, and indentation tuned for each language.

## Prerequisites

- [Zed](https://zed.dev/download) editor
- (Optional) LLVM language servers for LSP features — see [Language Support](#language-support).

## Installation

This extension is installed as a Zed **dev extension**: clone the repository, then point Zed at the directory. Zed compiles the extension to WebAssembly on first install, so a local Rust toolchain is required.

### Install the Rust toolchain

Install Rust via [rustup](https://rustup.rs) (stable). On macOS / Linux:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

On Windows, download and run `rustup-init.exe` from [rustup.rs](https://rustup.rs).

Rust must be installed through `rustup`; Zed handles the required WebAssembly target automatically when it builds the dev extension.

### Clone the repository

```bash
git clone https://github.com/felixtensor/zed-mlir-suite.git
```

### Install as a dev extension

In Zed, open the command palette (`Cmd+Shift+P` on macOS, `Ctrl+Shift+P` on Linux/Windows) and run **`zed: install dev extension`** — or open **Extensions** (`Cmd+Shift+X` / `Ctrl+Shift+X`) and click **Install Dev Extension**. Select the cloned directory.

Zed builds the extension on install; the first build fetches dependencies and may take a minute or two. If the build fails, run **`zed: open log`** to inspect `Zed.log` for details.

## Language Support

### Core editing

No LLVM binaries are required. To keep Tree-sitter highlighting, symbol outlines, bracket matching, and indentation without LSP features, disable language servers in your user settings or project `.zed/settings.json`:

```jsonc
{
  "languages": {
    "MLIR": { "enable_language_server": false },
    "PDLL": { "enable_language_server": false },
    "TableGen": { "enable_language_server": false }
  }
}
```

See [Language Server Setup](docs/LANGUAGE_SERVER.md#disable-lsp) for details.

### LSP code intelligence

Install `mlir-lsp-server`, `mlir-pdll-lsp-server`, and `tblgen-lsp-server` to enable the LSP features supported by each language. Configure each server's binary path in Zed; alternatively, make the containing directory available on the worktree's `$PATH`. See [Language Server Setup](docs/LANGUAGE_SERVER.md#configure-lsp) for the capability matrix, build instructions, settings, and SSH remote development.

## Screenshots

### Out-of-Tree Dialect Highlighting

![Out-of-Tree Dialect Highlighting](https://raw.githubusercontent.com/felixtensor/zed-mlir-suite/assets/screenshots/downstream-triton.png)

### MLIR in C++ Raw Strings

![MLIR in C++ Raw Strings](https://raw.githubusercontent.com/felixtensor/zed-mlir-suite/assets/screenshots/cpp-inject-mlir.png)

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

The out-of-tree dialect screenshot uses a TritonGPU test file from [Triton](https://github.com/triton-lang/triton).

For MLIR tooling in other editors, see:

- [vscode-mlir](https://github.com/llvm/vscode-mlir) — official VS Code extension for MLIR, PDLL, and TableGen.
- [mlir-mode](https://github.com/llvm/llvm-project/tree/main/mlir/utils/emacs) — Emacs major mode and LSP client, shipped in the LLVM monorepo.

## Feedback & Contributions

Development priorities are tracked in the [roadmap](docs/ROADMAP.md). See [CONTRIBUTING.md](CONTRIBUTING.md) for issue reporting, development, and validation guidance. Participation is governed by the [Code of Conduct](CODE_OF_CONDUCT.md).

- Use the [issue chooser](https://github.com/felixtensor/zed-mlir-suite/issues/new/choose) to report a bug, request a feature, or ask a setup question.
- Follow the [pull request guidance](CONTRIBUTING.md#pull-requests) when submitting a change.

## License

Apache License 2.0 with LLVM Exceptions.
