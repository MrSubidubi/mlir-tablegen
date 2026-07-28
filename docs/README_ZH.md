# zed-mlir-suite

[![EN](https://img.shields.io/badge/lang-EN-lightgrey?style=flat-square)](../README.md)
[![中文](https://img.shields.io/badge/lang-中文-red?style=flat-square)](README_ZH.md)

[![CI](https://img.shields.io/github/actions/workflow/status/felixtensor/zed-mlir-suite/ci.yml?style=flat-square&logo=githubactions&logoColor=white&label=CI)](https://github.com/felixtensor/zed-mlir-suite/actions/workflows/ci.yml)
[![Version](https://img.shields.io/github/v/tag/felixtensor/zed-mlir-suite?style=flat-square&logo=github&label=version)](https://github.com/felixtensor/zed-mlir-suite/tags)
[![License](https://img.shields.io/badge/license-Apache%202.0%20with%20LLVM%20Exceptions-blue?style=flat-square&logo=apache&logoColor=white)](../LICENSE)
[![Stars](https://img.shields.io/github/stars/felixtensor/zed-mlir-suite?style=flat-square&logo=github)](https://github.com/felixtensor/zed-mlir-suite/stargazers)

为 [Zed](https://zed.dev) 编辑器提供 [MLIR](https://mlir.llvm.org)、[TableGen](https://llvm.org/docs/TableGen/) 和 [PDLL](https://mlir.llvm.org/docs/PDLL/) 支持。

## 功能特性

- **MLIR、TableGen 和 PDLL 的 Tree-sitter grammar** — 固定使用的 MLIR parser 在解析 [corpus 中的 566 个官方 MLIR 测试文件](https://github.com/felixtensor/tree-sitter-mlir/blob/2d5e709cd733123b5878325cc3c2b29a972e0b1f/examples/README.md)时不会产生 `ERROR` 节点，这些文件覆盖 24 个 dialect 目录；MLIR Suite 的 Zed queries 基于这些语法树提供高亮。
- **C++ raw string 内嵌 MLIR 高亮** — 当 Zed 内置的 C++ grammar 按分隔符注入 `raw_string_content` 时，`R"mlir(…)mlir"` 字符串中的 MLIR 会使用本扩展的 MLIR grammar 高亮。
- **一流的自定义 dialect 支持** — 用户自定义或外部 `dialect.op` 形式均可正确识别和高亮，你的项目自有 dialect 开箱即用。
- **符号大纲** — 在大纲面板中导航 MLIR、TableGen 和 PDLL 符号。
- **集成三种上游 LLVM Language Server**：
  - `mlir-lsp-server` 用于 `.mlir`
  - `mlir-pdll-lsp-server` 用于 `.pdll`
  - `tblgen-lsp-server` 用于 `.td`
- **更丰富的补全标签样式** — MLIR 与 PDLL language server 返回的补全项会按 LSP kind 和 detail 分类着色，数值、块、dialect、操作、类型、属性、约束以及 include 路径都能获得一致的高亮。
- **编辑体验优化** — 括号匹配、自动补全配对符号，以及针对每种语言调优的缩进。

## 前置条件

- [Zed](https://zed.dev) 编辑器
- （可选）LLVM Language Server 用于 LSP 功能 — 详见 [语言支持](#语言支持)。

## 安装

本扩展以 Zed **开发扩展（dev extension）** 方式安装：克隆仓库后将目录交给 Zed，Zed 会在首次安装时将扩展编译为 WebAssembly，因此本地需要可用的 Rust 工具链。

### 安装 Rust 工具链

通过 [rustup](https://rustup.rs) 安装 Rust（stable）。macOS / Linux：

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Windows 下从 [rustup.rs](https://rustup.rs) 下载并运行 `rustup-init.exe`。

Rust 必须通过 `rustup` 安装；Zed 构建开发扩展时会自动处理所需的 WebAssembly target，无需手动添加。

### 克隆仓库

```bash
git clone https://github.com/felixtensor/zed-mlir-suite.git
```

### 作为开发扩展安装

在 Zed 中打开命令面板（macOS 按 `Cmd+Shift+P`，Linux/Windows 按 `Ctrl+Shift+P`），执行 **`zed: install dev extension`** —— 或打开 **Extensions**（`Cmd+Shift+X` / `Ctrl+Shift+X`）并点击 **Install Dev Extension**，然后选择克隆的目录。

Zed 会在安装时构建扩展；首次构建需要拉取依赖，可能耗时一两分钟。若构建失败，请执行 **`zed: open log`**，在 `Zed.log` 中查看详细信息。

## 语言支持

### 基础编辑功能

无需安装 LLVM 二进制文件。若只需 Tree-sitter 高亮、符号大纲、括号匹配和缩进，可在用户配置或项目 `.zed/settings.json` 中禁用 Language Server：

```jsonc
{
  "languages": {
    "MLIR": { "enable_language_server": false },
    "PDLL": { "enable_language_server": false },
    "TableGen": { "enable_language_server": false }
  }
}
```

详细说明见 [Language Server Setup](LANGUAGE_SERVER.md#disable-lsp)（仅英文）。

### LSP 代码智能

安装 `mlir-lsp-server`、`mlir-pdll-lsp-server` 和 `tblgen-lsp-server` 后，可使用各服务器为对应语言提供的 LSP 功能。建议在 Zed 中配置各服务器的二进制文件路径；也可以选择将其所在目录加入当前 worktree 的 `$PATH`。能力矩阵、构建方法、配置项及 SSH 远程开发说明见 [Language Server Setup](LANGUAGE_SERVER.md#configure-lsp)（仅英文）。

## 截图

### 外部 dialect 高亮

![Out-of-Tree Dialect Highlighting](https://raw.githubusercontent.com/felixtensor/zed-mlir-suite/assets/screenshots/downstream-triton.png)

### C++ raw string 内嵌 MLIR

![MLIR in C++ Raw Strings](https://raw.githubusercontent.com/felixtensor/zed-mlir-suite/assets/screenshots/cpp-inject-mlir.png)

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

外部 dialect 高亮截图使用了 [Triton](https://github.com/triton-lang/triton) 仓库中的 TritonGPU 测试文件。

其他编辑器中的 MLIR 工具：

- [vscode-mlir](https://github.com/llvm/vscode-mlir) — 官方的 MLIR、PDLL 和 TableGen VS Code 扩展。
- [mlir-mode](https://github.com/llvm/llvm-project/tree/main/mlir/utils/emacs) — Emacs 主模式及 LSP 客户端，随 LLVM 单体仓库发布。

## 反馈与贡献

开发方向和优先级记录在 [路线图](ROADMAP.md) 中。[贡献指南](../CONTRIBUTING.md)（仅英文）说明了 Issue 报告、开发与验证要求。参与本项目须遵守 [行为准则](../CODE_OF_CONDUCT.md)（仅英文）。

- 通过 [Issue 选择器](https://github.com/felixtensor/zed-mlir-suite/issues/new/choose) 报告错误、提出功能请求或咨询配置问题。
- 提交改动时，请遵循 [拉取请求指南](../CONTRIBUTING.md#pull-requests)。

## 许可证

Apache License 2.0 with LLVM Exceptions。
