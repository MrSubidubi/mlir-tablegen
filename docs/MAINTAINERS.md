# Maintainer Guide

This guide covers issue triage, ownership boundaries, labels, and
cross-repository parser work. Contributor-facing development and validation
guidance lives in [CONTRIBUTING.md](../CONTRIBUTING.md).

## Quick triage

1. Check that the report contains a useful reproducer, affected language, Zed
   version, MLIR Suite commit, and relevant local or SSH environment details.
   Ask for only the information needed by the affected path.
2. Confirm the request-kind label applied by the Issue Form and add the relevant
   `MLIR`, `TableGen`, or `PDLL` language labels.
3. Inspect the boundary in this order:
   - language detection and host-language context;
   - concrete syntax tree;
   - query captures and stock-theme rendering;
   - extension-side language-server discovery and configuration;
   - LLVM language-server behavior outside Zed.
4. Choose one outcome:
   - keep the issue in MLIR Suite;
   - link or redirect to Zed or LLVM;
   - create or link a parser issue.
5. Keep the suite issue open while MLIR Suite still needs to consume, work
   around, or verify an upstream change.

## Ownership and common symptoms

MLIR Suite integrates several layers:

- Pinned Tree-sitter parsers in [`extension.toml`](../extension.toml) define the
  concrete syntax trees for MLIR, TableGen, and PDLL.
- Files under [`languages/`](../languages) define language detection, editing
  behavior, highlighting, indentation, bracket matching, outlines, and
  injections over those trees.
- Rust code under [`src/`](../src) discovers and launches LLVM language servers,
  translates settings into command-line arguments, and styles selected
  completion labels.
- Zed supplies the extension host, LSP client, themes, bundled C++ language, and
  the C++-to-MLIR raw-string injection entry point.
- LLVM supplies `mlir-lsp-server`, `mlir-pdll-lsp-server`, and
  `tblgen-lsp-server` and owns their protocol responses and advertised
  capabilities.

Use observable behavior to select the first check:

| Symptom | Check first | Likely action |
|---|---|---|
| File has the wrong language mode | Language configuration and host context | Fix in the suite, or report a confirmed Zed detection issue |
| Valid source produces `ERROR` or `MISSING` | Syntax tree under the pinned parser | Investigate a parser issue |
| Syntax tree is correct but highlighting is wrong | Query captures, precedence, and a stock theme | Fix suite queries, or report a confirmed Zed rendering issue |
| Standalone or injected content is missing | Host tree, injection range, and injected grammar | Fix suite queries or report a confirmed host-grammar issue |
| Language server does not start | Binary resolution, settings, arguments, and remote paths | Fix suite discovery or configuration |
| Diagnostics, hover, or navigation are wrong | Reproduce with the LLVM server outside Zed | Link or report an LLVM issue when confirmed |
| Completion text is styled incorrectly | Server label, kind, detail, and suite styling rules | Fix suite styling or identify an upstream payload issue |

Keep uncertain reports in this repository until the responsible layer is
understood.

## Triage outcomes

### Keep in MLIR Suite

Keep the issue in this repository when it concerns:

- parsing, highlighting, editing, outlines, or injections observed through MLIR
  Suite;
- language-server discovery, startup, settings, compilation-database detection,
  include paths, or SSH remote behavior;
- MLIR or PDLL completion-label styling implemented by the extension;
- dev-extension installation, documentation, tests, CI, or packaging;
- upstream work that MLIR Suite still needs to consume or verify.

A suspected dependency problem remains here until the boundary has enough
evidence to choose another outcome.

### Link or redirect to Zed or LLVM

Use the Zed tracker for a general editor, extension-host, LSP-client, bundled C++,
or theme-rendering problem confirmed not to be specific to MLIR Suite:

- [zed-industries/zed](https://github.com/zed-industries/zed/issues)

Use the LLVM tracker when the same server behavior reproduces outside Zed with
the relevant LLVM language-server binary:

- [llvm/llvm-project](https://github.com/llvm/llvm-project/issues)

When no suite work remains, record the evidence and upstream link in the local
issue, then close it as not planned. When MLIR Suite must consume, work around,
or verify the upstream result, keep the local issue open as the integration
tracker. Do not require the reporter to determine this boundary before filing.

### Create or link a parser issue

Evidence for a parser root cause includes:

- valid source producing `ERROR` or `MISSING` under the pinned grammar;
- the same tree problem reproducing outside Zed;
- a required node or field being absent, malformed, or attached to the wrong
  construct.

A color mismatch, query-capture mistake, theme difference, or incorrect LLVM
server result is not by itself evidence of a parser defect.

Create a dedicated parser child when the work is specific to the suite report.
Cross-link an existing parser issue when it already tracks a general problem for
multiple consumers. Use a separate parser issue for each affected parser.

## Labels

| Category | Labels | Application |
|---|---|---|
| Request kind | `Bug`, `Enhancement`, `Question` | Applied by the Issue Form |
| Language | `MLIR`, `TableGen`, `PDLL` | Add at least one when applicable |
| Area | `Documentation` | Use for documentation corrections or improvements |
| Integration path | `LSP` | Use for language-server setup or behavior; marks the affected path, not the root cause |
| Root cause | `Tree-sitter` | Use with a language label when parser evidence exists |
| Workflow | `Duplicate`, `Invalid`, `Good first issue`, `Help wanted`, `Wontfix` | Apply during triage |

Update labels when later evidence changes the ownership decision. Applying
`Tree-sitter` does not require a parser issue to exist, but it should reflect a
confirmed or well-supported parser diagnosis rather than a general syntax
symptom.

## Parser escalation

The parser target follows the language label:

| Language label | Parser repository |
|---|---|
| `MLIR` | [`felixtensor/tree-sitter-mlir`](https://github.com/felixtensor/tree-sitter-mlir) |
| `TableGen` | [`felixtensor/tree-sitter-tablegen`](https://github.com/felixtensor/tree-sitter-tablegen) |
| `PDLL` | [`felixtensor/tree-sitter-pdll`](https://github.com/felixtensor/tree-sitter-pdll) |

Before creating a parser issue:

1. Reduce the report to valid source that still reproduces the tree problem.
2. Capture the current tree and the expected node or field structure.
3. Record the language-detection, query, and theme checks that excluded a
   suite-side presentation problem.
4. Describe the effect on MLIR Suite or another parser consumer.

The MLIR Suite issue remains the user-facing parent and final integration
tracker.

- A newly created parser task dedicated to the report may be added as a
  cross-repository child of the suite issue.
- Do not make the suite report a child of the parser issue.
- Cross-link an existing or general parser issue instead of re-parenting it.
- If more than one parser is involved, use a separate child or link for each.

Create a focused parser child with GitHub CLI v2.94.0 or later:

```sh
gh issue create \
  --repo felixtensor/tree-sitter-mlir \
  --parent https://github.com/felixtensor/zed-mlir-suite/issues/<number> \
  --title "<parser-level problem>" \
  --body-file <parser-issue.md>
```

Select the repository according to the language label. A minimal parser issue
body can use this structure:

```md
## Minimal source

## Current syntax tree

## Expected syntax tree

## Consumer impact

Observed in felixtensor/zed-mlir-suite#<number>.
```

Do not require mirrored descriptions, labels, milestones, workflow states, or
reciprocal links in the parser repository. Keep the parser issue focused on the
source, tree structure, expected parser behavior, and consumer impact.

## Integrating parser fixes

When a parser fix becomes available, update the relevant grammar pin in
[`extension.toml`](../extension.toml) together with any required suite-side
query, configuration, fixture, or test changes. Follow the grammar-pin validation
checklist in
[CONTRIBUTING.md](../CONTRIBUTING.md#parser-highlighting-and-query-changes).

Before closing the suite issue:

1. Confirm the parser commit is public and fetchable by Zed.
2. Describe relevant node or field changes.
3. Review every affected query consumer: `highlights.scm`, `injections.scm`,
   `outline.scm`, `indents.scm`, and `brackets.scm` where present.
4. Load or compile affected queries against the exact new pin.
5. Test the original suite-level reproducer as a dev extension.
6. Verify relevant standalone and injected-language contexts.

Closing a parser child does not complete the suite parent. Close the parent only
after MLIR Suite consumes the parser commit and verifies the original Zed
behavior. Do not update unrelated grammar pins or commit local grammar checkouts,
generated grammar Wasm files, or `extension.wasm` as part of the integration.
