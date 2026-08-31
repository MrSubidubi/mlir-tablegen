# Zed MLIR

[![EN](https://img.shields.io/badge/lang-EN-lightgrey?style=flat-square)](../README.md)
[![中文](https://img.shields.io/badge/lang-中文-red?style=flat-square)](README_ZH.md)

[![CI](https://img.shields.io/github/actions/workflow/status/felixtensor/mlir-tablegen/ci.yml?style=flat-square&logo=githubactions&logoColor=white&label=CI)](https://github.com/felixtensor/mlir-tablegen/actions/workflows/ci.yml)
[![Version](https://img.shields.io/github/v/tag/felixtensor/mlir-tablegen?style=flat-square&logo=github&label=version)](https://github.com/felixtensor/mlir-tablegen/tags)
[![License](https://img.shields.io/badge/license-Apache%202.0%20with%20LLVM%20Exceptions-blue?style=flat-square&logo=apache&logoColor=white)](../LICENSE)
[![Stars](https://img.shields.io/github/stars/felixtensor/mlir-tablegen?style=flat-square&logo=github)](https://github.com/felixtensor/mlir-tablegen/stargazers)

为 [Zed](https://zed.dev) 编辑器提供 [MLIR](https://mlir.llvm.org)、[TableGen](https://llvm.org/docs/TableGen/) 和 [PDLL](https://mlir.llvm.org/docs/PDLL/) 支持。

## 功能特性

- **MLIR、TableGen 和 PDLL 的 Tree-sitter grammar** — 固定使用的 MLIR parser 在解析[从官方 MLIR 测试套件中精选的用例集](https://github.com/felixtensor/tree-sitter-mlir/blob/main/examples/README.md)时不会产生 `ERROR` 节点；本扩展的 Zed queries 基于这些语法树提供高亮。
- **C++ raw string 内嵌 MLIR 高亮** — 当 Zed 内置的 C++ grammar 按分隔符注入 `raw_string_content` 时，`R"mlir(…)mlir"` 字符串中的 MLIR 会使用本扩展的 MLIR grammar 高亮。
- **一流的自定义 dialect 支持** — 用户自定义或外部 `dialect.op` 形式均可正确识别和高亮，你的项目自有 dialect 开箱即用。
- **符号大纲** — 在大纲面板中导航 MLIR、TableGen 和 PDLL 符号。
- **集成三种上游 LLVM Language Server**：
  - `mlir-lsp-server` 用于 `.mlir`
  - `mlir-pdll-lsp-server` 用于 `.pdll`
  - `tblgen-lsp-server` 用于 `.td`
- **更丰富的补全标签样式** — MLIR 与 PDLL language server 返回的补全项会按 LSP kind 和 detail 分类着色，数值、块、dialect、操作、类型、属性、约束以及 include 路径都能获得一致的高亮。
- **编辑体验优化** — 括号匹配、自动补全配对符号，以及针对每种语言调优的缩进。
- **Vim 文本对象** — 三种语言均提供函数和注释文本对象，语法中有对应结构的还提供类对象，以及基于它们的移动命令。

## 前置条件

- [Zed](https://zed.dev/download) 编辑器
- （可选）LLVM Language Server 用于 LSP 功能 — 详见 [语言支持](#语言支持)。

## 安装

在 macOS 按 `Cmd+Shift+X`、Linux/Windows 按 `Ctrl+Shift+X` 打开扩展面板 —— 或从菜单栏选择 **Zed > Extensions** —— 搜索 **MLIR** 并点击 **Install**。除此之外无需任何准备：不需要 Rust 工具链，也不需要 LLVM 二进制文件。

### 作为开发扩展安装

若想改用本地检出运行 —— 例如试用尚未发布的改动，或参与本扩展的开发 —— 先克隆仓库：

```bash
git clone https://github.com/felixtensor/mlir-tablegen.git
```

然后打开命令面板（macOS 按 `Cmd+Shift+P`，Linux/Windows 按 `Ctrl+Shift+P`），执行 **`zed: install dev extension`** 并选择克隆的目录。Zed 会在安装时将扩展编译为 WebAssembly，因此这条路径需要通过 [rustup](https://rustup.rs) 安装的 stable Rust 工具链。首次构建需要拉取依赖，可能耗时一两分钟；若失败，执行 **`zed: open log`** 查看详细信息。

开发环境配置见 [CONTRIBUTING.md](../CONTRIBUTING.md)（仅英文）；关于 Zed 扩展的结构，可参考 Zed 文档的 [Developing Extensions](https://zed.dev/docs/extensions/developing-extensions) 章节（仅英文）。

## 语言支持

### 基础编辑功能

无需安装 LLVM 二进制文件。若只需 Tree-sitter 高亮、符号大纲、括号匹配、缩进和 Vim 文本对象，可在用户配置或项目 `.zed/settings.json` 中禁用 Language Server：

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

安装 `mlir-lsp-server`、`mlir-pdll-lsp-server` 和 `tblgen-lsp-server` 后，可使用各服务器为对应语言提供的 LSP 功能。建议在 Zed 中配置各服务器的二进制文件路径；也可以选择将其所在目录加入当前 worktree 的 `$PATH`。能力矩阵、构建方法和配置项见 [Language Server Setup](LANGUAGE_SERVER.md#configure-lsp)（仅英文），SSH 远程开发见 [SSH Remote Development](REMOTE_DEVELOPMENT.md)（仅英文）。

### Vim 模式

在 Vim 模式下，`af` / `if` 和 `ac` / `ic` 分别选择函数和类，`gc` 选取光标周围连续的注释，`]m` 在函数间跳转：

| 语言 | 函数（`af` / `if`、`]m`） | 类（`ac` / `ic`） |
| --- | --- | --- |
| MLIR | `func.func`、`llvm.func` | `module`、`builtin.module` |
| PDLL | 顶层 `Pattern`、`Constraint`、`Rewrite` | — |
| TableGen | 具名 `def`、`defm` | `class`、`multiclass` |

各语言的映射如何对应到 grammar 的节点结构，以及 MLIR 中移动命令的深度限制，见 [Vim Mode](VIM_MODE.md)（仅英文）。

## 截图

### 外部 dialect 高亮

![Out-of-Tree Dialect Highlighting](https://raw.githubusercontent.com/felixtensor/mlir-tablegen/assets/screenshots/downstream-triton.png)

### C++ raw string 内嵌 MLIR

![MLIR in C++ Raw Strings](https://raw.githubusercontent.com/felixtensor/mlir-tablegen/assets/screenshots/cpp-inject-mlir.png)

### 跳转到定义

![Go to Definition](https://raw.githubusercontent.com/felixtensor/mlir-tablegen/assets/screenshots/go-to-definition.gif)

### 查找引用

![Find References](https://raw.githubusercontent.com/felixtensor/mlir-tablegen/assets/screenshots/find-references.gif)

### 悬停 / 签名

![Hover / Signature](https://raw.githubusercontent.com/felixtensor/mlir-tablegen/assets/screenshots/hover.gif)

### 补全

![Completion](https://raw.githubusercontent.com/felixtensor/mlir-tablegen/assets/screenshots/completion.gif)

### 诊断

![Diagnostics](https://raw.githubusercontent.com/felixtensor/mlir-tablegen/assets/screenshots/diagnostics.gif)

### 符号大纲

![Symbol Outline](https://raw.githubusercontent.com/felixtensor/mlir-tablegen/assets/screenshots/outline.gif)

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

开发方向和优先级记录在 [路线图](ROADMAP.md) 中（仅英文）。[贡献指南](../CONTRIBUTING.md)（仅英文）说明了 Issue 报告、开发与验证要求。

- 通过 [Issue 选择器](https://github.com/felixtensor/mlir-tablegen/issues/new/choose) 报告错误、提出功能请求或咨询配置问题。
- 提交改动时，请遵循 [拉取请求指南](../CONTRIBUTING.md#pull-requests)。

## 许可证

Apache License 2.0 with LLVM Exceptions。
