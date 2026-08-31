# Zed MLIR

[![EN](https://img.shields.io/badge/lang-EN-blue?style=flat-square)](README.md)
[![中文](https://img.shields.io/badge/lang-中文-lightgrey?style=flat-square)](docs/README_ZH.md)

[![CI](https://img.shields.io/github/actions/workflow/status/felixtensor/zed-mlir/ci.yml?style=flat-square&logo=githubactions&logoColor=white&label=CI)](https://github.com/felixtensor/zed-mlir/actions/workflows/ci.yml)
[![Version](https://img.shields.io/github/v/tag/felixtensor/zed-mlir?style=flat-square&logo=github&label=version)](https://github.com/felixtensor/zed-mlir/tags)
[![License](https://img.shields.io/badge/license-Apache%202.0%20with%20LLVM%20Exceptions-blue?style=flat-square&logo=apache&logoColor=white)](LICENSE)
[![Stars](https://img.shields.io/github/stars/felixtensor/zed-mlir?style=flat-square&logo=github)](https://github.com/felixtensor/zed-mlir/stargazers)

[MLIR](https://mlir.llvm.org), [TableGen](https://llvm.org/docs/TableGen/), and [PDLL](https://mlir.llvm.org/docs/PDLL/) support for the [Zed](https://zed.dev) editor.

## Features

- **Tree-sitter grammars** for MLIR (`.mlir`), TableGen (`.td`), and PDLL (`.pdll`) — the pinned MLIR parser parses a [curated corpus of official MLIR test files](https://github.com/felixtensor/tree-sitter-mlir/blob/main/examples/README.md) without `ERROR` nodes; this extension's Zed queries provide highlighting on top of those syntax trees.
- **MLIR inside C++ raw strings** — when Zed's bundled C++ grammar injects `raw_string_content` by delimiter, MLIR inside `R"mlir(…)mlir"` strings is highlighted using this extension's MLIR grammar.
- **First-class custom dialect support** — user-defined or out-of-tree `dialect.op` forms are recognized and highlighted correctly, so your project's own dialects just work.
- **Symbol outline** — navigate symbols in MLIR, TableGen, and PDLL from the outline panel.
- **Language Server integration** for all three upstream LLVM servers:
  - `mlir-lsp-server` for `.mlir`
  - `mlir-pdll-lsp-server` for `.pdll`
  - `tblgen-lsp-server` for `.td`
- **Rich completion labels** — completion items from the MLIR and PDLL language servers are classified by their LSP kind and detail, so values, blocks, dialects, operations, types, attributes, constraints, and include paths are colored consistently.
- **Editing ergonomics** — bracket matching, auto-close pairs, and indentation tuned for each language.
- **Vim text objects** — function and comment objects in all three languages, class objects where the grammar has a matching construct, and the motions built on them.

## Prerequisites

- [Zed](https://zed.dev/download) editor
- (Optional) LLVM language servers for LSP features — see [Language Support](#language-support).

## Installation

Open the extension gallery with `Cmd+Shift+X` on macOS or `Ctrl+Shift+X` on Linux/Windows — or **Zed > Extensions** from the menu bar — then search for **MLIR** and click **Install**. Nothing else is required: no Rust toolchain, and no LLVM binaries.

### Install as a dev extension

To run a local checkout instead — to try an unreleased change, or to work on the extension itself — clone the repository:

```bash
git clone https://github.com/felixtensor/zed-mlir.git
```

Then open the command palette (`Cmd+Shift+P` on macOS, `Ctrl+Shift+P` on Linux/Windows) and run **`zed: install dev extension`**, selecting the cloned directory. Zed compiles the extension to WebAssembly on install, so this path needs a stable Rust toolchain from [rustup](https://rustup.rs). The first build fetches dependencies and may take a minute or two; **`zed: open log`** has the details if it fails.

See [CONTRIBUTING.md](CONTRIBUTING.md) for the development setup, and the Zed docs on [Developing Extensions](https://zed.dev/docs/extensions/developing-extensions) for how extensions are structured.

## Language Support

### Core editing

No LLVM binaries are required. To keep Tree-sitter highlighting, symbol outlines, bracket matching, indentation, and Vim text objects without LSP features, disable language servers in your user settings or project `.zed/settings.json`:

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

Install `mlir-lsp-server`, `mlir-pdll-lsp-server`, and `tblgen-lsp-server` to enable the LSP features supported by each language. Configure each server's binary path in Zed; alternatively, make the containing directory available on the worktree's `$PATH`. See [Language Server Setup](docs/LANGUAGE_SERVER.md#configure-lsp) for the capability matrix, build instructions, settings, and [SSH remote development](docs/REMOTE_DEVELOPMENT.md).

### Vim mode

In Vim mode, `af` / `if` and `ac` / `ic` select functions and classes, `gc` takes the surrounding run of comments, and `]m` moves between functions:

| Language | Function (`af` / `if`, `]m`) | Class (`ac` / `ic`) |
| --- | --- | --- |
| MLIR | `func.func`, `llvm.func` | `module`, `builtin.module` |
| PDLL | top-level `Pattern`, `Constraint`, `Rewrite` | — |
| TableGen | named `def`, `defm` | `class`, `multiclass` |

See [Vim Mode](docs/VIM_MODE.md) for how each mapping follows the grammar's node shapes, and for the motion-depth limit in MLIR.

## Screenshots

### Out-of-Tree Dialect Highlighting

![Out-of-Tree Dialect Highlighting](https://raw.githubusercontent.com/felixtensor/zed-mlir/assets/screenshots/downstream-triton.png)

### MLIR in C++ Raw Strings

![MLIR in C++ Raw Strings](https://raw.githubusercontent.com/felixtensor/zed-mlir/assets/screenshots/cpp-inject-mlir.png)

### Go to Definition

![Go to Definition](https://raw.githubusercontent.com/felixtensor/zed-mlir/assets/screenshots/go-to-definition.gif)

### Find References

![Find References](https://raw.githubusercontent.com/felixtensor/zed-mlir/assets/screenshots/find-references.gif)

### Hover / Signature

![Hover / Signature](https://raw.githubusercontent.com/felixtensor/zed-mlir/assets/screenshots/hover.gif)

### Completion

![Completion](https://raw.githubusercontent.com/felixtensor/zed-mlir/assets/screenshots/completion.gif)

### Diagnostics

![Diagnostics](https://raw.githubusercontent.com/felixtensor/zed-mlir/assets/screenshots/diagnostics.gif)

### Symbol Outline

![Symbol Outline](https://raw.githubusercontent.com/felixtensor/zed-mlir/assets/screenshots/outline.gif)

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

Development priorities are tracked in the [roadmap](docs/ROADMAP.md). See [CONTRIBUTING.md](CONTRIBUTING.md) for issue reporting, development, and validation guidance.

- Use the [issue chooser](https://github.com/felixtensor/zed-mlir/issues/new/choose) to report a bug, request a feature, or ask a setup question.
- Follow the [pull request guidance](CONTRIBUTING.md#pull-requests) when submitting a change.

## License

Apache License 2.0 with LLVM Exceptions.
