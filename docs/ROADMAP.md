# Roadmap

This document tracks the planned direction of **MLIR Suite**. Items are grouped by priority, not by commitment — timing depends on upstream LLVM releases, Zed extension-API surface, user feedback, and available time. Nothing here is a promise.

For shipped milestones and tag-by-tag release history, see [CHANGELOG.md](../CHANGELOG.md).

## Near-term

### v0.6.x follow-ups

- **Investigate lit-aware highlighting for `// RUN:` lines.** MLIR / PDLL / TableGen tests commonly use LLVM lit `RUN:` directives, but those directives are not plain bash: they support lit substitutions (`%s`, `%t`, `%{...}`, `%(line)`), continuations, and test-suite-specific tool substitution. Explore whether `highlights.scm` / `injections.scm` can highlight command-like structure without misrepresenting lit syntax as generic shell. Prefer explicit lit substitution captures and only inject `shellscript` into command spans if it behaves correctly on real LLVM / MLIR tests.
- **Investigate lit-aware runnables for tests.** Zed `runnables.scm` can surface runnable syntax, but executing a single `RUN:` line correctly requires lit substitution and test-suite configuration, not just passing the comment payload to a shell. Start with a runnable for the whole current lit test file (for example, `llvm-lit $ZED_FILE`) and evaluate whether a small lit-aware helper is needed before exposing individual `RUN:` pipelines.
- ~~**Verify MLIR-in-C++ raw strings.** Zed's bundled C++ grammar already injects `raw_string_content` using the delimiter as `@injection.language`. Since this extension registers `mlir`, the injection should "just work" inside `R"mlir(…)mlir"`, but it has not been confirmed end-to-end. Verify with a real `.cpp` file and document the result; if the wiring is missing, file an upstream issue.~~
- **Continued highlight & query polish.** Steady, focused improvements across the three grammars and capability files as edge cases surface.

## Mid-term

- **Vim text objects (`textobjects.scm`).** Add `@function.around` / `@function.inside`, `@class.around` / `@class.inside`, and `@comment.around` captures with language-specific semantics: `func.func` as a function, MLIR modules / regions as larger class-like sections, TableGen `class` / named `def` records as sections, and PDLL `Pattern` / `Constraint` / `Rewrite` as top-level function-like sections. Do not treat every MLIR block as a class-level object unless testing shows the motions stay useful.
- **Investigate LSP initialization / workspace configuration.** The extension currently translates settings into CLI flags, which is the documented path for the MLIR-related servers. The Zed extension API exposes `language_server_initialization_options` and `language_server_workspace_configuration`; verify whether `mlir-lsp-server`, `mlir-pdll-lsp-server`, and `tblgen-lsp-server` actually consume compilation-database / extra-include configuration over LSP before migrating anything. Keep CLI flags as the stable path unless upstream support is confirmed.
- **Custom symbol labels.** Implement `label_for_symbol` where Zed's API can improve readability of MLIR symbols — e.g. distinguish dialect prefixes and render attribute/type details compactly. Treat UI column alignment as best-effort because final layout is controlled by Zed.
- **Semantic-token defaults.** If `mlir-lsp-server` reports semantic tokens with useful custom token types, ship a `semantic_token_rules.json` with sensible defaults so Zed's `combined` semantic-token mode layers cleanly on top of the tree-sitter highlights without users having to write rules by hand.
- **Inlay hints.** Verify and document PDLL inlay hints from `mlir-pdll-lsp-server`; add MLIR inlay hints for SSA value types and block-argument types only if / when `mlir-lsp-server` supports `textDocument/inlayHint`.
- **`.mlirbc` bytecode support.** Track Zed and `mlir-lsp-server` support for opening MLIR bytecode as editable text. `mlir-lsp-server` can inspect bytecode, but the extension still needs a Zed-side way to hand a binary `.mlirbc` buffer to the server as an editable text document; do not simply register `.mlirbc` as normal text unless that path is verified.
- **TableGen grammar — broaden corpus coverage.** Beyond the focused navigation validation set, keep extending and validating [`felixtensor/tree-sitter-tablegen`](https://github.com/felixtensor/tree-sitter-tablegen) against four corpora and gate version bumps on a CI harness that asserts zero `ERROR` nodes:
  - **MLIR TableGen** — dialect / op / pass definitions under `mlir/include/mlir/` and `mlir/test/`
  - **LLVM TableGen** — target backends under `llvm/lib/Target/*/` (heavy use of intrinsics, patterns, register classes)
  - **Clang TableGen** — `clang/include/clang/Basic/{Attr,Diagnostic,StmtNodes,…}.td`
  - **LLDB TableGen** — command option definitions under `lldb/source/Commands/Options.td` and related

  Scope the zero-`ERROR` gate to curated, valid source corpora; deliberately invalid diagnostic tests should be tracked separately.

## Ideas (unscored)

- Code-lens-style links from a PDLL `Pattern` to the TableGen op definition it rewrites, pending a suitable Zed extension API surface. First verify what `mlir-pdll-lsp-server` already provides via go-to-definition.
- Quick-fix for "missing `include`" in TableGen (auto-insert the canonical header path), most likely as an upstream `tblgen-lsp-server` code action rather than a Zed-only feature.
- Dialect-aware highlighting inside MLIR string attributes that embed recognized DSLs. Keep this opt-in / whitelist-driven so ordinary MLIR string attributes are not over-highlighted.
- Block folding driven by tree-sitter regions. Zed currently derives folds from indentation and does not document a `folds.scm` capability for extensions; revisit if/when one is exposed.

## Out of scope (today)

- **LSP settings-change behaviour.** Zed documents initialization options as startup-time configuration that requires a language-server restart to reapply. The extension API does not expose hooks to intercept settings changes or present custom restart prompts, so finer-grained control (prompt / auto-restart / ignore) is not implementable by extensions today.

## How to propose changes

Open an issue at [felixtensor/zed-mlir-suite](https://github.com/felixtensor/zed-mlir-suite/issues) with:
- What problem you hit or what workflow you want,
- Any pointers to upstream LLVM docs or related issues.
