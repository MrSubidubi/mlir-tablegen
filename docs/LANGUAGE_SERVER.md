# Language Server Setup

This extension integrates the three language servers provided by LLVM:

| Language | Server ID |
|---|---|
| MLIR | `mlir-lsp-server` |
| PDLL | `mlir-pdll-lsp-server` |
| TableGen | `tblgen-lsp-server` |

The available LSP features depend on the server and LLVM version. The servers
are optional; Tree-sitter highlighting, symbol outlines, bracket matching,
indentation, and [Vim text objects](VIM_MODE.md) work without them.

## Server Capabilities

The following matrix reflects
[`llvm/llvm-project@06bf4bfff830`](https://github.com/llvm/llvm-project/commit/06bf4bfff830).
This extension does not pin or bundle LLVM, so capabilities may differ in other
LLVM builds.

| LSP capability | MLIR | PDLL | TableGen |
|---|:---:|:---:|:---:|
| Completion | ✅ | ✅ | ➖ |
| Diagnostics | ✅ | ✅ | ✅ |
| Signature help | ➖ | ✅ | ➖ |
| Definition | ✅ | ✅ | ✅ |
| References | ✅ | ✅ | ✅ |
| Document links | ➖ | ✅ | ✅ |
| Hover | ✅ | ✅ | ✅ |
| Document symbols | ⚠️ | ✅ | ➖ |
| Inlay hints | ➖ | ✅ | ➖ |
| Code actions | ✅ | ➖ | ➖ |
| Semantic tokens | ➖ | ➖ | ➖ |
| Formatting | ➖ | ➖ | ➖ |

*Key: ✅ Supported · ⚠️ Conditional · ➖ Not supported.*

Diagnostics use the standard `textDocument/publishDiagnostics` notification,
which is not advertised in the initialization capability object.
MLIR document symbols require the client to advertise hierarchical document
symbols.

Completion items from the MLIR and PDLL servers are classified by their LSP kind
and detail, so values, blocks, dialects, operations, types, attributes,
constraints, and include paths are colored consistently.

## Disable LSP

If the LLVM language servers are not installed, disable LSP for these languages
to prevent Zed from trying to initialize them:

```jsonc
{
  "languages": {
    "MLIR": {
      "enable_language_server": false
    },
    "PDLL": {
      "enable_language_server": false
    },
    "TableGen": {
      "enable_language_server": false
    }
  }
}
```

Add this to your user `settings.json` or a project's `.zed/settings.json`.
See Zed's official documentation for
[enabling or disabling language servers](https://zed.dev/docs/configuring-languages#enabling-or-disabling-language-servers).

## Configure LSP

### Build the Servers

The three servers live in the `llvm-project` monorepo under `mlir/tools/`.
Follow the [official MLIR Getting Started guide](https://mlir.llvm.org/getting_started/)
to build them. A typical Unix-like flow is:

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

After a successful build, the binaries are in `llvm-project/build/bin/`.
Configure each server's binary path directly in Zed. Alternatively, make that
directory available on the worktree's `$PATH`.

If `mlir` is listed in `LLVM_ENABLE_PROJECTS` and you build the default `all`
target, the three servers are produced with the rest of MLIR and do not need a
separate build command.

### Configure Zed

Configure each server under `lsp.<server-id>` in Zed's `settings.json`. The
extension resolves the executable in this order:

1. `binary.path`
2. `settings.path`
3. The worktree's `$PATH`

#### Extension Settings

| Field | Type | Applies to | Description |
|---|---|---|---|
| `path` | `string` | All | Path to the server binary |
| `compilation_database` | `string` | TableGen, PDLL | Path to the compilation-database YAML |
| `extra_dirs` | `string[]` | TableGen, PDLL | Extra include directories |
| `log` | `string` | All | Log verbosity: `"error"`, `"info"`, or `"verbose"` |
| `pretty` | `bool` | All | Pretty-print JSON output |

All fields are optional. When `compilation_database` is unset and
`binary.arguments` does not already contain the corresponding database flag,
the extension searches the worktree's `build/` and `out/` directories for the
TableGen or PDLL compilation database.

Zed's native `binary.path`, `binary.arguments`, and `binary.env` fields are also
supported. `binary.path` selects the executable, `binary.arguments` supplies
launch arguments, and `binary.env` overrides matching environment variables.

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

After changing a server's launch settings, open the command palette and run
`zed: restart language server`.

### SSH Remote Development

Language servers run on the remote server for projects opened over SSH. Set
remote binary paths with `zed: open server settings`, not
`zed: open settings file`, which edits settings on the local UI machine.

See [SSH Remote Development](REMOTE_DEVELOPMENT.md) for settings scopes, path
resolution, and a complete remote example.
