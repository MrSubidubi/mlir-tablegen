# Zed MLIR

[MLIR](https://mlir.llvm.org) (`.mlir`), [TableGen](https://llvm.org/docs/TableGen/)
(`.td`), and [PDLL](https://mlir.llvm.org/docs/PDLL/) (`.pdll`) support for the
[Zed](https://zed.dev) editor.

## Development

To develop this extension, see the [Developing Extensions](https://zed.dev/docs/extensions/developing-extensions) section of the Zed docs.

## Highlighting

Custom and out-of-tree dialects need no configuration: any `dialect.op` form is
recognized, so a project's own dialects behave like upstream ones. MLIR embedded
in C++ raw strings is highlighted as MLIR wherever Zed's bundled C++ grammar
injects `raw_string_content` by delimiter, as in `R"mlir(…)mlir"`.

## Language Servers

The extension integrates the three language servers from the LLVM project:
`mlir-lsp-server` for `.mlir`, `mlir-pdll-lsp-server` for `.pdll`, and
`tblgen-lsp-server` for `.td`. They are optional — Tree-sitter highlighting,
symbol outlines, bracket matching, and indentation work without them.

Completion items from the MLIR and PDLL servers are classified by their LSP kind
and detail, so values, blocks, dialects, operations, types, attributes,
constraints, and include paths are colored consistently.

### Server Capabilities

The available LSP features depend on the server and LLVM version. The following
matrix reflects
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
which is not advertised in the initialization capability object. MLIR document
symbols require the client to advertise hierarchical document symbols.

### Disable the Servers

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

Add this to your user `settings.json` or a project's `.zed/settings.json`. See
Zed's documentation for
[enabling or disabling language servers](https://zed.dev/docs/configuring-languages#enabling-or-disabling-language-servers).

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
executable is resolved in this order: `binary.path`, then `settings.path`, then
the worktree's `$PATH`.

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

When a project is opened over SSH, the source code, language servers, tasks, and
terminals run on the remote server; the local machine only runs the Zed UI. See
Zed's [remote development documentation](https://zed.dev/docs/remote-development#zed-settings)
for the full model.

Zed keeps the settings scopes separate, and editing one does not update another.
`zed: open settings file` edits the local UI machine, `zed: open server settings`
edits the remote server, and `zed: open project settings file` (or
`.zed/settings.json`) applies to everyone opening that project. Configure the
language-server binaries in the **remote server settings**, not the local
settings file: if Zed runs on Windows but the project is opened on a Linux
server, the `path` value must be a Linux path on the remote server.

```jsonc
{
  "lsp": {
    "mlir-lsp-server": {
      "settings": {
        "path": "/home/you/llvm-project/build/bin/mlir-lsp-server"
      }
    },
    "tblgen-lsp-server": {
      "settings": {
        "path": "/home/you/llvm-project/build/bin/tblgen-lsp-server"
      }
    },
    "mlir-pdll-lsp-server": {
      "settings": {
        "path": "/home/you/llvm-project/build/bin/mlir-pdll-lsp-server"
      }
    }
  }
}
```

The same rule applies to the other path-like settings: `compilation_database`
must point to a file visible to the machine running the language server,
`extra_dirs` must point to include directories visible to that same machine, and
the auto-detected `build/` and `out/` compilation databases are searched relative
to the worktree. Never put host-specific absolute paths — `C:\...` from a Windows
UI host, or `/Applications/...` and Homebrew paths from a macOS UI host — into a
project `.zed/settings.json` used by a Linux SSH workspace; the remote server
cannot execute them.
