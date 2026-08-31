# Contributing to Zed MLIR

Thank you for helping improve MLIR, TableGen, and PDLL support in Zed.
Contributions may include Rust extension code, Tree-sitter queries, language
configuration, tests, documentation, and updates to pinned parser revisions.

## Before you start

Use this repository as the first stop for bugs, feature requests, and setup
questions. You do not need to determine whether a highlighting or editing
problem comes from a parser, a query, Zed, or another dependency before
reporting it. Provide the smallest useful reproducer and the maintainer will
triage the responsible layer.

Search the [existing issues](https://github.com/felixtensor/mlir-tablegen/issues)
and the [roadmap](docs/ROADMAP.md) before opening a report. Use the Bug, Feature,
or Question form so the report contains the information needed for triage.

This repository is the right place to report problems with:

- MLIR, TableGen, or PDLL parsing, highlighting, and editing behavior;
- standalone and injected-language support;
- symbol outlines and completion-label styling;
- LLVM language-server discovery, startup, settings, and SSH remote behavior;
- installing it from the extension gallery or as a Zed dev extension;
- project documentation, tests, CI, and packaging.

A confirmed general Zed problem may be redirected to
[zed-industries/zed](https://github.com/zed-industries/zed/issues). LLVM server
behavior reproducible outside Zed may be redirected to
[llvm/llvm-project](https://github.com/llvm/llvm-project/issues). Uncertain cases
should remain here until the responsible layer is understood.

For substantial new capabilities, especially work that depends on a new Zed
extension API or LLVM language-server behavior, open a feature request before
investing in an implementation. Focused bug fixes and documentation corrections
may go directly to a pull request.

## Development setup

Install Rust through [rustup](https://rustup.rs) and add the WebAssembly target
used by Zed extensions:

```bash
rustup target add wasm32-wasip2
```

Run the same Rust checks as CI for source, query, configuration, dependency, and
release changes:

```bash
cargo fmt -- --check
cargo test --lib
cargo clippy --target wasm32-wasip2 -- -D warnings
cargo build --target wasm32-wasip2
```

Documentation-only changes may report these checks as not applicable.

Install the checkout through **zed: install dev extension** before testing
user-visible behavior. LLVM language-server binaries are only required when a
change affects LSP integration.

## Keep changes focused

- Keep each pull request to one coherent change.
- Do not combine unrelated formatting, dependency, grammar-pin, or version
  changes with a focused fix.
- Do not update unrelated grammar pins in the same pull request.
- Do not commit files ignored by `.gitignore`, including `grammars/`, `target/`,
  `.zed/`, `tmp/`, logs, generated grammar Wasm files, or `extension.wasm`.
- Update `Cargo.lock` when dependency changes are necessary.
- The maintainer updates `CHANGELOG.md` during release preparation. Do not edit a
  published changelog section unless correcting its historical record.

## Code and query standards

### Rust

- Keep code formatted with `rustfmt` and free of Clippy warnings.
- Prefer focused, deterministic helpers that can be covered by unit tests.
- Use the Zed extension API and `Worktree` facilities for platform, environment,
  path, and process behavior instead of assuming access to the host environment.
- Preserve documented settings precedence and compatibility unless a breaking
  change is intentional and documented.
- Avoid new dependencies when the extension API or standard library is
  sufficient.

### Tree-sitter queries and language configuration

- Inspect the concrete syntax tree before changing a query. Parser structure and
  visual classification are separate concerns.
- Match only nodes and fields available in the exact grammar commit pinned by
  `extension.toml`.
- Preserve the query file's precedence conventions: broad fallbacks must not
  override more specific captures.
- Use Zed's documented capture names and verify captures or ranges, not only the
  color produced by one theme.
- Keep context-specific visual heuristics in this repository's queries rather
  than adding parser nodes solely to encode a preferred color.
- Treat auto-closing configuration and `brackets.scm` matching as separate
  behaviors and test them separately.
- Add positive and negative examples for injections so ordinary strings or code
  fields are not over-injected.

### Documentation

- Update the English documentation and identify corresponding Chinese sections
  that need synchronization. Update both when you can validate the translation;
  otherwise the maintainer will coordinate the Chinese update.
- Summarize user-visible impact in the pull request rather than editing the
  changelog directly.

## Validation by change type

Select the checks that apply and describe any important environment that was
unavailable. The pull request template intentionally asks for only the checks
actually performed; use the guidance below for affected areas.

### Parser, highlighting, and query changes

- Include a minimal source example and, when available, the relevant syntax-tree
  fragment or `ERROR` / `MISSING` node.
- Test a realistic LLVM/MLIR source in addition to the minimal example.
- Check every affected query against the exact pinned grammar revision.
- For MLIR, test standalone `.mlir` and C++ `R"mlir(...)mlir"` when the change
  can affect injected content.
- For PDLL and TableGen C++ injections, test host-language captures, delimiters,
  injected C++ content, and a negative example that must not be injected.
- Test with a stock Zed theme when the report might be theme-specific.

When updating a grammar pin:

1. Reference the parser issue, pull request, release, or commit when one exists.
2. Use a public commit that Zed can fetch.
3. Describe relevant node or field changes.
4. Review every query consumer for that language: `highlights.scm`,
   `injections.scm`, `outline.scm`, `indents.scm`, and `brackets.scm` where
   present.
5. Load or compile affected queries against the exact new pin.
6. Re-test the original issue reproducer.

The repository does not yet have an automated CI gate that compiles every query
against every pinned grammar. Until that roadmap item is implemented, describe
the dev-extension and representative-file checks performed for grammar pins and
query changes.

### Editing and navigation changes

- Exercise auto-close behavior by typing the opening token.
- Exercise bracket matching independently, including nested pairs.
- Check indentation on opening, nested, empty, and closing constructs.
- Check outlines for named and anonymous declarations, top-level and nested
  constructs, duplicate entries, and truncated labels.

### LSP and completion changes

- Identify the LLVM server and version or commit used for testing.
- State whether the executable came from `binary.path`, `settings.path`, or
  `$PATH`.
- Include relevant `binary.arguments`, `compilation_database`, and `extra_dirs`
  settings.
- Distinguish extension startup/configuration behavior from server responses that
  reproduce outside Zed.
- For remote changes, state whether paths, binaries, settings, and logs came from
  the local or SSH remote machine.
- For completion-label changes, include the server's label, kind, and detail and
  run the focused Rust unit tests.

## Pull requests

Keep each pull request focused on one coherent change. The description should:

1. Explain the user-visible problem or maintenance goal.
2. Describe the solution, ownership boundary, and important tradeoffs when
   relevant.
3. Link related issues with `Fixes #<number>` when applicable.
4. Reference parser, Zed, or LLVM work when it is relevant and already exists.
5. List only the validation performed and the relevant versions, grammar commits,
   platforms, and local or SSH environments.
6. Include a minimal source sample, screenshot, syntax-tree fragment, or log
   excerpt when it makes the result easier to review.

For documentation-only changes, `N/A — documentation-only` is sufficient in the
Validation section. Do not fill unrelated validation categories merely to
complete the template.

## License

By contributing, you agree that your contribution is licensed under the
[Apache License 2.0 with LLVM Exceptions](LICENSE) used by this repository.
