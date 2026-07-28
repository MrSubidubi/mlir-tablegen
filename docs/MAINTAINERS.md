# Maintainer Guide

This guide covers issue triage, ownership boundaries, labels, and
cross-repository parser work. Contributor-facing development and validation
guidance lives in [CONTRIBUTING.md](../CONTRIBUTING.md).

## Triage workflow

1. Check that the report contains a useful reproducer, affected language, Zed
   version, MLIR Suite commit, and relevant local or SSH environment details.
   Ask only for information needed by the affected path.
2. Confirm the request-kind label applied by the Issue Form and add the relevant
   `MLIR`, `TableGen`, or `PDLL` language labels.
3. Use the symptom table below to identify the first ownership boundary to
   inspect.
4. Reproduce the problem at the narrowest practical layer: pinned parser, suite
   query or configuration, Zed, or LLVM language server.
5. Route the issue using the evidence-based rules below.

Do not require reporters to identify the responsible layer. Keep uncertain
reports in this repository until the boundary is understood.

## Ownership and routing

MLIR Suite owns the language resources under [`languages/`](../languages),
including its PDLL/TableGen C++ injection queries, and the extension integration
under [`src/`](../src). [`extension.toml`](../extension.toml) pins the Tree-sitter
parsers. Zed owns the extension host, LSP client, themes, bundled C++ language,
and the C++-to-MLIR raw-string injection entry point. LLVM owns the three
language servers, their advertised capabilities, and their protocol responses.

Use observable behavior to choose the first check:

| Symptom | Check first | Route if confirmed |
|---|---|---|
| File has the wrong language mode | Language configuration and host context | Fix in the suite, or report a Zed detection issue |
| Valid source produces `ERROR` or `MISSING` | Syntax tree under the exact pinned parser | Create or link a parser issue |
| Syntax tree is correct but highlighting is wrong | Query captures, precedence, and a stock theme | Fix suite queries, or report a Zed rendering issue |
| Standalone or injected content is missing | Host tree, injection range, and injected grammar | Fix suite queries, or report a host-grammar or Zed issue |
| Language server does not start | Binary resolution, settings, arguments, and remote paths | Fix suite discovery or configuration |
| Diagnostics, hover, or navigation are wrong | The same LLVM server outside Zed | Report an LLVM issue if the behavior reproduces |
| Completion text is styled incorrectly | Server label, kind, detail, and suite styling rules | Fix suite styling or identify an LLVM payload issue |

### Keep the issue in MLIR Suite

Keep the issue here when MLIR Suite owns the fix or still needs to consume,
work around, or verify an upstream result. This includes language resources,
language-server launch and configuration, completion-label styling,
documentation, tests, CI, packaging, and final parser integration.

### Route to Zed or LLVM

Use [zed-industries/zed](https://github.com/zed-industries/zed/issues) for a
general editor, extension-host, LSP-client, bundled C++, or theme-rendering
problem confirmed not to be specific to MLIR Suite.

Use [llvm/llvm-project](https://github.com/llvm/llvm-project/issues) when the same
server behavior reproduces outside Zed with the relevant LLVM language-server
binary.

When no suite work remains, record the evidence and upstream link in the local
issue, then close it as not planned. Otherwise keep it open as the integration
tracker. Parser-specific routing is described below.

## Labels

| Category | Labels | Application |
|---|---|---|
| Request kind | `Bug`, `Enhancement`, `Question` | Applied by the Issue Form |
| Language | `MLIR`, `TableGen`, `PDLL` | Add at least one when applicable |
| Area | `Documentation` | Use for documentation corrections or improvements |
| Integration path | `LSP` | Use for language-server setup or behavior; this is not a root-cause label |
| Root cause | `Tree-sitter` | Use with a language label when parser evidence exists |
| Workflow | `Duplicate`, `Invalid`, `Good first issue`, `Help wanted`, `Wontfix` | Apply during triage and closure |

Close `Duplicate` issues as duplicates and `Invalid` or `Wontfix` issues as not
planned. `Good first issue` and `Help wanted` describe open work.

Update labels when later evidence changes the ownership decision. Applying
`Tree-sitter` does not require a parser issue to exist, but it should reflect a
confirmed or well-supported parser diagnosis rather than a general syntax
symptom.

## Parser issues

The language label determines the parser repository:

| Language label | Parser repository |
|---|---|
| `MLIR` | [`felixtensor/tree-sitter-mlir`](https://github.com/felixtensor/tree-sitter-mlir) |
| `TableGen` | [`felixtensor/tree-sitter-tablegen`](https://github.com/felixtensor/tree-sitter-tablegen) |
| `PDLL` | [`felixtensor/tree-sitter-pdll`](https://github.com/felixtensor/tree-sitter-pdll) |

### When to escalate

Parser evidence includes:

- valid source producing `ERROR` or `MISSING` under the exact pinned grammar;
- the same tree problem reproducing outside Zed;
- a required node or field being absent, malformed, or attached to the wrong
  construct.

A color mismatch, query-capture mistake, theme difference, or incorrect LLVM
server response is not by itself evidence of a parser defect.

Before escalating:

1. Reduce the report to valid source that still reproduces the tree problem.
2. Capture the current tree and expected node or field structure.
3. Record the language-detection, query, and theme checks that excluded a
   suite-side presentation problem.
4. Describe the effect on MLIR Suite or another parser consumer.

### Issue relationships

The MLIR Suite issue remains the user-facing parent and final integration
tracker.

- Add a newly created, report-specific parser task as a cross-repository child
  of the suite issue.
- Link an existing or general parser issue from the suite tracker instead of
  re-parenting it; a backlink from the parser issue is optional.
- Never make the suite report a child of the parser issue.
- Use a separate child or link for each affected parser.

Do not require mirrored descriptions, labels, milestones, or workflow states in
the parser repository.

### Creating a parser issue

Create a focused parser child with GitHub CLI v2.94.0 or later, selecting the
repository from the language table above:

```bash
gh issue create \
  --repo felixtensor/tree-sitter-mlir \
  --parent https://github.com/felixtensor/zed-mlir-suite/issues/<number> \
  --title "<parser-level problem>" \
  --body-file <parser-issue.md>
```

A minimal parser issue body can use this structure:

```md
## Minimal source

## Current syntax tree

## Expected syntax tree

## Consumer impact

Observed in felixtensor/zed-mlir-suite#<number>.
```

Keep the parser issue focused on the source, tree structure, expected parser
behavior, and consumer impact.

### Integrating and closing

When a parser fix becomes available, update the relevant grammar pin in
[`extension.toml`](../extension.toml) together with any required suite-side
query, configuration, fixture, or test changes. Follow the
[parser, highlighting, and query validation checklist](../CONTRIBUTING.md#parser-highlighting-and-query-changes)
in `CONTRIBUTING.md` rather than duplicating it here.

Before closing the suite issue:

1. Confirm the parser commit is public and fetchable by Zed.
2. Record the relevant node or field changes and upstream reference.
3. Confirm the suite change consumes the new pin and includes every required
   query, configuration, fixture, or test update.
4. Confirm the contribution checklist was completed against the exact pin.
5. Verify the original suite reproducer in Zed, including relevant standalone
   and injected-language contexts.

Closing a parser child does not complete the suite parent. Close the parent only
after MLIR Suite consumes the parser commit and verifies the original behavior.
Do not update unrelated grammar pins or commit local grammar checkouts,
generated grammar Wasm files, or `extension.wasm` as part of the integration.
