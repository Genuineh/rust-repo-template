# cosmos — project validator & generator

`cosmos` 是本仓库内提供的命令行工具，用来生成、校验并管理项目模板（本仓库也被 `cosmos` 用作示例模板）。

## 概览

- 二进制名：`cosmos`
- 目的：生成缺失的项目文件（本地写盘或 dry-run）、校验项目结构与 plan 流程、对 AI 输出进行规则化评估并提供可执行建议。

## 常用命令

### 生成模板（dry-run 默认）

列出将要生成/复制的文件（不写盘）:

```bash
cargo run -p rust-repo-template --bin cosmos -- generate --template this_repo --category all
```

写入本地目录（例如 `./out`）:

```bash
cargo run -p rust-repo-template --bin cosmos -- generate --template this_repo --category all --apply --out-dir ./out
```

可选 category：`all`, `basis`, `docs`, `ci`, `tests`, `examples`, `scripts`, `plan`

---

### 校验项目结构

快速校验（presence/heuristics）:

```bash
cargo run -p rust-repo-template --bin cosmos -- validate --level quick
```

完整校验（更多规则和 plan 检查）:

```bash
cargo run -p rust-repo-template --bin cosmos -- validate --level full
```

输出为简洁的 errors/warnings 列表，退出码非零表示存在问题。

---

### plan 管理

列出任务:

```bash
cargo run -p rust-repo-template --bin cosmos -- plan list
```

校验 plan 结构与 task 文件:

```bash
cargo run -p rust-repo-template --bin cosmos -- plan validate
```

`plan validate` 会检查 `plan/todo.toml`、引用的 `task_file` 是否存在、完成的任务是否已归档到 `plan/archive/` 等规则。

---

### AI 评估（rule / llm）

默认的规则评估（无需外部 API）:

```bash
cargo run -p rust-repo-template --bin cosmos -- ai-eval --mode rule
```

LLM 驱动评估（需要配置外部 provider，尚未启用）:

```bash
cargo run -p rust-repo-template --bin cosmos -- ai-eval --mode llm
```

当 LLM 模式被请求但未配置时，命令会以明确错误退出；后续将添加插件接口以安全配置 provider（feature-gated）。

## 配置

将来会支持 `.cosmos.toml` 或 `.repo-guard.toml` 来定制规则开关、忽略列表和（可选）LLM provider 配置。当前工具使用内置默认规则。

## 开发者说明

- 源码入口：`src/bin/cosmos.rs`。
- 模板示例放在：`templates/this_repo.toml`。
- 测试：`cargo test` 包含 CLI 集成测试（`tests/cli_*.rs`）。

---

请把此文档作为快速参考，并在你需要时把更多规则或样例添加到 `docs/` 或模板文件中。