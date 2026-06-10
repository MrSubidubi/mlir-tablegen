# zed-mlir-suite

[![EN](https://img.shields.io/badge/lang-EN-lightgrey?style=flat-square)](../README.md)
[![中文](https://img.shields.io/badge/lang-中文-red?style=flat-square)](README_ZH.md)

[![CI](https://img.shields.io/github/actions/workflow/status/felixtensor/zed-mlir-suite/ci.yml?style=flat-square&logo=githubactions&logoColor=white&label=CI)](https://github.com/felixtensor/zed-mlir-suite/actions/workflows/ci.yml)
[![Version](https://img.shields.io/github/v/tag/felixtensor/zed-mlir-suite?style=flat-square&logo=github&label=version)](https://github.com/felixtensor/zed-mlir-suite/tags)
[![License](https://img.shields.io/badge/license-Apache%202.0%20with%20LLVM%20Exceptions-blue?style=flat-square&logo=apache&logoColor=white)](../LICENSE)
[![Stars](https://img.shields.io/github/stars/felixtensor/zed-mlir-suite?style=flat-square&logo=github)](https://github.com/felixtensor/zed-mlir-suite/stargazers)

为 [Zed](https://zed.dev) 编辑器提供 [MLIR](https://mlir.llvm.org)、TableGen 和 PDLL 支持。

## 功能特性

- **Tree-sitter 语法高亮** — 覆盖 MLIR（`.mlir`）、TableGen（`.td`）和 PDLL（`.pdll`），在 403 个官方 MLIR 测试文件（涵盖 11 个核心 dialect）上实现 100% 通过率，几乎所有合法的 MLIR 语法都能正确高亮。
- **C++ raw string 内嵌 MLIR 高亮** — 当 Zed 的 C++ raw-string injection 支持可用时，C++ `R"mlir(…)mlir"` 字符串中的 MLIR 会使用本扩展的 MLIR 高亮。
- **一流的自定义 dialect 支持** — 用户自定义或外部 `dialect.op` 形式均可正确识别和高亮，你的项目自有 dialect 开箱即用。
- **符号大纲** — 在大纲面板中导航 MLIR、TableGen 和 PDLL 符号。
- **集成三种上游 LLVM Language Server**：
  - `mlir-lsp-server` 用于 `.mlir`
  - `mlir-pdll-lsp-server` 用于 `.pdll`
  - `tblgen-lsp-server` 用于 `.td`
- **编辑体验优化** — 括号匹配、自动补全配对符号，以及针对每种语言调优的缩进。

## 前置条件

- [Zed](https://zed.dev) 编辑器
- （可选）LLVM Language Server 用于 LSP 功能 — 详见 [Language Server 配置](#language-server-配置)。

## 安装

本扩展以 Zed **开发扩展（dev extension）** 方式安装：克隆仓库后将目录交给 Zed，Zed 会在首次安装时将扩展编译为 WebAssembly，因此本地需要可用的 Rust 工具链。

### 1. 安装 Rust 工具链

通过 [rustup](https://rustup.rs) 安装 Rust（stable）。macOS / Linux：

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Windows 下从 [rustup.rs](https://rustup.rs) 下载并运行 `rustup-init.exe`。

随后添加 Zed 扩展所使用的 WebAssembly 目标：

```bash
rustup target add wasm32-wasip2
```

确保在启动 Zed 的同一个 shell 中工具链可用（Zed 会继承其 `PATH`）：

```bash
cargo --version
rustup target list --installed
```

已安装目标列表中需包含 `wasm32-wasip2`。

### 2. 克隆仓库

```bash
git clone https://github.com/felixtensor/zed-mlir-suite.git
```

### 3. 作为开发扩展安装

在 Zed 中打开命令面板（macOS 按 `Cmd+Shift+P`，Linux/Windows 按 `Ctrl+Shift+P`），执行 **`zed: install dev extension`** —— 或打开 **Extensions**（`Cmd+Shift+X` / `Ctrl+Shift+X`）并点击 **Install Dev Extension**，然后选择克隆的目录。

Zed 会在安装时构建扩展；首次构建需要拉取依赖，可能耗时一两分钟。若构建因目标缺失而失败，请重新执行 `rustup target add wasm32-wasip2` 并重新安装。

## Language Server 配置

### 编译服务器

三个 Language Server 位于 `llvm-project` 单体仓库的 `mlir/tools/` 目录下。按照 [MLIR 官方入门指南](https://mlir.llvm.org/getting_started/) 编译即可，典型的 Unix 编译流程如下：

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

编译成功后，二进制文件位于 `llvm-project/build/bin/`。将该目录加入 `$PATH`，或通过 `settings.json` 为每个服务器指定绝对路径（见下文）。

> 提示：如果在 `LLVM_ENABLE_PROJECTS` 中列出了 `mlir` 并编译默认的 `all` 目标，三个 LSP 服务器会随 MLIR 一起生成，无需单独构建。

### 配置

每个服务器在 Zed 的 `settings.json` 中通过 `lsp.<server-id>.settings` 进行配置。扩展会通过 `settings.path` 查找服务器二进制文件，若未设置则回退到 `$PATH`。

| 语言 | Server ID |
|---|---|
| MLIR | `mlir-lsp-server` |
| PDLL | `mlir-pdll-lsp-server` |
| TableGen | `tblgen-lsp-server` |

#### 可用配置项

| 字段 | 类型 | 适用 | 说明 |
|---|---|---|---|
| `path` | `string` | 全部 | 服务器二进制文件路径 |
| `compilation_database` | `string` | tblgen, pdll | 编译数据库 YAML 文件路径 |
| `extra_dirs` | `string[]` | tblgen, pdll | 额外 include 目录 |
| `log` | `string` | 全部 | 日志级别：`"error"`、`"info"` 或 `"verbose"` |
| `pretty` | `bool` | 全部 | 美化 JSON 输出 |

所有字段均为可选。扩展会自动检测工作区中的编译数据库文件；如果检测成功，无需手动配置。

#### SSH 远程开发

使用 Zed SSH 远程开发时，Language Server 运行在远端服务器上。MLIR Suite 的服务器二进制路径应通过 `zed: open server settings` 配置，而不是 `zed: open settings file`；后者打开的是本机 Zed 配置。详见 [SSH remote development](REMOTE_DEVELOPMENT.md)（仅英文）。

#### 示例

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

> Zed 原生的 `binary.path` 和 `binary.arguments` 字段仍然有效，设置后会优先使用。

修改配置后，打开命令面板（macOS 按 `Cmd+Shift+P`，Linux/Windows 按 `Ctrl+Shift+P`）并执行 `zed: restart language server` 以应用更改。

## 截图

### 跳转到定义

![Go to Definition](https://raw.githubusercontent.com/felixtensor/zed-mlir-suite/assets/screenshots/go-to-definition.gif)

### 查找引用

![Find References](https://raw.githubusercontent.com/felixtensor/zed-mlir-suite/assets/screenshots/find-references.gif)

### 悬停 / 签名

![Hover / Signature](https://raw.githubusercontent.com/felixtensor/zed-mlir-suite/assets/screenshots/hover.gif)

### 补全

![Completion](https://raw.githubusercontent.com/felixtensor/zed-mlir-suite/assets/screenshots/completion.gif)

### 诊断

![Diagnostics](https://raw.githubusercontent.com/felixtensor/zed-mlir-suite/assets/screenshots/diagnostics.gif)

### 符号大纲

![Symbol Outline](https://raw.githubusercontent.com/felixtensor/zed-mlir-suite/assets/screenshots/outline.gif)

## 致谢

本扩展基于以下项目构建：

- [MLIR](https://mlir.llvm.org) — LLVM 项目中的多层中间表示框架。
- [tree-sitter-mlir](https://github.com/felixtensor/tree-sitter-mlir) — MLIR 的 Tree-sitter 语法。
- [tree-sitter-tablegen](https://github.com/felixtensor/tree-sitter-tablegen) — TableGen 的 Tree-sitter 语法。
- [tree-sitter-pdll](https://github.com/felixtensor/tree-sitter-pdll) — PDLL 的 Tree-sitter 语法。
- 三个 LSP 服务器（`mlir-lsp-server`、`mlir-pdll-lsp-server`、`tblgen-lsp-server`）是 [LLVM 项目](https://github.com/llvm/llvm-project) 的一部分。

其他编辑器中的 MLIR 工具：

- [vscode-mlir](https://github.com/llvm/vscode-mlir) — 官方的 MLIR、PDLL 和 TableGen VS Code 扩展。
- [mlir-mode](https://github.com/llvm/llvm-project/tree/main/mlir/utils/emacs) — Emacs 主模式及 LSP 客户端，随 LLVM 单体仓库发布。

## 反馈与贡献

本扩展正在积极开发中。请参阅 [路线图](ROADMAP.md) 了解接下来的计划。欢迎各种形式的贡献 — 错误报告、功能请求和拉取请求：

- [提交 Issue](https://github.com/felixtensor/zed-mlir-suite/issues)
- 提交 Pull Request 包含你的改进

## 许可证

Apache License 2.0 with LLVM Exceptions。
