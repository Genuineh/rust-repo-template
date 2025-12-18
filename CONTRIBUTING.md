# 贡献指南

感谢你考虑为本模板贡献！请遵循以下流程：

- Fork → 新分支 → 提交 → 提交 PR。
- 提交前确保通过 `cargo fmt` / `cargo clippy` / `cargo test`。
- 我们提供可选的本地 git hook（运行 `./scripts/install-git-hooks.sh` 安装）来在每次 commit 时自动运行 `cargo fmt` 并执行 `cargo clippy`（若 clippy 报错则会阻止提交）。
  - 安装 pre-commit: `pip install --user pre-commit`
  - 启用并安装钩子（安装脚本会做这一步）：`./scripts/install-git-hooks.sh`
  - 或手动：`pre-commit install --install-hooks`（安装之后本地会自动运行 pre-commit 钩子）
- 在 PR 描述中说明变更意图，并如果使用了 AI（如 Copilot 或 LLM）生成代码，请贴出 prompt 和相关上下文。

PR 模板里包含了一份 checklist，请按要求填写。

修复 bug 或新增特性时请添加或更新 `examples/` 与 `tests/` 中的示例。