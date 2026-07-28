# Roadmap

This document tracks the planned direction of **MLIR Suite**. Core language and LSP integration is in place, so remaining work favors focused parser and query maintenance over broad feature expansion. Items are grouped by priority, not by commitment — timing depends on upstream LLVM releases, Zed extension-API surface, concrete user feedback, and available time. Nothing here is a promise.

For shipped milestones and tag-by-tag release history, see [CHANGELOG.md](../CHANGELOG.md).

## Near-term

### v0.6.x follow-ups

- ~~**TableGen `[{ ... }]` auto-close.** Test a config-only entry first (`{ start = "[{", end = "}]", close = true, newline = true }`); this controls insertion, independently of bracket matching.~~ **Closed:** the experiment is complete, but the pair is deliberately not shipped without concrete editing demand.
- ~~**TableGen `[{ ... }]` matching, if needed.** The pinned grammar merges each delimiter into an anonymous token that `brackets.scm` cannot address. Only pursue named open/close nodes through the parser branch/push/pin workflow if multi-character matching proves useful after the auto-close experiment.~~ **Closed with auto-close:** matching remains unshipped unless concrete demand justifies both the editor behavior and parser churn.
- ~~**Current-Zed compatibility pass.** Re-test PDLL/TableGen document links with `lsp_document_links` enabled, verify PDLL inlay hints and completion-label rendering with stock themes, and separately reproduce the historical TableGen cross-file definition issue with valid compilation databases / `extra_dirs` and protocol logs.~~ **Completed on Zed 1.12.0:** all paths worked as expected; no extension-side follow-up was required.
- **Query smoke tests against the pinned grammars.** Suite CI checks the Rust extension but never compiles `languages/*/*.scm` against the exact grammar SHAs in `extension.toml`, so a grammar bump can leave a stale node name unnoticed. Before the next pin update, add a lightweight gate that uses temporary checkouts of the public pins, compiles every query, and parses representative fixtures without committing grammar checkouts or duplicating upstream corpora.
- **MLIR alias outline support.** Add stable `name:` fields for `#` / `!` alias definitions on a non-`main` `tree-sitter-mlir` branch, test, commit, and push the change, then pin that public SHA before updating `outline.scm`. Keep external resources out of the outline because they have no stable symbol name.

## Mid-term

- **Vim text objects (`textobjects.scm`).** Add `@function.around` / `@function.inside`, `@class.around` / `@class.inside`, and `@comment.around` captures with language-specific semantics: `func.func` as a function, MLIR modules and selected stable regions as larger class-like sections, TableGen `class` / named `def` records as sections, and PDLL `Pattern` / `Constraint` / `Rewrite` as top-level function-like sections. Do not treat every MLIR region or block as a class-level object unless testing shows the motions stay useful.
- **TableGen grammar — broaden corpus coverage.** The current real-world gate covers 141 MLIR and 7 LLVM TableGen files. Keep extending and validating [`felixtensor/tree-sitter-tablegen`](https://github.com/felixtensor/tree-sitter-tablegen) against four corpora and gate version bumps on zero `ERROR` nodes:
  - **MLIR TableGen** — dialect / op / pass definitions under `mlir/include/mlir/` and `mlir/test/`
  - **LLVM TableGen** — target backends under `llvm/lib/Target/*/` (heavy use of intrinsics, patterns, register classes)
  - **Clang TableGen** — `clang/include/clang/Basic/{Attr,Diagnostic,StmtNodes,…}.td`
  - **LLDB TableGen** — command option definitions under `lldb/source/Commands/Options.td` and related

  Scope the zero-`ERROR` gate to curated, valid source corpora; deliberately invalid diagnostic tests should be tracked separately.

## Ideas (unscored)

- Lit-aware `// RUN:` highlighting and per-line runnables. Comments are single tokens in all three grammars, so faithfully highlighting lit substitutions (`%s`, `%t`, `%{...}`) would require a dedicated tree-sitter lit grammar injected into comment content — injecting generic `shellscript` misrepresents lit syntax — and executing a single `RUN:` line requires lit substitution plus test-suite configuration. A project-specific whole-file wrapper covers the practical workflow in the meantime; revisit only if a maintained lit grammar appears or user demand justifies owning one.
- Quick-fix for "missing `include`" in TableGen (auto-insert the canonical header path), most likely as an upstream `tblgen-lsp-server` code action rather than a Zed-only feature.
- Dialect-aware highlighting inside MLIR string attributes that embed recognized DSLs. Keep this opt-in / whitelist-driven so ordinary MLIR string attributes are not over-highlighted.
- Semantic-token defaults, if one of the LLVM servers begins advertising useful semantic token types.
- Custom symbol labels, if a future `label_for_symbol` API exposes richer data than the current symbol kind and name.
- Block folding driven by tree-sitter regions. Zed currently derives folds from indentation and does not document a `folds.scm` capability for extensions; revisit if/when one is exposed.

## Out of scope (today)

- **`.mlirbc` bytecode editor.** `vscode-mlir` implements this with a custom editor, virtual file system, and custom requests that the current Zed extension API does not expose. Do not register `.mlirbc` as normal text.
- **PDLL intermediate output.** `vscode-mlir` uses the custom `pdll/viewOutput` request, an output-kind picker, and temporary editor documents to show the AST, generated MLIR, or C++ output. Zed and `zed_extension_api` do not currently expose the custom-request and editor-command surfaces needed to reproduce this workflow; revisit only if both add the necessary support.
- **Extension-side LSP response synthesis.** Diagnostics, navigation, inlay hints, and code actions come from the LLVM servers and are handled by Zed core. New standard responses belong upstream in LLVM; revisit suite work only when they require launch/configuration integration or reveal a concrete Zed compatibility issue.
- **LSP initialization / workspace-configuration migration.** The current LLVM servers do not consume compilation-database or extra-include settings through these LSP channels, so CLI flags remain the supported path until upstream behavior changes.
- **LSP settings-change behaviour.** Zed documents initialization options as startup-time configuration that requires a language-server restart to reapply. The extension API does not expose hooks to intercept settings changes or present custom restart prompts, so finer-grained control (prompt / auto-restart / ignore) is not implementable by extensions today.

## How to propose changes

Open an issue at [felixtensor/zed-mlir-suite](https://github.com/felixtensor/zed-mlir-suite/issues) with:
- What problem you hit or what workflow you want,
- Any pointers to upstream LLVM docs or related issues.
