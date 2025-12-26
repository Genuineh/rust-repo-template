# 使用本模板创建新仓库

本仓库旨在作为 GitHub Template 使用，便于快速新建 Rust 项目并与 AI 协作。

步骤：

1. 点击 GitHub 上的 **Use this template** 创建新仓库。
2. 修改 `Cargo.toml` 中的 `name`、`description`、`repository`、`license` 等字段。
3. 检查并更新 `README.md`、`CONTRIBUTING.md` 中的相关信息（尤其是 AI 协作条目）。
4. 在新仓库里启用 CI（默认已包含 `.github/workflows`）。如果你也希望“新仓库”继续作为模板给其他项目使用，可在仓库设置里将其标记为 Template repository。

AI 协作建议：

- 在 PR 中披露是否使用 AI（例如：`AI usage disclosure`）并粘贴 prompt 与重要输出。
- 使用 `.github/ai/prompt_templates.md` 中的示例作为起点来复现或改进代码。

自定义 CI Hooks：

- 你可以通过可选 hook 脚本扩展固定流水线的各个阶段：把脚本放到 `.github/custom/`，并使用 `before-<stage>.sh` / `after-<stage>.sh` 命名（例如 `.github/custom/before-test.sh`、`.github/custom/after-build.sh`）。
- 当前工作流会识别的 stage：`build`、`test`、`security`、`docs`（详见 CI Hooks 文档）。
- CI 只在对应脚本存在时运行它们；如果未添加脚本，CI 会自动跳过该扩展步骤。
- 请参考 `.github/custom/README.md` 中的约定与示例（模板默认不提供 hook 脚本，以免与团队规范冲突）。

常见定制：
- 替换 `CODEOWNERS`、`LICENSE` 的持有者信息
- 增加/精简 CI jobs（例如添加 `cargo-audit`、`cargo-deny`）

