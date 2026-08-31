# SSH Remote Development

This extension follows Zed's remote development model: when a project is opened
over SSH, the source code, language servers, tasks, and terminals run on the
remote server. The local machine only runs the Zed UI.

See Zed's official remote development documentation:
<https://zed.dev/docs/remote-development#zed-settings>

## Which Settings File To Edit

Zed has separate settings scopes in remote workspaces:

| Scope | How to open it | Use it for |
|---|---|---|
| Local user settings | `zed: open settings file` | Local UI preferences, local workspaces, SSH connection definitions |
| Remote server settings | `zed: open server settings` | Remote language-server paths, remote environment-specific settings |
| Project settings | `zed: open project settings file` or `.zed/settings.json` in the project | Settings that are valid for everyone opening that project |

Local user settings and remote server settings are intentionally separate.
Editing one does not update the other.

For SSH remote development, configure this extension's language-server binaries
in the remote server settings, not the local settings file. For example, if Zed
is running on Windows but the project is opened on a Linux server, the `path`
value must be a Linux path on the remote server.

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

`zed: open settings file` still opens the local machine's settings. That is
expected: those settings are for local Zed. They are not the right place to put
Linux-only language-server paths for a remote SSH workspace.

## How the Extension Resolves Paths

The extension asks Zed for settings and environment information for the current
worktree. The server command is resolved in this order:

1. `lsp.<server-id>.binary.path`
2. `lsp.<server-id>.settings.path`
3. The worktree's `PATH`

In a local workspace, those values are resolved against the local machine. In an
SSH remote workspace, they are resolved against the remote server.

The same rule applies to other path-like settings:

- `compilation_database` must point to a file visible to the machine running the
  language server.
- `extra_dirs` must point to include directories visible to that same machine.
- Auto-detected `build/tablegen_compile_commands.yml`,
  `out/tablegen_compile_commands.yml`, `build/pdll_compile_commands.yml`, and
  `out/pdll_compile_commands.yml` are searched relative to the worktree.

## Typical Remote Layout

Local UI host + Linux SSH server is the standard setup. The split is:

- Local user settings (`zed: open settings file`): local UI preferences, SSH
  connection definitions, and host-specific LSP paths if you also open local
  workspaces on this machine.
- Remote server settings (`zed: open server settings`): Linux paths to
  `mlir-lsp-server`, `tblgen-lsp-server`, and `mlir-pdll-lsp-server` on the
  remote server.
- Project settings (`zed: open project settings file`): settings valid for
  every developer opening the project on the remote server, such as shared
  relative build layouts or project-specific LSP options.

Never put host-specific absolute paths into project `.zed/settings.json` for a
Linux SSH workspace — the remote server cannot execute them. Examples of paths
that do not belong there: `C:\...` on a Windows UI host; `/Applications/...`,
Homebrew, or other macOS-specific paths on a macOS UI host.

After changing any LSP setting, run `zed: restart language server`.
