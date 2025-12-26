# Generate Agent 操作指南 🔧📋

> 目的：为自动化/半自动化的 AI Agent 提供一份清晰、可执行、与 GitHub Agent 准则一致的工作手册，指导如何完成需求、复现与修复 Bug、调试定位问题、进行质量保证与阅读项目文档。

---

## 1. 总体流程（一步步执行） ✅

1. **理解任务 / triage**
   - 阅读 issue、PR 描述或需求说明，确认目标与期望行为。
   - 如信息不够，发评论请求最小可复现步骤（repro）或更多上下文（环境、日志、输入样本）。
2. **复现问题 / 验证需求**
   - 在本地复现问题或确认当前行为是否与预期不符；使用最小可复现示例。 
3. **建立跟踪单元**
   - 若无 issue，则创建一个（包含复现步骤、预期与实际行为与重要日志）。
   - 添加合适 labels（bug、feature、needs-info 等）。
4. **创建分支**
   - 遵循分支命名约定：`fix/<short>-#<issue>`、`feat/<short>-#<issue>` 或 `docs/<short>-#<issue>`。
5. **编写测试（先写测试）**
   - 先添加失败用例（单元或集成测试），确保修复后测试通过。
6. **实现修复 / 功能**
   - 小步提交，保持变更最小化，确保每次提交都能通过本地测试与 lint。
7. **本地 QA 与 CI**
   - 运行 lints、格式化、测试；修复 CI 报错。
8. **提交 PR**
   - PR 应引用关联 issue，写明变更要点、如何验证、测试覆盖情况。
9. **Review、Merge 与发布**
   - 等待审查；合并后按项目流程更新 Changelog / 发布（如适用）。

---

## 2. 调试与定位问题清单 🕵️‍♂️

- 常用命令（在仓库根目录）：
  - `cargo test`（运行所有测试）
  - `cargo test <name>`（运行特定测试）
  - `cargo run --example hello`（运行示例）
  - `cargo clippy --all-targets --all-features -- -D warnings`
  - `cargo fmt -- --check`
  - `RUST_BACKTRACE=1 cargo test` / `RUST_LOG=debug`（启用回溯 / 日志）
- 复现步骤：
  - 获取最小复现示例，缩小输入与影响范围。
  - 使用 `git bisect` 在回归场景中定位引入回归的提交。
  - 在关键点加 `dbg!(...)` 或使用结构化日志（`tracing`）来收集上下文。
- 检查构建 artifacts 与 CI 日志：查看 `target/`、CI 构建日志与 `examples/` 输出。
- 若问题与平台或依赖版本相关：尝试切换 Rust 版本（`rustup`）或在 CI 矩阵中重现。

---

## 3. Bug 修复与代码规范 🔧

- 每个 Bug 修复必须包含对应测试（单元或集成）。
- 遵循最小变更原则，优先在最小范围内修复并保持向后兼容性。
- 遵从仓库的风格配置：使用 `rustfmt.toml`、`clippy.toml`，并在本地运行格式化与 lint。
- 提交信息建议采用 Conventional Commits（例如 `fix(...) : 描述` 或 `feat(...) : 描述`）。
- PR 中应说明：复现步骤、修复原理、测试说明与可能的边界条件。

---

## 4. 质量保证（QA）🧪

- 本地检查：
  - `cargo test`
  - `cargo clippy`（不应有警告）
  - `cargo fmt`（格式化检查）
- CI 要求：所有变更在 CI 上通过（测试、lint、文档构建等）。
- 测试覆盖：关键逻辑应有测试覆盖；建议增加集成测试覆盖真实场景。
- 额外工具（按需）：`cargo-tarpaulin`（覆盖率）、`cargo-fuzz`（模糊测试）。
- 回归防护：对历史回归建立回归测试，防止问题再次出现。

---

## 5. 文档阅读与更新 📚

- 阅读优先级（建议顺序）：
  1. `README.md`
  2. `CONTRIBUTING.md`
  3. `TEMPLATE_USAGE.md`
  4. `docs/index.md` 与 `docs/` 下的相关文档
  5. `Cargo.toml`（依赖、版本）与 `mkdocs.yml`（文档配置）
- 对用户可见行为的变更必须更新相应文档（README / docs / 示例）。
- 若添加新脚本或命令，应在 `examples/` 或 `docs/` 中提供示例与使用步骤。

---

## 6. GitHub / PR 与 Agent 行为准则 🧭

- 遵守仓库 `CONTRIBUTING.md` 中的流程与约定。
- 不要在没有授权的情况下直接向受保护分支推送；使用 PR 流程。
- 切勿在提交或 PR 中包含密钥或敏感信息；若发现 secrets，立即创建私有 issue 并 @maintainers。
- PR 应包含：关联 issue、变更摘要、验证步骤与测试结果。
- 若影响面较大或存在设计性变更，先创建提案（RFC）并请求人工确认。
- 自动化 Agent 在遇到大改动、边界决策或兼容性风险时应主动请求人工复核。

---

## 7. 常用快速命令参考 ⌨️

- Run tests: `cargo test`
- Run single test: `cargo test <test_name>`
- Run example: `cargo run --example hello`
- Lint: `cargo clippy --all-targets --all-features -- -D warnings`
- Format check: `cargo fmt -- --check`
- Build release: `cargo build --release`
- Enable backtrace: `RUST_BACKTRACE=1 cargo test`
- Search code: `rg "pattern" src/`

---

## 8. PR 模板示例（中文） 📝

**标题**：`fix: <简短描述> (#<issue>)`

**正文**：
- 关联 issue: #<issue>
- 变更点概述（2-3 行）
- 如何复现（或如何验证）
- 测试覆盖（新增/修改的测试）
- 额外备注（是否需要迁移、兼容性影响等）

---

## 9. 快速检查清单（PR/提交前） ✅

- [ ] 有 issue 或有说明
- [ ] 已添加失败用例的测试并通过
- [ ] `cargo clippy` 无警告
- [ ] `cargo fmt` 格式化检查通过
- [ ] CI 全绿（或已解释异常）
- [ ] 文档已更新（如需）
- [ ] 无敏感信息提交

---

## 10. 把指南加入仓库的建议 💡

- 建议路径：`docs/AGENT_INSTRUCTIONS.md`（当前已添加）或 `.github/AGENT_INSTRUCTIONS.md`（更靠近流程约定）。
- 已添加并提交：
  - `.github/copilot-instructions.md`（仓库级 Copilot 指令，包含如何构建、测试、lint 以及编码约定）。
  - `.github/copilot-setup-steps.yml`（预安装 Copilot ephemeral 环境的示例步骤，帮助 Copilot 更快完成构建/测试）。
- 推荐进一步可选项：
  - 路径特定指令：`.github/instructions/**/*.instructions.md`（可用 front matter 的 `applyTo` 来限定规则适用的文件集）。
- 可选择的下一步：我可以为你创建额外路径级指令、或创建 issue 征求维护者意见并列出变更清单。请告诉我你的偏好。

---

## 11. 与 GitHub Copilot 的集成（最佳实践） 🤖

以下要点直接参考并对齐了 GitHub 官方“Get the best results” 指导：

- 把 issue 当作 prompt：
  - 任务应**清晰、可限定**，包含问题描述、验收标准（需要单元测试吗）、以及应修改的文件或目录。将 issue 视为 AI 的 prompt 可以显著提升结果质量。
- 任务类型选择建议：
  - 适合交给 Copilot 的任务：修复小规模 bug、改善测试覆盖、文档改进、可局部实现的功能、可重复的低风险改动、可自动验证的改进（如格式、lint）。
  - 不适合交给 Copilot 的任务：广泛的重构、需要深领域知识或业务逻辑判断的大改动、敏感或生产关键问题（安全、认证、PII、事件响应）、含糊或开放式的需求。
- 在 PR 上与 Copilot 互动：
  - 可以在 PR 评论中 mention `@copilot` 并说明需要改进的点；建议把评论**批量提交**（Start a review）以便 Copilot 一次性处理多个改动请求。
  - Copilot 仅响应具有写权限的用户评论。
- 仓库级与路径级自定义指令：
  - 仓库级：添加 `.github/copilot-instructions.md` 来说明如何构建、测试与项目约定（示例见下方）。
  - 路径级：使用 `.github/instructions/**/*.instructions.md` 并在 front matter 使用 `applyTo`（glob）来限定，例如针对 `**/tests/*.spec.ts` 的测试规范。
- 预安装依赖：
  - 可添加 `.github/copilot-setup-steps.yml`，在 Copilot 的临时开发环境中预装工具与依赖，能显著提高 Copilot 在首次运行时的效率和成功率。
- 组织级指令与优先级：
  - Copilot 会优先使用仓库级指令，其次是组织级指令；合理设置组织级规则能统一多仓库行为。
- MCP 与自定义 agent：
  - 若仓库使用了 MCP（Model Context Protocol）或需要特殊工具链，可配置 MCP 服务或创建专用 agent（例如：测试专员、文档专家或语言特定 agent）。自定义 agent 可绑定特定工具权限，限制写操作范围以降低风险。

---

### 仓库级 `copilot-instructions.md` 示例（模板）

```
This repository is a Rust template for small command-line tools and libraries. Please follow these guidelines when Copilot is assigned tasks:

## Development
- Build: `cargo build --workspace`
- Test: `cargo test --workspace`
- Format check: `cargo fmt -- --check`
- Lint: `cargo clippy --all-targets --all-features -- -D warnings`

## Testing & CI
- All changes must pass the workspace tests and `cargo clippy` without warnings.

## Repository structure
- `src/`: library and binary sources
- `examples/`: example programs
- `docs/`: project docs

## Guidelines
- Add unit tests for bug fixes and new features
- Keep changes small and well-scoped
```

> 将该文件放置在仓库根目录下：`.github/copilot-instructions.md`。

---

### `copilot-setup-steps.yml` 示例（片段）

```
# 用于在 Copilot ephemeral 环境中预安装依赖
steps:
  - name: install-rustup
    run: rustup toolchain install stable --no-self-update
  - name: install-cargo-tools
    run: cargo install cargo-clippy cargo-tarpaulin || true
  - name: verify
    run: cargo --version && rustc --version
```

---

## 12. 结语与下一步建议 ✅

- 本模板包含 `plan/` 工作流目录结构（`plan/todo.toml`、`plan/tasks/`、`plan/archive/`）。
- 本仓库提供了用于管理/校验 plan 的脚本：`scripts/plan_create.py`、`scripts/plan_close.py`、`scripts/validate_plan.py`。
- CI 中包含 plan 校验工作流（见 `.github/workflows/validate-plan.yml`）。

如果你希望进一步加强工作流（例如增加更严格的校验或将部分校验加入 pre-commit），建议先在 issue 中写清楚验收标准再改动。

---

*本文档可由 AI 辅助生成/更新；以仓库当前代码与 CI 配置为准。*
