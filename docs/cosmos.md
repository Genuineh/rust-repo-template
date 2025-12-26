# cosmos — project validator & generator

`cosmos` 是本仓库内提供的命令行工具，用来生成项目文件（模板）、校验仓库结构，并管理 `plan/` 任务流。

## 运行方式

`cosmos` 是一个二进制（`src/bin/cosmos.rs`）。常见运行方式：

1) 在本仓库里直接运行（包名固定为 `rust-repo-template`）：

```bash
cargo run -p rust-repo-template --bin cosmos -- <cmd> [args]
```

2) 在由模板生成的新仓库里运行（包名通常已变为你的项目名）：

```bash
cargo run --bin cosmos -- <cmd> [args]
```

3) 安装后在任意目录运行（推荐给日常使用）：

```bash
cargo install --path . --bin cosmos
cosmos <cmd> [args]
```

## 命令总览

- `cosmos generate`：从模板生成/同步文件（默认 dry-run）
- `cosmos validate`：校验仓库结构与 plan 规则
- `cosmos plan`：管理 `plan/` 状态机（创建/评审/流转/归档/钩子）
- `cosmos ai`：AI/LLM 配置检查与评估（可选 feature）

## generate（生成/同步模板）

默认是 dry-run（只打印将生成的文件，不写盘）：

```bash
cosmos generate --template default --category all
```

写入到目录（例如 `./out`）：

```bash
cosmos generate --template default --category all --apply --out-dir ./out
```

常用参数：

- `--category`：`all|basis|docs|ci|tests|examples|scripts|plan`
- `--template`：模板名（默认 `default`）
- `--apply`：真正写盘（否则 dry-run）
- `--out-dir` / `-o`：输出目录（默认 `out`）
- `--project-name`：模板变量 `{{project-name}}`
- `--var key=value`：额外模板变量，可多次传入
- `--verify`：生成后在输出目录里跑 `fmt/clippy/test`

“同步”行为（当 `--apply` 且目标目录已存在时）：

- 会检测 **missing / modified / extra** 文件
- 交互模式会要求确认（删除需要额外输入 `DELETE`）
- 非交互：`--yes` 自动确认；如存在删除，还需加 `--allow-delete`
- 默认不会覆盖已存在文件；对比内容不同的文件需 `--force` 才会覆盖

模板来源优先级：

1. 仓库内 `templates/<name>/`（如果存在）
2. 二进制内置模板（embedded templates）
3. 可执行文件相邻的 `templates/<name>/`（适合已安装二进制）

## validate（校验仓库）

```bash
cosmos validate --level quick
```

- `--level quick|full`：目前主要用于预留扩展点（两者校验项高度重叠）
- `--fix`：尝试自动修复部分问题（如 plan 引用路径规范化）

退出码约定：

- 有 **errors**：退出码 `2`（适合 CI）
- 只有 **warnings**：仍返回成功，仅提示

## plan（任务流与钩子）

`plan/` 是一个轻量任务状态机，任务元信息在 `plan/todo.toml`，任务内容在 `plan/tasks/<id>/task.md`，归档后在 `plan/archive/<id>/task.md`。

### 常用命令

列出：

```bash
cosmos plan list
```

创建：

```bash
cosmos plan create --kind feature --title "Add X" --content "..."
```

更新内容/元数据（不直接改 status）：

```bash
cosmos plan update --id 0001 --title "New title" --content "..."
```

评审（`pending_review -> queued` 或拒绝回退/保持）：

```bash
cosmos plan review --id 0001 --decision accept --message "LGTM" --author alice
```

流转：

```bash
cosmos plan start  --id 0001
cosmos plan test   --id 0001
cosmos plan accept --id 0001
cosmos plan finish --id 0001
```

其他：

```bash
cosmos plan show   --id 0001
cosmos plan log    --id 0001 --message "note" --author alice
cosmos plan reopen --id 0001
cosmos plan delete --id 0001 --yes
cosmos plan validate --fix
```

### plan hooks（Python）

你可以在 `scripts/plan-hooks/` 放置 Python 钩子：

- 单文件：`scripts/plan-hooks/<hook>.py`
- 或目录：`scripts/plan-hooks/<hook>/*.py`（按文件名排序执行）

管理命令：

```bash
cosmos plan hooks add   --name pre_finish
cosmos plan hooks list
cosmos plan hooks check --name pre_finish
```

### `--ai-validate`（计划流转前的 AI 校验）

`cosmos plan --ai-validate <subcmd ...>` 会在执行对应动作前生成一份 JSON 报告，并把路径通过环境变量 `PLAN_AI_VALIDATION_PATH` 传给 plan hooks。

注意：如果二进制没有启用 `llm` feature，AI 校验不会阻塞（报告会提示“未启用”）。

## ai（LLM：可选 feature）

```bash
cosmos ai doctor
```

```bash
cosmos ai eval
```

- `ai eval` 需要用 `--features llm` 构建，并配置 `LLM_PROVIDER=stub`（内置 stub provider 仅做演示，会生成 `.cosmos_llm_report.txt`）

## CI 集成

- `.github/workflows/cosmos-validate.yml`：在 PR / push 时运行 `cargo test` + `cosmos validate`
- `.github/workflows/ci.yml`：主流水线（由 `project.toml` 控制启用哪些 job；支持 `.github/custom/before-*.sh`/`after-*.sh`）

## 开发者说明

- 源码入口：`src/bin/cosmos.rs`
- 模板清单（categories/paths）：`templates/default.toml`
- 相关测试：`tests/cli_*.rs`