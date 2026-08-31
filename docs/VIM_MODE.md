# Vim Mode

Zed's Vim mode provides function, class, and comment text objects, and the
motions built on them. This extension supplies the captures behind them for
MLIR, TableGen, and PDLL — no language server is involved, since they come from
the Tree-sitter grammars. Coverage follows each grammar, so not every language
has every object.

## Mapping

| Language | Function (`af` / `if`, `]m`) | Class (`ac` / `ic`) |
| --- | --- | --- |
| MLIR | `func.func`, `llvm.func` | `module`, `builtin.module` |
| PDLL | top-level `Pattern`, `Constraint`, `Rewrite` | — |
| TableGen | named `def`, `defm` | `class`, `multiclass` |

`gc` takes the surrounding run of comments in all three languages.

The mapping follows each grammar's node shapes rather than applying one rule
everywhere:

- **MLIR** gives the function object to `func.func` and `llvm.func`, and the
  class object to modules. Other regions and blocks are nested operation bodies
  rather than file sections, so treating them as classes would make `ac` select
  an arbitrary nesting level.
- **PDLL** gives the function object to its three top-level declarations, and
  has no class-like construct. Inline `Constraint` and `Rewrite` helpers are
  skipped in favor of the declaration enclosing them, because the grammar
  aliases the inline forms to the same node types as the top-level ones.
- **TableGen** keeps `class` / `multiclass` and named `def` / `defm` on
  different nesting levels inside a multiclass. Anonymous records are skipped in
  favor of the declaration enclosing them.

## Motion Depth in MLIR

Zed bounds how deep the motions search, but not the text objects. In MLIR that
bound falls inside an explicit `module { ... }`, so `]]` stops on the module and
`]m` has nothing to visit, while `af` and `ac` keep working at any depth.

This affects only the motions. Selecting a function or class with a text object
behaves the same regardless of nesting.
