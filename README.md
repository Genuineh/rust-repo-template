# {{project-name}}

> Rust repo template optimized for fast starts and smooth AI collaboration.


## Features ✅

- Rust 2021 edition
- CI: fmt, clippy, test
- Issue / PR templates and contributor guide
- AI collaboration helpers: prompt templates, scripts, guidelines
- Devcontainer / Codespaces ready


## 快速开始 🚀

1. 在 GitHub 上复制本模板仓库（Use "Use this template"）
2. 修改 `Cargo.toml` 中的 `name` / `repository` 等元信息
3. 本地运行：

```bash
cargo build
cargo test
```


## AI 协作指南 💡

- 在 Issue / PR 中附上最小可复现示例（代码 + 期望行为 + 实际行为）。
- 我们提供了 GitHub Issue Forms（Bug / Feature）包含 AI 使用字段，提交 issue 时请使用表单并填写是否使用 AI 与 prompt（若有）。
- 如果在 PR 中使用了 AI（例如 Copilot / LLM），请在 PR 模板内注明「使用了 AI」，并附上 prompt 与简短自检结论。
- 我们在 `.github/ai/prompt_templates.md` 提供了常用 prompt 示例，供复现与检查。


## 开发规范 🔧

- 使用 `rustfmt` 和 `clippy`：
  - `cargo fmt --all`  
  - `cargo clippy --all-targets --all-features -- -D warnings`
- 编写单元测试，并保持测试快速且确定性


## 示例

查看 `examples/hello.rs` 与 `tests/` 了解如何编写最小可复现示例。


---

喜欢的话把这个仓库设为模板（Repository settings -> Template repository），然后点 Use this template 开始新项目。