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
- 仓库包含对 Copilot 的指令文件（`.github/copilot-instructions.md`）以及 Copilot ephemeral 环境预装示例（`.github/copilot-setup-steps.yml`），用于提高自动化 agent 的构建与测试成功率。
- 项目文档位于 `docs/`，你可以本地用 `mkdocs build` 构建，或启用 GitHub Pages（可选择使用 `.github/workflows/docs-deploy.yml` 自动部署）。

---

## 项目策略与决策说明 🧾

- `Cargo.lock`：**保留**在模板中以便提供可复现的示例构建与一致的 CI 结果（如果你更偏向库模板，可在 fork 后移除并在 README 中注明）。
- 文档归档策略：我们已将较为细节的文档移动到 `docs/archive/`，主要文档集中在 `docs/getting-started.md` 与 `docs/AGENT_INSTRUCTIONS.md` 以便维护者与 AI 更快速定位关键信息。


## 开发规范 🔧

- 使用 `rustfmt` 和 `clippy`：
  - `cargo fmt --all`  
  - `cargo clippy --all-targets --all-features -- -D warnings`
- 代码风格配置：仓库根目录包含 `rustfmt.toml` 与 `.editorconfig`，请在提交前使用 `cargo fmt` 格式化并遵循这些设置。
- Clippy 配置：仓库根目录包含 `clippy.toml`，用于定义 `msrv` 与全局允许的 lint 列表（CI 与 pre-commit 会读取它并将允许项以 `-A` 形式传递给 `cargo clippy`）。
- 推荐：在 VS Code 中启用 `Format on Save`（`.vscode/settings.json` 已包含示例设置）。
- 编写单元测试，并保持测试快速且确定性


## 示例

查看 `examples/hello.rs` 与 `tests/` 了解如何编写最小可复现示例。

## 宇宙工具：cosmos (示例生成与校验) 🔭

本仓库同时作为 `cosmos` CLI 的示例模板。`cosmos` 可以生成、校验并管理项目结构与 `plan/` 流程。本仓库的模板说明位于 `templates/this_repo.toml`。

示例用法（**默认 dry-run**）：
```bash
# 列出将要生成的文件（不写盘）
cargo run -p rust-repo-template --bin cosmos -- generate --template this_repo --category all

# 写入本地目录 ./out
cargo run -p rust-repo-template --bin cosmos -- generate --template this_repo --category all --apply --out-dir ./out

# 进行快速校验
cargo run -p rust-repo-template --bin cosmos -- validate --level quick
```

更多文档：参见 `docs/cosmos.md`。


---

喜欢的话把这个仓库设为模板（Repository settings -> Template repository），然后点 Use this template 开始新项目。