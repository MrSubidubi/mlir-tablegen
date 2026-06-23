# Changelog

This document records shipped project history reconstructed from the local Git
tags. Dates are tag or commit dates. The v0.1.0 tag was restored
retroactively on the last commit that still declared version 0.1.0.

For future plans, see [docs/ROADMAP.md](docs/ROADMAP.md).

## [v0.6.1] - 2026-06-23

### Added

- Added rich completion labels for the MLIR and PDLL language servers,
  classifying values, blocks, dialects, aliases, operations, builtin types,
  attributes, constraints, and include paths by their LSP kind and detail.
- Added unit tests for the completion-label classification helpers.
- Documented MLIR syntax highlighting inside C++ raw string literals.

### Changed

- Migrated `extension.toml` language-server metadata to the `languages = [...]`
  field; the deprecated singular `language` field is no longer used.
- Bumped the MLIR grammar revision to align with tree-sitter-mlir v0.1.3 and
  v0.1.6.
- Added `publish = false` and the Apache-2.0 WITH LLVM-exception license to
  `Cargo.toml` to match the repository `LICENSE` file.
- Added `cargo test --lib` to the CI workflow.

### Improved

- Improved MLIR highlighting coverage for new parser nodes and anonymous tokens
  introduced by the grammar updates.
- Improved MLIR and TableGen highlight capture names to match Zed's documented
  highlight set.
- Improved PDLL constraint classification by base name instead of a hardcoded
  prefix list.

### Fixed

- Fixed language-server binary resolution to probe `PATH` per worktree instead
  of reusing a shared cache across worktrees.
- Fixed coloring for prefix-stripped MLIR attribute and type aliases in
  completions.

## [v0.6.0] - 2026-06-02

### Added

- Added TableGen symbol outline navigation for stable named records and useful
  top-level declarations.
- Added SSH remote development documentation.

### Changed

- Updated the TableGen grammar revision for merged object-name support.
- Updated the PDLL grammar revision to the parser-aligned tree-sitter-pdll
  commit.
- Tightened PDLL word-character handling to match the updated lexer while
  keeping dotted operation names selectable.

### Improved

- Improved MLIR highlighting for source locations.
- Improved TableGen outline labels for computed object names.
- Improved PDLL highlighting for the new `op_name` and `negated_call_expr`
  parser nodes.

### Fixed

- Fixed TableGen outline truncation for paste, bang-operator, suffix, and
  code-fragment object-name expressions.
- Fixed PDLL outline captures to include only named top-level declarations.
- Removed stale PDLL inline type-constraint highlight assumptions after the
  parser update.

## [v0.5.3] - 2026-05-20

### Added

- Added bilingual README documentation and refreshed developer-install notes.
- Added the project changelog and refreshed the v0.6 roadmap.

### Changed

- Updated the MLIR and TableGen grammar revisions used by the extension.
- Simplified language-server settings parsing.

### Improved

- Improved MLIR highlighting for string escapes and dimension separators.
- Improved TableGen highlighting for ODS fields, definition names, and LHS-only
  declaration / binding captures.
- Restricted TableGen `<>` indentation to constructs that commonly span
  multiple lines.

### Fixed

- Fixed CI to build the published WebAssembly target.
- Fixed TableGen C++ injection coverage for renamed and shared declaration
  fields.
- Fixed language-server startup behavior by inheriting the user's shell
  environment and logging invalid settings.

## [v0.5.2] - 2026-05-07

### Added

- Switched TableGen support to the maintained
  [`felixtensor/tree-sitter-tablegen`](https://github.com/felixtensor/tree-sitter-tablegen)
  grammar.
- Added TableGen C++ injection for ODS code-carrying fields, with injection
  restricted to known code fields instead of arbitrary descriptions or strings.
- Added TableGen string escape highlighting.

### Changed

- Render TableGen `[{ ... }]` code literals as string-like source regions
  instead of generic embedded content.
- Highlight TableGen `$name` uniformly as `@variable.special`.

### Fixed

- Removed an invalid anonymous-node match from the TableGen queries.

## [v0.5.1] - 2026-04-24

### Added

- Added structured LSP settings and extra include directory support for
  TableGen and PDLL language servers.
- Added auto-detection for `tablegen_compile_commands.yml` and
  `pdll_compile_commands.yml` in common build directories.
- Added GitHub Actions CI for build verification.
- Added C++ injection inside PDLL native `[{ ... }]` code blocks.

### Changed

- Refactored language-server integration into dedicated server modules.
- Unified server configuration handling across MLIR, PDLL, and TableGen.
- Renamed the repository to `zed-mlir-suite` and updated extension metadata.
- Reorganized README onboarding and configuration documentation.
- Replaced manually played README videos with optimized auto-playing GIFs.

### Improved

- Refined MLIR highlighting for dictionary attribute keys, composite builtin
  type nodes, affine keywords/operators, and indentation behavior.
- Added comments documenting Zed's last-match-wins query behavior where it
  affects MLIR highlighting rules.

## [v0.5.0] - 2026-04-21

### Added

- Wired up all three upstream LLVM language servers:
  `mlir-lsp-server`, `mlir-pdll-lsp-server`, and `tblgen-lsp-server`.
- Added per-server binary path resolution and argument passthrough through
  Zed LSP settings.
- Added LSP setup documentation, screenshots, and the first roadmap document.

### Changed

- Rebranded the extension from `MLIR` to `MLIR Suite`.
- Changed the extension id to `mlir-suite`.
- Cleaned up `extension.toml` grammar and language-server metadata.
- Hosted demo media through GitHub user attachments.

### Fixed

- Rewrote TableGen indentation using generic bracket-pair matching.

## [v0.4.0] - 2026-04-20

### Added

- Added initial PDLL language support, including grammar registration,
  highlights, indentation, bracket matching, and symbol outline.
- Added README feedback and contribution guidance.

### Improved

- Ordered PDLL highlights for Zed's last-match-wins query semantics.
- Improved PDLL builtin type constraint highlighting.
- Improved TableGen highlighting for member access and `let` item fields.

### Fixed

- Reordered MLIR `dense_resource` bare id highlighting to preserve the intended
  fallback behavior.

## [v0.3.1] - 2026-04-16

### Added

- Added MLIR highlighting support for `public` visibility.
- Added MLIR module `attributes` highlighting.

### Changed

- Updated README content to reflect current features and dev-install workflow.
- Aligned MLIR module highlighting with the latest grammar/query behavior.

## [v0.3.0] - 2026-04-13

### Added

- Added TableGen (`.td`) language support.
- Added TableGen grammar registration, language configuration, highlighting,
  indentation, and bracket matching.

### Fixed

- Fixed the TableGen `block_comment` configuration.

## [v0.2.1] - 2026-04-02

### Added

- Added MLIR indentation support.
- Added MLIR symbol outline support.

### Improved

- Expanded and refined MLIR syntax highlighting to better match the intended
  TextMate-style scopes.
- Improved `dense_resource` highlighting and bumped the MLIR grammar revision.

### Fixed

- Fixed an issue that prevented the MLIR language from loading.

## [v0.2.0] - 2026-03-31

### Changed

- Bumped the extension and crate version to 0.2.0.
- Synced the MLIR grammar revision with the latest `tree-sitter-mlir` changes
  available at the time.
- Updated MLIR language configuration for Zed compatibility, including
  `line_comments`, word characters, autoclose behavior, and quote exclusions in
  comments / strings.
- Simplified MLIR syntax highlighting after the grammar update.

### Fixed

- Fixed installation failures and version metadata issues.

## [v0.1.0] - 2026-03-12

### Added

- Added the initial Rust-based Zed extension structure.
- Added MLIR language registration, grammar metadata, syntax highlighting, and
  bracket matching.
- Added the initial README, license, Cargo manifest, and extension metadata.

### Changed

- Switched the MLIR grammar source to
  [`felixtensor/tree-sitter-mlir`](https://github.com/felixtensor/tree-sitter-mlir)
  before the first tagged baseline.

[v0.6.1]: https://github.com/felixtensor/zed-mlir-suite/compare/v0.6.0...v0.6.1
[v0.6.0]: https://github.com/felixtensor/zed-mlir-suite/compare/v0.5.3...v0.6.0
[v0.5.3]: https://github.com/felixtensor/zed-mlir-suite/compare/v0.5.2...v0.5.3
[v0.5.2]: https://github.com/felixtensor/zed-mlir-suite/compare/v0.5.1...v0.5.2
[v0.5.1]: https://github.com/felixtensor/zed-mlir-suite/compare/v0.5.0...v0.5.1
[v0.5.0]: https://github.com/felixtensor/zed-mlir-suite/compare/v0.4.0...v0.5.0
[v0.4.0]: https://github.com/felixtensor/zed-mlir-suite/compare/v0.3.1...v0.4.0
[v0.3.1]: https://github.com/felixtensor/zed-mlir-suite/compare/v0.3.0...v0.3.1
[v0.3.0]: https://github.com/felixtensor/zed-mlir-suite/compare/v0.2.1...v0.3.0
[v0.2.1]: https://github.com/felixtensor/zed-mlir-suite/compare/v0.2.0...v0.2.1
[v0.2.0]: https://github.com/felixtensor/zed-mlir-suite/compare/v0.1.0...v0.2.0
[v0.1.0]: https://github.com/felixtensor/zed-mlir-suite/releases/tag/v0.1.0
