use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use glob::glob;
use include_dir::{include_dir, Dir};
use tempfile::TempDir;

use serde::Deserialize;

/// Simple project validator & generator (cosmos)
#[derive(Parser)]
#[command(name = "cosmos")]
#[command(version)]
#[command(about = "Manage and validate project templates", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate files from a template
    Generate {
        /// Category to generate (all|basis|docs|ci|tests|examples|scripts|plan)
        #[arg(short, long, default_value_t = String::from("all"))]
        category: String,
        /// Destination directory (default: ./out)
        #[arg(short, long, value_name = "DIR", default_value = "out")]
        out_dir: PathBuf,
        /// Actually write files to disk (default: dry-run)
        #[arg(long)]
        apply: bool,
        /// Non-interactive: automatically apply fixes (assume yes)
        #[arg(long, default_value_t = false)]
        yes: bool,
        /// Allow automatic deletions when used with --yes
        #[arg(long, default_value_t = false)]
        allow_delete: bool,
        /// Force overwrites of certain protected files like Cargo.toml/project.toml
        #[arg(long, default_value_t = false)]
        force: bool,
        /// Template name (default: default)
        #[arg(long, default_value = "default")]
        template: String,
        /// Alternative template directory to use instead of repository 'templates/<name>'
        #[arg(long, value_name = "DIR")]
        template_dir: Option<PathBuf>,
        /// Project name to use for templating (used for {{project-name}})
        #[arg(long)]
        project_name: Option<String>,
        /// Additional template variables in key=value form; may be repeated
        #[arg(long = "var", value_parser = parse_key_val, num_args=0..)]
        vars: Vec<(String, String)>,
        /// After generation, run verification steps (fmt/clippy/test)
        #[arg(long)]
        verify: bool,
    },

    /// Validate repository / template
    Validate {
        /// Quick vs full checks
        #[arg(short, long, default_value_t = String::from("quick"))]
        level: String,
        /// Attempt to automatically fix issues when possible
        #[arg(long, default_value_t = false)]
        fix: bool,
    },

    /// Third-party AI / LLM related commands
    Ai {
        #[command(subcommand)]
        sub: AiCmd,
    },

    /// Plan-related commands
    Plan {
        /// Run AI validation before each plan state transition (runs before user hooks)
        #[arg(long, default_value_t = false)]
        ai_validate: bool,
        #[command(subcommand)]
        sub: PlanCmd,
    },

    /// Project manifest related commands
    Project {
        #[command(subcommand)]
        sub: ProjectCmd,
    },
}

#[derive(Subcommand)]
enum ProjectCmd {
    /// Emit GitHub Actions outputs parsed from project.toml (writes to $GITHUB_OUTPUT if set)
    GhaOutputs {},

    /// Validate project.toml against Cargo.toml and artifact settings
    Validate {
        /// Treat warnings as errors
        #[arg(long, default_value_t = false)]
        strict: bool,
    },
}

#[derive(Subcommand)]
enum PlanCmd {
    /// List plan tasks
    List {},
    /// Validate plan structure and task files
    Validate {
        task: Option<String>,
        #[arg(long, default_value_t = false)]
        fix: bool,
    },
    /// Manage plan hook scripts (add/list/check)
    Hooks {
        #[command(subcommand)]
        sub: HooksCmd,
    },
    /// Create a new task in plan
    Create {
        /// Kind: bug or feature
        #[arg(short, long, default_value = "feature")]
        kind: String,
        /// Title for the task
        #[arg(short, long)]
        title: String,
        /// initial content for the task file
        #[arg(long)]
        content: Option<String>,
        /// assignee
        #[arg(long)]
        assignee: Option<String>,
    },
    /// Update an existing task (metadata and content only — status changes via specific actions)
    Update {
        /// Task id to update
        #[arg(short, long)]
        id: String,
        /// New title
        #[arg(long)]
        title: Option<String>,
        /// New assignee
        #[arg(long)]
        assignee: Option<String>,
        /// Replace task file content
        #[arg(long)]
        content: Option<String>,
    },
    /// Review a pending task (decision: accept|reject)
    Review {
        #[arg(short, long)]
        id: String,
        /// decision: accept | reject
        #[arg(long)]
        decision: String,
        /// optional message recorded in history
        #[arg(long)]
        message: Option<String>,
        /// reviewer name
        #[arg(long)]
        author: Option<String>,
    },
    /// Start working on a queued task (queued -> working)
    Start {
        #[arg(short, long)]
        id: String,
        #[arg(long)]
        message: Option<String>,
        #[arg(long)]
        author: Option<String>,
    },
    /// Run tests on a working task (working -> testing)
    Test {
        #[arg(short, long)]
        id: String,
        #[arg(long)]
        message: Option<String>,
        #[arg(long)]
        author: Option<String>,
    },
    /// Mark tests accepted (testing -> under_acceptance)
    Accept {
        #[arg(short, long)]
        id: String,
        #[arg(long)]
        message: Option<String>,
        #[arg(long)]
        author: Option<String>,
    },
    /// Finish an accepted task (under_acceptance -> finished)
    Finish {
        #[arg(short, long)]
        id: String,
        #[arg(long)]
        message: Option<String>,
        #[arg(long)]
        author: Option<String>,
    },
    /// Reopen a finished task (archive -> tasks, status -> pending_review)
    Reopen {
        #[arg(short, long)]
        id: String,
        #[arg(long)]
        message: Option<String>,
        #[arg(long)]
        author: Option<String>,
    },
    /// Delete a task
    Delete {
        /// Task id to delete
        #[arg(short, long)]
        id: String,
        /// Confirm deletion without prompt
        #[arg(long, default_value_t = false)]
        yes: bool,
    },
    /// Log an event into a task's history
    Log {
        /// Task id
        #[arg(short, long)]
        id: String,
        /// Message to append to history
        #[arg(short, long)]
        message: String,
        /// Author of the log entry
        #[arg(long)]
        author: Option<String>,
    },
    /// Show details of a task
    Show {
        /// Task id
        #[arg(short, long)]
        id: String,
    },
}

#[derive(Subcommand)]
enum AiCmd {
    /// Show how AI/LLM is configured for this repo
    Doctor {},
    /// Run an AI/LLM evaluation across the repo
    Eval {},
}

#[derive(Subcommand)]
enum HooksCmd {
    /// Create a new hook script from a template
    Add {
        #[arg(short, long)]
        name: String,
    },
    /// List available hook scripts
    List {},
    /// Check syntax and entrypoint for a hook (or all if omitted)
    Check {
        #[arg(short, long)]
        name: Option<String>,
    },
}

#[derive(serde::Serialize)]
struct AiValidationReport {
    tool: &'static str,
    kind: &'static str,
    task_id: String,
    from_status: String,
    to_status: String,
    ok: bool,
    summary: String,
    suggestions: Vec<String>,
}

fn task_root_dir(repo_root: &Path, id: &str) -> PathBuf {
    let tasks = repo_root.join("plan/tasks").join(id);
    if tasks.exists() {
        return tasks;
    }
    repo_root.join("plan/archive").join(id)
}

fn get_task_context(repo_root: &Path, id: &str) -> Result<(String, String)> {
    let plan = read_todo(repo_root)?;
    for t in plan.task {
        if t.id == id {
            return Ok((t.status.unwrap_or_default(), t.task_file.unwrap_or_default()));
        }
    }
    anyhow::bail!("plan: task '{}' not found", id)
}

fn write_ai_validation_report(
    repo_root: &Path,
    id: &str,
    from_status: &str,
    to_status: &str,
    report: &AiValidationReport,
) -> Result<PathBuf> {
    let root = task_root_dir(repo_root, id);
    let reports_dir = root.join("reports");
    fs::create_dir_all(&reports_dir)?;
    let path = reports_dir.join("ai_validation.json");
    fs::write(&path, serde_json::to_string_pretty(report)?)?;
    // Also keep a per-transition copy for audit/debugging.
    let per = reports_dir.join(format!("ai_validation_{}_to_{}.json", from_status, to_status));
    let _ = fs::write(&per, serde_json::to_string_pretty(report)?);
    Ok(path)
}

fn ai_validate_transition(
    repo_root: &Path,
    id: &str,
    from_status: &str,
    to_status: &str,
) -> Result<AiValidationReport> {
    // Default behavior: if LLM isn't enabled/available, we don't block.
    #[cfg(feature = "llm")]
    {
        let res = rust_repo_template::llm::evaluate_with_llm(repo_root);
        match res {
            Ok(()) => Ok(AiValidationReport {
                tool: "cosmos",
                kind: "plan_transition",
                task_id: id.to_string(),
                from_status: from_status.to_string(),
                to_status: to_status.to_string(),
                ok: true,
                summary: "LLM validation completed (stub)".to_string(),
                suggestions: vec![
                    "Review the generated .cosmos_llm_report.txt for details".to_string()
                ],
            }),
            Err(e) => Ok(AiValidationReport {
                tool: "cosmos",
                kind: "plan_transition",
                task_id: id.to_string(),
                from_status: from_status.to_string(),
                to_status: to_status.to_string(),
                ok: true,
                summary: format!("LLM validation unavailable/non-fatal: {}", e),
                suggestions: vec![
                    "Enable and configure an LLM provider to enforce checks".to_string()
                ],
            }),
        }
    }
    #[cfg(not(feature = "llm"))]
    {
        let _ = repo_root;
        Ok(AiValidationReport {
            tool: "cosmos",
            kind: "plan_transition",
            task_id: id.to_string(),
            from_status: from_status.to_string(),
            to_status: to_status.to_string(),
            ok: true,
            summary: "LLM support not enabled in this build".to_string(),
            suggestions: vec![
                "Rebuild with `--features llm` to enable third-party AI validation".to_string()
            ],
        })
    }
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct TemplateManifest {
    name: Option<String>,
    categories: HashMap<String, Vec<String>>,
}

fn load_manifest(repo_root: &Path, template_name: &str) -> Result<Option<TemplateManifest>> {
    let path = repo_root.join("templates").join(format!("{}.toml", template_name));
    if path.exists() {
        let s =
            fs::read_to_string(&path).with_context(|| format!("reading manifest {:?}", path))?;
        let m: TemplateManifest = toml::from_str(&s).context("parsing manifest")?;
        Ok(Some(m))
    } else {
        Ok(None)
    }
}

fn expand_patterns(repo_root: &Path, patterns: &[String]) -> Result<Vec<PathBuf>> {
    let mut matches = Vec::new();
    for pat in patterns {
        // Patterns are relative to repo root
        let full = repo_root.join(pat);
        let pattern = full.to_string_lossy().to_string();
        for p in (glob(&pattern).context("globbing pattern")?).flatten() {
            // skip .git and target directories
            if p.components().any(|c| c.as_os_str() == ".git" || c.as_os_str() == "target") {
                continue;
            }
            matches.push(p);
        }
    }
    matches.sort();
    matches.dedup();
    Ok(matches)
}

fn copy_paths_to(repo_root: &Path, paths: &[PathBuf], dest: &Path, force: bool) -> Result<()> {
    for p in paths {
        let rel = p.strip_prefix(repo_root).unwrap_or(p);
        let destpath = dest.join(rel);
        if let Some(parent) = destpath.parent() {
            fs::create_dir_all(parent)?;
        }
        if p.is_dir() {
            // create dir marker
            fs::create_dir_all(&destpath)?;
        } else {
            if destpath.exists() && !force {
                println!("Skipping existing file: {}", destpath.display());
                continue;
            }
            fs::copy(p, &destpath).with_context(|| format!("copy {:?} to {:?}", p, destpath))?;
        }
    }
    Ok(())
}

fn validate_repo(repo_root: &Path, _level: &str, fix: bool) -> Result<(Vec<String>, Vec<String>)> {
    let mut errors = Vec::new();
    let mut warnings = Vec::new();

    // required basis files
    for b in &["Cargo.toml", "README.md", "LICENSE", "CONTRIBUTING.md"] {
        if !repo_root.join(b).exists() {
            errors.push(format!("Missing required file: {}", b));
        }
    }

    // docs
    if !repo_root.join("docs").exists() {
        warnings.push("docs/ missing".to_string());
    }

    // workflows
    let wf = repo_root.join(".github/workflows");
    if !wf.exists() {
        warnings.push(".github/workflows missing".to_string());
    } else {
        let mut found = false;
        for ent in
            wf.read_dir().with_context(|| format!("reading workflows dir {:?}", wf))?.flatten()
        {
            let p = ent.path();
            if p.extension()
                .and_then(|s| s.to_str())
                .map(|s| s == "yml" || s == "yaml")
                .unwrap_or(false)
            {
                found = true;
                break;
            }
        }
        if !found {
            warnings.push("No workflow yml files found under .github/workflows".to_string());
        }
    }

    // scripts
    if !repo_root.join("scripts").exists() {
        warnings.push("scripts/ missing".to_string());
    } else if !repo_root.join("scripts/validate_plan.py").exists() {
        warnings.push("scripts/validate_plan.py missing".to_string());
    }

    // plan checks
    let plan_issues = validate_plan(repo_root)?;
    for p in plan_issues.clone() {
        errors.push(p);
    }

    // project.toml consistency checks (blocking on errors)
    if repo_root.join("project.toml").exists() {
        let report =
            validate_project_manifest(repo_root, false).context("validating project.toml")?;
        errors.extend(report.errors);
        warnings.extend(report.warnings);
    }

    // AI heuristics
    let ai_w = check_ai_heuristics(repo_root)?;
    for w in ai_w {
        warnings.push(w);
    }

    // attempt auto-fixes if requested and we found issues
    if fix {
        let mut applied = Vec::new();
        // Only attempt fixes when there are errors or missing files
        if !errors.is_empty() || !warnings.is_empty() {
            let fixes = auto_fix_repo(repo_root)?;
            for f in fixes {
                applied.push(f);
            }
            // Re-run validations to update lists
            let (new_errors, new_warnings) = validate_repo(repo_root, _level, false)?;
            errors = new_errors;
            warnings = new_warnings;
            if !applied.is_empty() {
                println!("Applied fixes:");
                for f in applied {
                    println!(" - {}", f);
                }
            }
        }
    }

    Ok((errors, warnings))
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, serde::Serialize)]
struct PlanTodo {
    meta: Option<toml::Value>,
    #[serde(default)]
    task: Vec<PlanTask>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, serde::Serialize, Clone)]
struct PlanTask {
    id: String,
    kind: Option<String>, // "bug" or "feature"
    title: Option<String>,
    status: Option<String>, // pending_review|queued|working|testing|under_acceptance|finished
    assignee: Option<String>,
    task_file: Option<String>,
}

// Helpers to read/write todo.toml and manage next_id
fn read_todo(repo_root: &Path) -> Result<PlanTodo> {
    let todo = repo_root.join("plan/todo.toml");
    if !todo.exists() {
        return Ok(PlanTodo { meta: None, task: Vec::new() });
    }
    let s = fs::read_to_string(&todo).context("reading plan/todo.toml")?;
    let plan: PlanTodo = toml::from_str(&s).context("parsing plan/todo.toml")?;
    Ok(plan)
}

fn write_todo(repo_root: &Path, plan: &PlanTodo) -> Result<()> {
    let plan_dir = repo_root.join("plan");
    fs::create_dir_all(&plan_dir)?;
    let s = toml::to_string_pretty(plan).context("serializing todo.toml")?;
    fs::write(plan_dir.join("todo.toml"), s).context("writing plan/todo.toml")?;
    Ok(())
}

fn read_next_id(repo_root: &Path) -> Result<u32> {
    let p = repo_root.join("plan/next_id.txt");
    if p.exists() {
        let s = fs::read_to_string(p)?.trim().to_string();
        if let Ok(n) = s.parse::<u32>() {
            return Ok(n);
        }
    }
    // fallback: compute max existing id + 1
    let mut max = 0u32;
    let plan = read_todo(repo_root)?;
    for t in plan.task {
        if let Ok(n) = t.id.parse::<u32>() {
            if n > max {
                max = n;
            }
        }
    }
    Ok(max + 1)
}

fn write_next_id(repo_root: &Path, n: u32) -> Result<()> {
    let plan_dir = repo_root.join("plan");
    fs::create_dir_all(&plan_dir)?;
    fs::write(plan_dir.join("next_id.txt"), format!("{:04}\n", n))?;
    Ok(())
}

fn validate_plan(repo_root: &Path) -> Result<Vec<String>> {
    let mut issues = Vec::new();
    let plan_dir = repo_root.join("plan");
    let todo = plan_dir.join("todo.toml");
    if !todo.exists() {
        issues.push("plan/todo.toml missing".to_string());
        return Ok(issues);
    }
    let s = fs::read_to_string(&todo).context("reading plan/todo.toml")?;
    let plan: PlanTodo = toml::from_str(&s).context("parsing plan/todo.toml")?;

    if plan.task.is_empty() {
        issues.push("plan: no tasks found in todo.toml".to_string());
    }

    let allowed_status =
        ["pending_review", "queued", "working", "testing", "under_acceptance", "finished"];

    for t in &plan.task {
        if t.id.trim().is_empty() {
            issues.push(format!("plan: task with empty id: {:?}", t.title));
            continue;
        }
        if let Some(kind) = &t.kind {
            if kind != "bug" && kind != "feature" {
                issues.push(format!(
                    "plan: task {} has invalid kind '{}', must be 'bug' or 'feature'",
                    t.id, kind
                ));
            }
        }
        if let Some(st) = &t.status {
            if !allowed_status.contains(&st.as_str()) {
                issues.push(format!("plan: task {} has invalid status '{}'", t.id, st));
            }
        }
        if let Some(tf) = &t.task_file {
            let p = plan_dir.join(tf);
            if !p.exists() {
                issues.push(format!("plan: referenced task_file '{}' not found", tf));
            } else {
                // if task is finished, ensure it is in archive/
                if let Some(st) = &t.status {
                    if st == "finished" && !tf.starts_with("archive/") {
                        issues.push(format!(
                            "plan: task {} marked finished but task_file '{}' is not in archive/",
                            t.id, tf
                        ));
                    }
                }
            }
        } else {
            issues.push(format!("plan: task {} missing task_file", t.id));
        }
    }

    let tasks = repo_root.join("plan/tasks");
    if !tasks.exists() {
        issues.push("plan/tasks/ missing".to_string());
    }
    Ok(issues)
}

use std::process::Command;

fn check_ai_heuristics(repo_root: &Path) -> Result<Vec<String>> {
    let mut warnings = Vec::new();
    if !repo_root.join(".github/copilot-instructions.md").exists()
        && !repo_root.join(".github/ai").exists()
    {
        warnings.push("No AI guidelines or .github/copilot-instructions.md found".to_string());
    }
    let readme = fs::read_to_string(repo_root.join("README.md")).unwrap_or_default();
    if !readme.to_lowercase().contains("ai") {
        warnings.push("README doesn't mention AI collaboration guidance".to_string());
    }
    Ok(warnings)
}

fn append_history(
    repo_root: &Path,
    id: &str,
    message: Option<&str>,
    author: Option<&str>,
) -> Result<()> {
    // prefer tasks dir, fallback to archive
    let mut history_dir = repo_root.join("plan/tasks").join(id).join("history");
    if !history_dir.exists() {
        history_dir = repo_root.join("plan/archive").join(id).join("history");
    }
    fs::create_dir_all(&history_dir)?;
    let ts = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
    let filename = format!("{}.md", ts);
    let path = history_dir.join(&filename);
    let mut contents = String::new();
    contents.push_str(&format!("time: {}\n", ts));
    if let Some(a) = author {
        contents.push_str(&format!("author: {}\n", a));
    }
    contents.push_str("---\n");
    if let Some(m) = message {
        contents.push_str(m);
    }
    fs::write(&path, contents)?;
    Ok(())
}

/// Run a user-provided hook script if present. Scripts are looked up under
/// scripts/plan-hooks/<hook_name>.py or scripts/plan-hooks/<hook_name>/*.py
/// Scripts are Python files and receive context via environment variables:
/// - PLAN_TASK_ID, PLAN_REPO_ROOT, PLAN_CURRENT_STATUS, PLAN_NEW_STATUS, PLAN_TASK_FILE
fn run_user_hook(
    repo_root: &Path,
    hook_name: &str,
    id: &str,
    new_status: Option<&str>,
    ai_validation_path: Option<&Path>,
) -> Result<()> {
    let hooks_dir = repo_root.join("scripts/plan-hooks");
    let single = hooks_dir.join(format!("{}.py", hook_name));
    let dir = hooks_dir.join(hook_name);
    let mut scripts: Vec<std::path::PathBuf> = Vec::new();
    if single.exists() {
        scripts.push(single);
    }
    if dir.exists() && dir.is_dir() {
        let mut entries: Vec<_> = fs::read_dir(&dir)?.filter_map(|e| e.ok()).collect();
        entries.sort_by_key(|e| e.path());
        for e in entries {
            let p = e.path();
            if p.is_file() && p.extension().and_then(|s| s.to_str()) == Some("py") {
                scripts.push(p);
            }
        }
    }
    if scripts.is_empty() {
        return Ok(());
    }
    // gather task info
    let (cur_status, tf) = get_task_context(repo_root, id)?;
    for s in scripts {
        let mut cmd = Command::new("python3");
        cmd.arg(s.as_os_str());
        cmd.env("PLAN_TASK_ID", id);
        cmd.env("PLAN_REPO_ROOT", repo_root);
        cmd.env("PLAN_CURRENT_STATUS", &cur_status);
        cmd.env("PLAN_NEW_STATUS", new_status.unwrap_or(""));
        cmd.env("PLAN_TASK_FILE", &tf);
        if let Some(p) = ai_validation_path {
            cmd.env("PLAN_AI_VALIDATION_PATH", p);
        }

        // Also provide a JSON context to stdin for scripts that want structured input.
        let ctx = serde_json::json!({
            "task_id": id,
            "repo_root": repo_root,
            "current_status": cur_status,
            "new_status": new_status.unwrap_or(""),
            "task_file": tf,
            "ai_validation_path": ai_validation_path.map(|p| p.to_string_lossy().to_string()).unwrap_or_default(),
        });
        cmd.stdin(std::process::Stdio::piped());
        cmd.stdout(std::process::Stdio::piped());
        cmd.stderr(std::process::Stdio::piped());

        let mut child = cmd.spawn().context("spawning hook script")?;
        if let Some(mut stdin) = child.stdin.take() {
            use std::io::Write;
            let _ = stdin.write_all(serde_json::to_string(&ctx)?.as_bytes());
        }
        let output = child.wait_with_output().context("running hook script")?;
        if !output.status.success() {
            let mut msg = String::new();
            if !output.stdout.is_empty() {
                msg.push_str(&String::from_utf8_lossy(&output.stdout));
            }
            if !output.stderr.is_empty() {
                if !msg.is_empty() {
                    msg.push('\n');
                }
                msg.push_str(&String::from_utf8_lossy(&output.stderr));
            }
            anyhow::bail!("hook '{}' failed for {}: {}", hook_name, id, msg.trim());
        }
    }
    Ok(())
}

fn set_task_status(
    repo_root: &Path,
    id: &str,
    expected: Option<&str>,
    new_status: &str,
) -> Result<()> {
    let mut plan = read_todo(repo_root)?;
    let mut found = false;
    for t in plan.task.iter_mut() {
        if t.id == id {
            let cur = t.status.clone().unwrap_or_default();
            if let Some(exp) = expected {
                if cur != exp {
                    anyhow::bail!("plan: cannot change status for {}: expected '{}' but current status is '{}'", id, exp, cur);
                }
            }
            // handle finished -> move to archive
            if new_status == "finished" {
                let src = repo_root.join("plan/tasks").join(id);
                let dst = repo_root.join("plan/archive").join(id);
                if src.exists() {
                    fs::create_dir_all(dst.parent().unwrap())?;
                    fs::rename(&src, &dst).or_else(|_| {
                        // fallback copy
                        fs::create_dir_all(&dst)?;
                        for entry in walkdir::WalkDir::new(&src).into_iter().filter_map(|e| e.ok())
                        {
                            let p = entry.path();
                            if p.is_file() {
                                if let Ok(rel) = p.strip_prefix(&src) {
                                    let target = dst.join(rel);
                                    if let Some(parent) = target.parent() {
                                        fs::create_dir_all(parent)?;
                                    }
                                    fs::copy(p, &target)?;
                                }
                            }
                        }
                        fs::remove_dir_all(&src)
                    })?;
                    // update task_file path to archive
                    if let Some(_tf) = &t.task_file {
                        t.task_file = Some(format!("archive/{}/{}", id, "task.md"));
                    }
                }
            }
            // handle reopen: bring archive back to tasks
            if new_status == "pending_review" {
                // if currently finished, the task_file should start with archive/
                if let Some(tf) = &t.task_file {
                    if tf.starts_with("archive/") {
                        let src = repo_root.join("plan/archive").join(id);
                        let dst = repo_root.join("plan/tasks").join(id);
                        if src.exists() {
                            fs::create_dir_all(dst.parent().unwrap())?;
                            fs::rename(&src, &dst).or_else(|_| {
                                fs::create_dir_all(&dst)?;
                                for entry in
                                    walkdir::WalkDir::new(&src).into_iter().filter_map(|e| e.ok())
                                {
                                    let p = entry.path();
                                    if p.is_file() {
                                        if let Ok(rel) = p.strip_prefix(&src) {
                                            let target = dst.join(rel);
                                            if let Some(parent) = target.parent() {
                                                fs::create_dir_all(parent)?;
                                            }
                                            fs::copy(p, &target)?;
                                        }
                                    }
                                }
                                fs::remove_dir_all(&src)
                            })?;
                            // update task_file path to tasks/
                            if let Some(_tf) = &t.task_file {
                                t.task_file = Some(format!("tasks/{}/{}", id, "task.md"));
                            }
                        }
                    }
                }
            }
            t.status = Some(new_status.to_string());
            found = true;
            break;
        }
    }
    if !found {
        anyhow::bail!("plan: task '{}' not found", id);
    }
    write_todo(repo_root, &plan)?;
    Ok(())
}

fn default_check_review_accept(repo_root: &Path, id: &str) -> Result<()> {
    // ensure task.md contains some acceptance criteria
    let task_file = repo_root.join("plan/tasks").join(id).join("task.md");
    if !task_file.exists() {
        anyhow::bail!("review check failed: task file not found: {}", task_file.display());
    }
    let content = fs::read_to_string(&task_file).unwrap_or_default();
    let has_acceptance = content.to_lowercase().contains("acceptance")
        || content.to_lowercase().contains("acceptance criteria")
        || content.to_lowercase().contains("tests")
        || content.len() > 100;
    if !has_acceptance {
        anyhow::bail!("review check failed: task appears to be missing acceptance criteria or tests in task.md");
    }
    Ok(())
}

fn default_check_start(repo_root: &Path, id: &str) -> Result<()> {
    // allow start if the task already shows queued status, or if history contains acceptance
    let plan = read_todo(repo_root)?;
    for t in plan.task.iter() {
        if t.id == id {
            if let Some(st) = &t.status {
                if st == "queued" {
                    return Ok(());
                }
            }
            break;
        }
    }
    let hist_dir = repo_root.join("plan/tasks").join(id).join("history");
    if hist_dir.exists() {
        for e in fs::read_dir(&hist_dir)?.filter_map(|e| e.ok()) {
            let body = fs::read_to_string(e.path()).unwrap_or_default().to_lowercase();
            if body.contains("accept") || body.contains("queued") || body.contains("lgtm") {
                return Ok(());
            }
        }
    }
    anyhow::bail!("start check failed: no acceptance found in history and task is not queued");
}

fn default_check_test(repo_root: &Path, id: &str) -> Result<()> {
    // ensure there is some test plan or presence of tests/ directory
    let task_file = repo_root.join("plan/tasks").join(id).join("task.md");
    let content = fs::read_to_string(&task_file).unwrap_or_default();
    if content.to_lowercase().contains("test") || repo_has_tests(repo_root) {
        return Ok(());
    }
    anyhow::bail!("test check failed: no test plan or tests detected for task");
}

fn default_check_accept(repo_root: &Path, id: &str) -> Result<()> {
    // check for presence of reports under plan/tasks/<id>/reports or history mentioning 'tests passed' or report
    let reports_dir = repo_root.join("plan/tasks").join(id).join("reports");
    if reports_dir.exists() && reports_dir.read_dir()?.next().is_some() {
        return Ok(());
    }
    let hist_dir = repo_root.join("plan/tasks").join(id).join("history");
    if hist_dir.exists() {
        for e in fs::read_dir(&hist_dir)?.filter_map(|e| e.ok()) {
            let body = fs::read_to_string(e.path()).unwrap_or_default().to_lowercase();
            if body.contains("test") || body.contains("report") || body.contains("passed") {
                return Ok(());
            }
        }
    }
    anyhow::bail!("accept check failed: no test reports or evidence found for task");
}

fn default_check_finish(repo_root: &Path, id: &str) -> Result<()> {
    // ensure acceptance artifacts exist (reports or explicit acceptance note)
    let reports_dir = repo_root.join("plan/tasks").join(id).join("reports");
    if reports_dir.exists() && reports_dir.read_dir()?.next().is_some() {
        return Ok(());
    }
    let hist_dir = repo_root.join("plan/tasks").join(id).join("history");
    if hist_dir.exists() {
        for e in fs::read_dir(&hist_dir)?.filter_map(|e| e.ok()) {
            let body = fs::read_to_string(e.path()).unwrap_or_default().to_lowercase();
            if body.contains("accept")
                || body.contains("acceptance")
                || body.contains("acceptance report")
            {
                return Ok(());
            }
        }
    }
    anyhow::bail!("finish check failed: no acceptance report or evidence found for task");
}

fn repo_has_tests(repo_root: &Path) -> bool {
    let tests_dir = repo_root.join("tests");
    if tests_dir.exists() {
        return true;
    }
    false
}

fn auto_fix_repo(repo_root: &Path) -> Result<Vec<String>> {
    let mut fixes = Vec::new();

    // 1) Ensure CONTRIBUTING.md exists: copy from docs/contributing.md if present
    if !repo_root.join("CONTRIBUTING.md").exists() {
        let candidates = ["docs/contributing.md", "CONTRIBUTING.md", ".github/CONTRIBUTING.md"];
        for c in candidates {
            let p = repo_root.join(c);
            if p.exists() {
                fs::copy(&p, repo_root.join("CONTRIBUTING.md"))?;
                fixes.push(format!("Copied {} -> CONTRIBUTING.md", c));
                break;
            }
        }
    }

    // 2) Fix plan todo entries
    let mut plan = read_todo(repo_root)?;
    let mut changed = false;
    for t in plan.task.iter_mut() {
        // normalize status
        if let Some(st) = &t.status {
            if st == "open" {
                t.status = Some("pending_review".to_string());
                fixes.push(format!("Updated status for {}: open -> pending_review", t.id));
                changed = true;
            } else if st == "done" {
                t.status = Some("finished".to_string());
                fixes.push(format!("Updated status for {}: done -> finished", t.id));
                changed = true;
            }
        }

        // ensure task_file points to existing file
        if let Some(tf) = t.task_file.clone() {
            let p = repo_root.join("plan").join(&tf);
            if !p.exists() {
                // try to normalize patterns like tasks/0001-template.md -> tasks/0001/task.md
                if tf.contains("-template") {
                    // try to extract id from tf or use t.id
                    let id = t.id.clone();
                    let new_tf = format!("tasks/{}/task.md", id);
                    let newp = repo_root.join("plan").join(&new_tf);
                    if newp.exists() {
                        t.task_file = Some(new_tf.clone());
                        fixes.push(format!("Fixed task_file for {}: {} -> {}", t.id, tf, new_tf));
                        changed = true;
                        continue;
                    }
                    // try to copy from template if available
                    let templ = repo_root
                        .join("templates")
                        .join("default")
                        .join("plan")
                        .join("tasks")
                        .join(&id)
                        .join("task.md");
                    if templ.exists() {
                        fs::create_dir_all(newp.parent().unwrap())?;
                        fs::copy(&templ, &newp)?;
                        t.task_file = Some(new_tf.clone());
                        fixes.push(format!(
                            "Copied template task {} to {}",
                            templ.display(),
                            newp.display()
                        ));
                        changed = true;
                        continue;
                    }
                }

                // try tasks/<id>/task.md
                let new_tf = format!("tasks/{}/task.md", t.id);
                let newp = repo_root.join("plan").join(&new_tf);
                if newp.exists() {
                    t.task_file = Some(new_tf.clone());
                    fixes.push(format!("Fixed task_file for {}: {} -> {}", t.id, tf, new_tf));
                    changed = true;
                    continue;
                }

                // as last resort, create placeholder
                fs::create_dir_all(newp.parent().unwrap())?;
                fs::write(&newp, format!("title: \"Task {}\"\n\nAutomatically created.", t.id))?;
                t.task_file = Some(new_tf.clone());
                fixes.push(format!("Created placeholder task file for {}", t.id));
                changed = true;
            }
        } else {
            // missing task_file: create one under tasks/{id}/task.md
            let new_tf = format!("tasks/{}/task.md", t.id);
            let newp = repo_root.join("plan").join(&new_tf);
            if !newp.exists() {
                fs::create_dir_all(newp.parent().unwrap())?;
                fs::write(&newp, format!("title: \"Task {}\"\n\nAutomatically created.", t.id))?;
                fixes.push(format!("Created task file {}", new_tf));
            } else {
                fixes.push(format!("Found existing task file {}", new_tf));
            }
            t.task_file = Some(new_tf);
            changed = true;
        }

        // if finished, ensure task_file in archive
        if let Some(st) = &t.status {
            if st == "finished" {
                if let Some(tf) = t.task_file.clone() {
                    if !tf.starts_with("archive/") {
                        let src = repo_root.join("plan").join(&tf);
                        if src.exists() {
                            let dst_dir = repo_root.join("plan").join("archive").join(&t.id);
                            fs::create_dir_all(&dst_dir)?;
                            let dst = dst_dir.join("task.md");
                            fs::rename(&src, &dst).or_else(|_| {
                                fs::copy(&src, &dst)?;
                                fs::remove_file(&src)
                            })?;
                            t.task_file = Some(format!("archive/{}/task.md", t.id));
                            fixes.push(format!("Moved {} to archive/{}", tf, t.id));
                            changed = true;
                        }
                    }
                }
            }
        }
    }

    if changed {
        write_todo(repo_root, &plan)?;
    }

    Ok(fixes)
}

fn run_cmd_in_dir(cmd: &str, args: &[&str], dir: &Path) -> Result<(bool, String)> {
    let output =
        Command::new(cmd).args(args).current_dir(dir).output().context("running command")?;
    let success = output.status.success();
    let mut out = String::new();
    out.push_str(&String::from_utf8_lossy(&output.stdout));
    out.push_str(&String::from_utf8_lossy(&output.stderr));
    Ok((success, out))
}

fn run_verification(dest: &Path) -> Result<bool> {
    let mut all_ok = true;
    println!("Verification summary:");

    // 1) cargo fmt --all -- --check
    let (ok, out) = run_cmd_in_dir("cargo", &["fmt", "--all", "--", "--check"], dest)?;
    println!(" - cargo fmt: {}", if ok { "OK" } else { "FAILED" });
    if !ok {
        println!("   {}", &out);
        all_ok = false;
    }

    // 2) cargo clippy --all-targets --all-features -- -D warnings
    let (ok, out) = run_cmd_in_dir(
        "cargo",
        &["clippy", "--all-targets", "--all-features", "--", "-D", "warnings"],
        dest,
    )?;
    println!(" - cargo clippy: {}", if ok { "OK" } else { "FAILED" });
    if !ok {
        println!("   {}", &out);
        all_ok = false;
    }

    // 3) cargo test --all --quiet
    let (ok, out) = run_cmd_in_dir("cargo", &["test", "--all", "--quiet"], dest)?;
    println!(" - cargo test: {}", if ok { "OK" } else { "FAILED" });
    if !ok {
        println!("   {}", &out);
        all_ok = false;
    }

    Ok(all_ok)
}

fn parse_key_val(s: &str) -> Result<(String, String), String> {
    let mut parts = s.splitn(2, '=');
    match (parts.next(), parts.next()) {
        (Some(k), Some(v)) => Ok((k.to_string(), v.to_string())),
        _ => Err(format!("invalid key=value: '{}'", s)),
    }
}

fn emit_project_gha_outputs(repo_root: &Path) -> Result<()> {
    use std::io::Write;

    let path = repo_root.join("project.toml");
    if !path.exists() {
        anyhow::bail!("project.toml not found at {}", path.display());
    }

    let s = fs::read_to_string(&path).with_context(|| format!("reading {}", path.display()))?;
    let data: toml::Value = toml::from_str(&s).context("parsing project.toml")?;

    let project = data.get("project").and_then(|v| v.as_table());
    let ci = data.get("ci").and_then(|v| v.as_table());
    let artifact = data.get("artifact").and_then(|v| v.as_table());
    let docker = data.get("docker").and_then(|v| v.as_table());

    let project_type = project
        .and_then(|t| t.get("type"))
        .and_then(|v| v.as_str())
        .unwrap_or("library")
        .to_string();

    let run_build = ci.and_then(|t| t.get("run_build")).and_then(|v| v.as_bool()).unwrap_or(true);
    let run_tests = ci.and_then(|t| t.get("run_tests")).and_then(|v| v.as_bool()).unwrap_or(true);
    let run_security =
        ci.and_then(|t| t.get("run_security")).and_then(|v| v.as_bool()).unwrap_or(true);
    let run_docs = ci.and_then(|t| t.get("run_docs")).and_then(|v| v.as_bool()).unwrap_or(true);

    let quick_gate_precommit = ci
        .and_then(|t| t.get("quick_gate"))
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().any(|x| x.as_str() == Some("pre-commit")))
        .unwrap_or(false);

    let outputs = artifact
        .and_then(|t| t.get("outputs"))
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(|x| x.as_str().map(|s| s.to_string())).collect::<Vec<_>>())
        .unwrap_or_default();

    let outputs_list = outputs.join(",");
    let outputs_contains_docker = outputs.iter().any(|s| s == "docker");

    let docker_enabled =
        docker.and_then(|t| t.get("enabled")).and_then(|v| v.as_bool()).unwrap_or(false);
    let docker_image =
        docker.and_then(|t| t.get("image")).and_then(|v| v.as_str()).unwrap_or("").to_string();

    let project_name =
        project.and_then(|t| t.get("name")).and_then(|v| v.as_str()).unwrap_or("").to_string();
    let project_version =
        project.and_then(|t| t.get("version")).and_then(|v| v.as_str()).unwrap_or("").to_string();

    let b = |v: bool| if v { "true" } else { "false" };
    let outputs_kv: [(String, String); 12] = [
        ("project_type".to_string(), project_type),
        ("run_build".to_string(), b(run_build).to_string()),
        ("run_tests".to_string(), b(run_tests).to_string()),
        ("run_security".to_string(), b(run_security).to_string()),
        ("run_docs".to_string(), b(run_docs).to_string()),
        ("quick_gate_precommit".to_string(), b(quick_gate_precommit).to_string()),
        ("outputs_list".to_string(), outputs_list),
        ("outputs_contains_docker".to_string(), b(outputs_contains_docker).to_string()),
        ("docker_enabled".to_string(), b(docker_enabled).to_string()),
        ("docker_image".to_string(), docker_image),
        ("project_name".to_string(), project_name),
        ("project_version".to_string(), project_version),
    ];

    if let Ok(gh_out) = std::env::var("GITHUB_OUTPUT") {
        let mut f = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&gh_out)
            .with_context(|| format!("opening {}", gh_out))?;
        for (k, v) in outputs_kv {
            writeln!(f, "{}={}", k, v).context("writing GITHUB_OUTPUT")?;
        }
    } else {
        for (k, v) in outputs_kv {
            println!("{}={}", k, v);
        }
    }

    Ok(())
}

#[derive(Debug, Default)]
struct ProjectValidationReport {
    errors: Vec<String>,
    warnings: Vec<String>,
}

impl ProjectValidationReport {
    fn has_blocking_issues(&self, strict: bool) -> bool {
        !self.errors.is_empty() || (strict && !self.warnings.is_empty())
    }
}

fn is_template_placeholder(s: &str) -> bool {
    // Heuristic: treat handlebars-style placeholders as template values.
    // This repo is itself a template, so we must not fail drift checks on placeholders.
    s.contains("{{") && s.contains("}}")
}

fn is_concrete_value(s: &str) -> bool {
    let trimmed = s.trim();
    !trimmed.is_empty() && !is_template_placeholder(trimmed)
}

fn read_toml_value(path: &Path) -> Result<toml::Value> {
    let s = fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    toml::from_str(&s).with_context(|| format!("parsing {}", path.display()))
}

fn collect_project_validation_issues(
    repo_root: &Path,
    project_toml: &toml::Value,
    cargo_toml: &toml::Value,
) -> ProjectValidationReport {
    let mut report = ProjectValidationReport::default();

    let cosmos = project_toml.get("cosmos").and_then(|v| v.as_table());
    let schema_version = cosmos.and_then(|t| t.get("schema_version")).and_then(|v| v.as_integer());
    if schema_version != Some(1) {
        report.errors.push("project.toml: expected [cosmos].schema_version = 1".to_string());
    }

    let project = project_toml.get("project").and_then(|v| v.as_table());
    let build = project_toml.get("build").and_then(|v| v.as_table());
    let artifact = project_toml.get("artifact").and_then(|v| v.as_table());
    let docker = project_toml.get("docker").and_then(|v| v.as_table());

    let project_name = project.and_then(|t| t.get("name")).and_then(|v| v.as_str()).unwrap_or("");
    let project_version =
        project.and_then(|t| t.get("version")).and_then(|v| v.as_str()).unwrap_or("");

    let cargo_pkg = cargo_toml.get("package").and_then(|v| v.as_table());
    let cargo_name = cargo_pkg.and_then(|t| t.get("name")).and_then(|v| v.as_str()).unwrap_or("");
    let cargo_version =
        cargo_pkg.and_then(|t| t.get("version")).and_then(|v| v.as_str()).unwrap_or("");

    // Drift checks: enable only when the manifest looks like a concrete repo (not a template).
    // We key off project.name because template repos commonly use handlebars placeholders there.
    let drift_checks_enabled = is_concrete_value(project_name);

    if drift_checks_enabled && !cargo_name.is_empty() && project_name != cargo_name {
        report.errors.push(format!(
            "project.toml: [project].name '{}' does not match Cargo.toml [package].name '{}'",
            project_name, cargo_name
        ));
    }
    if drift_checks_enabled
        && is_concrete_value(project_version)
        && !cargo_version.is_empty()
        && project_version != cargo_version
    {
        report.errors.push(format!(
            "project.toml: [project].version '{}' does not match Cargo.toml [package].version '{}'",
            project_version, cargo_version
        ));
    }

    let outputs: Vec<String> = artifact
        .and_then(|t| t.get("outputs"))
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(|x| x.as_str().map(|s| s.to_string())).collect::<Vec<_>>())
        .unwrap_or_default();

    let allowed_outputs = ["docker", "binary", "crate", "wasm", "deb", "rpm"];
    for o in &outputs {
        if !allowed_outputs.contains(&o.as_str()) {
            report
                .errors
                .push(format!("project.toml: [artifact].outputs contains unknown value '{}'", o));
        }
    }

    let outputs_contains_docker = outputs.iter().any(|s| s == "docker");
    let outputs_contains_binary = outputs.iter().any(|s| s == "binary");

    let docker_enabled =
        docker.and_then(|t| t.get("enabled")).and_then(|v| v.as_bool()).unwrap_or(false);
    let docker_image = docker.and_then(|t| t.get("image")).and_then(|v| v.as_str()).unwrap_or("");

    if docker_enabled {
        if !outputs_contains_docker {
            report.errors.push(
                "project.toml: [docker].enabled=true requires [artifact].outputs to include 'docker'"
                    .to_string(),
            );
        }
        if docker_image.trim().is_empty() {
            report.errors.push(
                "project.toml: [docker].image must be non-empty when docker is enabled".to_string(),
            );
        } else if drift_checks_enabled && is_template_placeholder(docker_image) {
            report.warnings.push(
                "project.toml: [docker].image appears to be a template placeholder".to_string(),
            );
        }
    } else if outputs_contains_docker {
        report.warnings.push(
            "project.toml: [artifact].outputs contains 'docker' but [docker].enabled is false"
                .to_string(),
        );
    }

    if outputs_contains_binary {
        let build_bins: Vec<String> = build
            .and_then(|t| t.get("bins"))
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter().filter_map(|x| x.as_str().map(|s| s.to_string())).collect::<Vec<_>>()
            })
            .unwrap_or_default();

        let cargo_bins_defined = cargo_toml
            .get("bin")
            .and_then(|v| v.as_array())
            .map(|a| !a.is_empty())
            .unwrap_or(false);
        let default_main_exists = repo_root.join("src/main.rs").exists();

        if build_bins.is_empty() && !cargo_bins_defined && !default_main_exists {
            report.errors.push(
                "project.toml: [artifact].outputs includes 'binary' but no binaries were found (set [build].bins, define [[bin]] in Cargo.toml, or provide src/main.rs)".to_string(),
            );
        }
    }

    report
}

fn validate_project_manifest(repo_root: &Path, strict: bool) -> Result<ProjectValidationReport> {
    let project_path = repo_root.join("project.toml");
    let cargo_path = repo_root.join("Cargo.toml");

    if !project_path.exists() {
        anyhow::bail!("project.toml not found at {}", project_path.display());
    }
    if !cargo_path.exists() {
        anyhow::bail!("Cargo.toml not found at {}", cargo_path.display());
    }

    let project_toml = read_toml_value(&project_path)?;
    let cargo_toml = read_toml_value(&cargo_path)?;

    let report = collect_project_validation_issues(repo_root, &project_toml, &cargo_toml);
    if report.has_blocking_issues(strict) {
        // return report to caller; command decides exit code
    }
    Ok(report)
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    // Assume repo root is current dir
    let repo_root = std::env::current_dir().context("current dir")?;

    match cli.command {
        Commands::Generate {
            category,
            out_dir,
            apply,
            yes,
            allow_delete,
            force,
            template,
            template_dir: _template_dir,
            project_name,
            vars,
            verify,
        } => {
            let manifest = load_manifest(&repo_root, &template)?;
            let categories = if let Some(m) = manifest {
                m.categories
            } else {
                // fallback to reasonable defaults for this repo template
                let mut map = HashMap::new();
                map.insert(
                    "basis".to_string(),
                    vec![
                        "Cargo.toml".into(),
                        "README.md".into(),
                        "LICENSE".into(),
                        "CONTRIBUTING.md".into(),
                    ],
                );
                map.insert("docs".to_string(), vec!["docs/**".into()]);
                map.insert("ci".to_string(), vec![".github/**".into()]);
                map.insert("tests".to_string(), vec!["tests/**".into()]);
                map.insert("examples".to_string(), vec!["examples/**".into()]);
                map.insert("scripts".to_string(), vec!["scripts/**".into()]);
                map.insert("plan".to_string(), vec!["plan/**".into()]);
                map
            };

            let mut pats = Vec::new();
            if category == "all" {
                for v in categories.values() {
                    pats.extend(v.clone());
                }
            } else if let Some(v) = categories.get(&category) {
                pats.extend(v.clone());
            } else {
                eprintln!("Unknown category '{}'", category);
                std::process::exit(2);
            }

            // If there is a folder templates/<name>/, use it as a template source
            // Resolve template source: prefer repo templates/, then embedded templates, then exe-relative templates/
            let mut _maybe_tempdir: Option<TempDir> = None;
            let template_dir = repo_root.join("templates").join(&template);
            let template_source = if template_dir.exists() && template_dir.is_dir() {
                template_dir
            } else {
                static EMBEDDED_TEMPLATES: Dir = include_dir!("templates");
                if let Some(d) = EMBEDDED_TEMPLATES.get_dir(&template) {
                    // extract embedded template into tempdir
                    let td =
                        tempfile::tempdir().context("creating temp dir for embedded template")?;
                    let td_path = td.path().to_path_buf();
                    fn write_dir(
                        d: &Dir,
                        base: &std::path::Path,
                        template: &str,
                    ) -> anyhow::Result<()> {
                        for file in d.files() {
                            let rel = file.path();
                            // remove a leading template dir segment if present (e.g., 'default/...')
                            let rel = if rel
                                .components()
                                .next()
                                .map(|c| c.as_os_str() == std::ffi::OsStr::new(template))
                                .unwrap_or(false)
                            {
                                rel.strip_prefix(template).unwrap()
                            } else {
                                rel
                            };
                            let dest = base.join(rel);
                            if let Some(parent) = dest.parent() {
                                std::fs::create_dir_all(parent)?;
                            }
                            std::fs::write(&dest, file.contents())?;
                        }
                        for sd in d.dirs() {
                            write_dir(sd, base, template)?;
                        }
                        Ok(())
                    }
                    write_dir(d, &td_path, &template)
                        .context("extracting embedded template files")?;
                    _maybe_tempdir = Some(td);
                    td_path
                } else {
                    // try executable-relative templates/ (useful for installed binaries)
                    match std::env::current_exe() {
                        Ok(exe) => {
                            if let Some(parent) = exe.parent() {
                                let exe_t = parent.join("templates").join(&template);
                                if exe_t.exists() && exe_t.is_dir() {
                                    exe_t
                                } else {
                                    template_dir
                                }
                            } else {
                                template_dir
                            }
                        }
                        Err(_) => template_dir,
                    }
                }
            };
            if template_source.exists() && template_source.is_dir() {
                // collect files inside template_source
                let mut src_files = Vec::new();
                for entry in
                    walkdir::WalkDir::new(&template_source).into_iter().filter_map(|e| e.ok())
                {
                    let p = entry.path().to_path_buf();
                    if p.is_file() {
                        // compute relative path inside template
                        let rel = p.strip_prefix(&template_source).unwrap().to_path_buf();
                        src_files.push((p, rel));
                    }
                }
                // Normalize rels to drop an accidental leading template component (e.g., 'default/...')
                for item in src_files.iter_mut() {
                    let rel = &mut item.1;
                    let s = rel.to_string_lossy();
                    if s.starts_with(&format!("{}/", template)) {
                        *rel = PathBuf::from(&s[template.len() + 1..]);
                    }
                }
                if src_files.is_empty() {
                    println!("Template '{}' has no files", template);
                    return Ok(());
                }

                // Build template context
                let mut hb_ctx = serde_json::Map::new();
                if let Some(n) = project_name {
                    hb_ctx.insert("project-name".to_string(), serde_json::Value::String(n));
                }
                for (k, v) in vars {
                    hb_ctx.insert(k, serde_json::Value::String(v));
                }
                let hb = handlebars::Handlebars::new();

                println!("Template '{}' matched {} files:", template, src_files.len());
                for (_, rel) in &src_files {
                    let dr = rel.to_string_lossy();
                    let display_rel: &std::path::Path = if dr.starts_with(&format!("{}/", template))
                    {
                        // strip leading template dir if present
                        let s = &dr[template.len() + 1..];
                        std::path::Path::new(s)
                    } else {
                        rel.as_path()
                    };
                    println!(" - {}", display_rel.display());
                }

                if apply {
                    let dest = out_dir.clone();

                    // Build desired rel paths from template
                    let mut desired: Vec<PathBuf> = Vec::new();
                    for (_s, rel) in &src_files {
                        desired.push(rel.clone());
                    }

                    // If destination exists, compute diffs and prompt for fixes
                    if dest.exists() {
                        // gather current files under dest
                        let mut current: Vec<PathBuf> = Vec::new();
                        for entry in walkdir::WalkDir::new(&dest).into_iter().filter_map(|e| e.ok())
                        {
                            let p = entry.path();
                            if p.is_file() {
                                if let Ok(rel) = p.strip_prefix(&dest) {
                                    current.push(rel.to_path_buf());
                                }
                            }
                        }

                        use std::collections::HashSet;
                        let desired_set: HashSet<_> = desired.iter().cloned().collect();
                        let current_set: HashSet<_> = current.iter().cloned().collect();

                        let mut missing: Vec<PathBuf> =
                            desired_set.difference(&current_set).cloned().collect();
                        missing.sort();
                        let mut extra: Vec<PathBuf> =
                            current_set.difference(&desired_set).cloned().collect();
                        extra.sort();

                        // modified: intersection where contents differ
                        let mut modified: Vec<PathBuf> = Vec::new();
                        let mut skipped_overwrite: Vec<PathBuf> = Vec::new();
                        for common in desired_set.intersection(&current_set) {
                            // find source path for this rel
                            let src_path = src_files
                                .iter()
                                .find(|(_s, r)| r == common)
                                .map(|(s, _)| s.clone());
                            if let Some(s_path) = src_path {
                                let dest_path = dest.join(common);
                                if let (Ok(sb), Ok(db)) = (fs::read(&s_path), fs::read(&dest_path))
                                {
                                    if sb != db {
                                        if !force {
                                            skipped_overwrite.push(common.clone());
                                        } else {
                                            modified.push(common.clone());
                                        }
                                    }
                                }
                            }
                        }
                        modified.sort();
                        skipped_overwrite.sort();

                        if !missing.is_empty() || !extra.is_empty() || !modified.is_empty() {
                            println!("Detected inconsistencies between template and destination:");
                            if !missing.is_empty() {
                                println!("  Missing files to add ({}):", missing.len());
                                for m in &missing {
                                    println!("   + {}", m.display());
                                }
                            }
                            if !modified.is_empty() {
                                println!("  Files that will be overwritten ({}):", modified.len());
                                for m in &modified {
                                    println!("   ~ {}", m.display());
                                }
                            }
                            if !skipped_overwrite.is_empty() {
                                println!("  Skipped overwriting existing files (use --force to override): {}", skipped_overwrite.len());
                                for s in &skipped_overwrite {
                                    println!("    (skipped) {}", s.display());
                                }
                            }

                            // Helper: protect common VCS or third-party directories from deletion
                            fn is_protected_path(p: &Path) -> bool {
                                // Protect common VCS and lockfiles
                                if let Some(name) = p.file_name().and_then(|s| s.to_str()) {
                                    if name == "Cargo.lock" {
                                        return true;
                                    }
                                }
                                if let Some(s) = p.to_str() {
                                    let s = s.replace('\\', "/");
                                    return s == ".git"
                                        || s.starts_with(".git/")
                                        || s == "node_modules"
                                        || s.starts_with("node_modules/")
                                        || s == "target"
                                        || s.starts_with("target/")
                                        || s == ".venv"
                                        || s.starts_with(".venv/");
                                }
                                false
                            }

                            // split extras into visible ones (we may delete) and protected ones (skip)
                            let mut extra_protected: Vec<PathBuf> =
                                extra.iter().filter(|p| is_protected_path(p)).cloned().collect();
                            extra_protected.sort();
                            let mut extra_visible: Vec<PathBuf> =
                                extra.iter().filter(|p| !is_protected_path(p)).cloned().collect();
                            extra_visible.sort();

                            if !extra_visible.is_empty() {
                                println!("  Extra files to delete ({}):", extra_visible.len());
                                for e in &extra_visible {
                                    println!("   - {}", e.display());
                                }
                            }
                            if !extra_protected.is_empty() {
                                println!("  Skipped {} protected files/dirs (e.g. .git, node_modules, target):", extra_protected.len());
                                for p in &extra_protected {
                                    println!("    (protected) {}", p.display());
                                }
                            }

                            // Auto-apply if --yes set; otherwise prompt
                            if yes {
                                if !extra_visible.is_empty() && !allow_delete {
                                    eprintln!("Detected deletions but --allow-delete not set; aborting. Use --allow-delete with --yes to permit deletions (protected paths will still be skipped).");
                                    std::process::exit(2);
                                }
                                if !extra_visible.is_empty() {
                                    for e in &extra_visible {
                                        let p = dest.join(e);
                                        if p.exists() {
                                            fs::remove_file(&p).with_context(|| {
                                                format!("deleting extra file {:?}", p)
                                            })?;
                                        }
                                    }
                                }
                            } else {
                                use std::io::{stdin, stdout, Write};
                                print!("Apply these fixes? [y/N]: ");
                                stdout().flush().ok();
                                let mut input = String::new();
                                let _ = stdin().read_line(&mut input);
                                if !matches!(input.trim().to_lowercase().as_str(), "y" | "yes") {
                                    println!("Aborted by user; no changes applied.");
                                    std::process::exit(0);
                                }

                                if !extra_visible.is_empty() {
                                    print!("This operation will DELETE {} files. Type DELETE to confirm deletions: ", extra_visible.len());
                                    stdout().flush().ok();
                                    let mut dconfirm = String::new();
                                    let _ = stdin().read_line(&mut dconfirm);
                                    if dconfirm.trim() != "DELETE" {
                                        println!("Deletion confirmation failed; aborting without changes.");
                                        std::process::exit(0);
                                    }

                                    // perform deletions of visible/non-protected extras
                                    for e in &extra_visible {
                                        let p = dest.join(e);
                                        if p.exists() {
                                            fs::remove_file(&p).with_context(|| {
                                                format!("deleting extra file {:?}", p)
                                            })?;
                                        }
                                    }
                                } else {
                                    println!("No deletions to perform (all extra items are protected). Continuing.");
                                }
                            }
                            // continue to write/overwrite missing and modified files
                        }
                    }

                    // write files (add/overwrite)
                    for (src, rel) in &src_files {
                        // render destination path
                        let mut rel_str = rel.to_string_lossy().to_string();
                        if rel_str.starts_with(&format!("{}/", template)) {
                            rel_str = rel_str[template.len() + 1..].to_string();
                        }
                        let mut dest_rel = match hb.render_template(&rel_str, &hb_ctx) {
                            Ok(s) => PathBuf::from(s),
                            Err(_) => rel.clone(),
                        };
                        // ensure dest_rel does not unintentionally contain a leading template directory segment
                        if dest_rel
                            .components()
                            .next()
                            .map(|c| c.as_os_str() == std::ffi::OsStr::new(&template))
                            .unwrap_or(false)
                        {
                            let mut iter = dest_rel.iter();
                            iter.next(); // skip first
                            let new_rel: PathBuf = iter.collect();
                            dest_rel = new_rel;
                        }
                        let destpath = dest.join(dest_rel);
                        if let Some(parent) = destpath.parent() {
                            fs::create_dir_all(parent)?;
                        }

                        // If target exists and is protected (Cargo.toml/project.toml) and --force not set, skip overwrite
                        if destpath.exists() && !force {
                            println!(
                                "Skipping existing file (use --force to overwrite): {}",
                                destpath.display()
                            );
                            continue;
                        }

                        // Read source and render template variables if present
                        let sbytes = fs::read(src)
                            .with_context(|| format!("reading template file {:?}", src))?;
                        let stext = String::from_utf8_lossy(&sbytes).to_string();
                        let rendered = match hb.render_template(&stext, &hb_ctx) {
                            Ok(s) => s,
                            Err(_) => stext,
                        };
                        fs::write(&destpath, rendered)
                            .with_context(|| format!("writing file {:?}", destpath))?;
                    }

                    println!("Template files written to {}", dest.display());

                    // If files were accidentally written under a leading template subdir (e.g., ./default/*),
                    // move them up to the destination root so generated projects are placed at `dest`.
                    // Detect any files whose relative path starts with the template name and move them up.
                    let mut found_nested = false;
                    for entry in walkdir::WalkDir::new(&dest).into_iter().filter_map(|e| e.ok()) {
                        if entry.path().is_file() {
                            if let Ok(rel) = entry.path().strip_prefix(&dest) {
                                let s = rel.to_string_lossy();
                                if s.starts_with(&format!("{}/", template)) {
                                    found_nested = true;
                                    break;
                                }
                            }
                        }
                    }
                    if found_nested {
                        println!("Detected nested template entries, flattening into destination");
                        for entry in walkdir::WalkDir::new(&dest).into_iter().filter_map(|e| e.ok())
                        {
                            let p = entry.path().to_path_buf();
                            if p.is_file() {
                                if let Ok(rel) = p.strip_prefix(&dest) {
                                    let s = rel.to_string_lossy();
                                    if s.starts_with(&format!("{}/", template)) {
                                        let trimmed = &s[template.len() + 1..];
                                        let target = dest.join(trimmed);
                                        if let Some(parent) = target.parent() {
                                            fs::create_dir_all(parent)?;
                                        }
                                        fs::rename(&p, &target).or_else(|_| {
                                            // fallback to copy+remove
                                            fs::copy(&p, &target).and_then(|_| fs::remove_file(&p))
                                        })?;
                                    }
                                }
                            }
                        }
                        // try to remove the now-empty template dir
                        let _ = fs::remove_dir_all(dest.join(&template));
                    }

                    // Optional verification step: run fmt/clippy/test in the generated project
                    if verify {
                        println!(
                            "Running verification checks (fmt/clippy/test) in {}",
                            dest.display()
                        );
                        match run_verification(&dest) {
                            Ok(true) => println!("Verification checks passed"),
                            Ok(false) => {
                                eprintln!("Verification checks failed");
                                std::process::exit(4);
                            }
                            Err(e) => {
                                eprintln!("Verification error: {}", e);
                                std::process::exit(5);
                            }
                        }
                    }
                } else {
                    println!("Dry run (no files written). Use --apply to write files.");
                }

                return Ok(());
            }

            // fallback to pattern-based copy using manifest/categories
            let matches = expand_patterns(&repo_root, &pats)?;
            if matches.is_empty() {
                println!("No files matched for category '{}', patterns: {:?}", category, pats);
                return Ok(());
            }

            println!("Matched {} paths", matches.len());
            for p in &matches {
                println!(" - {}", p.strip_prefix(&repo_root).unwrap_or(p).display());
            }

            if apply {
                let dest = out_dir;

                // desired relative paths from matches
                let mut desired: Vec<PathBuf> = Vec::new();
                for p in &matches {
                    if let Ok(rel) = p.strip_prefix(&repo_root) {
                        desired.push(rel.to_path_buf());
                    }
                }

                if dest.exists() {
                    // gather current files under dest
                    let mut current: Vec<PathBuf> = Vec::new();
                    for entry in walkdir::WalkDir::new(&dest).into_iter().filter_map(|e| e.ok()) {
                        let p = entry.path();
                        if p.is_file() {
                            if let Ok(rel) = p.strip_prefix(&dest) {
                                current.push(rel.to_path_buf());
                            }
                        }
                    }

                    use std::collections::HashSet;
                    let desired_set: HashSet<_> = desired.iter().cloned().collect();
                    let current_set: HashSet<_> = current.iter().cloned().collect();

                    let mut missing: Vec<PathBuf> =
                        desired_set.difference(&current_set).cloned().collect();
                    missing.sort();
                    let mut extra: Vec<PathBuf> =
                        current_set.difference(&desired_set).cloned().collect();
                    extra.sort();

                    // modified detection
                    let mut modified: Vec<PathBuf> = Vec::new();
                    for common in desired_set.intersection(&current_set) {
                        let src_path = repo_root.join(common);
                        let dest_path = dest.join(common);
                        if let (Ok(sb), Ok(db)) = (fs::read(&src_path), fs::read(&dest_path)) {
                            if sb != db {
                                modified.push(common.clone());
                            }
                        }
                    }
                    modified.sort();

                    if !missing.is_empty() || !extra.is_empty() || !modified.is_empty() {
                        println!(
                            "Detected inconsistencies between template patterns and destination:"
                        );
                        if !missing.is_empty() {
                            println!("  Missing files to add ({}):", missing.len());
                            for m in &missing {
                                println!("   + {}", m.display());
                            }
                        }
                        if !modified.is_empty() {
                            println!("  Files that will be overwritten ({}):", modified.len());
                            for m in &modified {
                                println!("   ~ {}", m.display());
                            }
                        }
                        if !extra.is_empty() {
                            println!("  Extra files to delete ({}):", extra.len());
                            for e in &extra {
                                println!("   - {}", e.display());
                            }
                        }

                        // If --yes was provided, we'll auto-apply; otherwise prompt
                        if yes {
                            // auto-confirm mode
                            if !extra.is_empty() && !allow_delete {
                                eprintln!("Detected deletions but --allow-delete not set; aborting. Use --allow-delete with --yes to permit deletions.");
                                std::process::exit(2);
                            }
                            if !extra.is_empty() {
                                // perform deletions
                                for e in &extra {
                                    let p = dest.join(e);
                                    if p.exists() {
                                        fs::remove_file(&p).with_context(|| {
                                            format!("deleting extra file {:?}", p)
                                        })?;
                                    }
                                }
                            }
                        } else {
                            use std::io::{stdin, stdout, Write};
                            print!("Apply these fixes? [y/N]: ");
                            stdout().flush().ok();
                            let mut input = String::new();
                            let _ = stdin().read_line(&mut input);
                            if !matches!(input.trim().to_lowercase().as_str(), "y" | "yes") {
                                println!("Aborted by user; no changes applied.");
                                std::process::exit(0);
                            }
                            if !extra.is_empty() {
                                print!("This operation will DELETE {} files. Type DELETE to confirm deletions: ", extra.len());
                                stdout().flush().ok();
                                let mut dconfirm = String::new();
                                let _ = stdin().read_line(&mut dconfirm);
                                if dconfirm.trim() != "DELETE" {
                                    println!(
                                        "Deletion confirmation failed; aborting without changes."
                                    );
                                    std::process::exit(0);
                                }
                            }

                            // perform deletions
                            for e in &extra {
                                let p = dest.join(e);
                                if p.exists() {
                                    fs::remove_file(&p)
                                        .with_context(|| format!("deleting extra file {:?}", p))?;
                                }
                            }
                        }
                    }
                }

                // now copy matched paths
                copy_paths_to(&repo_root, &matches, &dest, force)?;
                println!("Files written to {}", dest.display());
            } else {
                println!("Dry run (no files written). Use --apply to write files.");
            }
        }

        Commands::Validate { level, fix } => {
            println!("Running {} validation...", level);
            let (errors, warnings) = validate_repo(&repo_root, &level, fix)?;
            let has_errors = !errors.is_empty();
            println!("\nValidation summary: {} errors, {} warnings", errors.len(), warnings.len());
            if !errors.is_empty() {
                println!("\nErrors:");
                for e in &errors {
                    println!(" - {}", e);
                }
            }
            if !warnings.is_empty() {
                println!("\nWarnings:");
                for w in &warnings {
                    println!(" - {}", w);
                }
            }

            // Treat errors as a failing validation in automation.
            // Warnings are informational and do not fail the command.
            if has_errors {
                std::process::exit(2);
            }
        }

        Commands::Ai { sub } => match sub {
            AiCmd::Doctor {} => {
                println!("AI/LLM configuration:");
                #[cfg(feature = "llm")]
                println!(" - Build feature: llm (enabled)");
                #[cfg(not(feature = "llm"))]
                println!(" - Build feature: llm (disabled; rebuild with --features llm)");
                let provider = std::env::var("LLM_PROVIDER").ok().unwrap_or_default();
                if provider.is_empty() {
                    println!(" - LLM_PROVIDER: (not set)");
                    println!("   Hint: set LLM_PROVIDER=stub for the built-in stub provider");
                } else {
                    println!(" - LLM_PROVIDER: {}", provider);
                }
                println!("\nPlan integration:");
                println!(" - Use 'cosmos plan --ai-validate <transition>' to run AI validation before user hooks");
                println!(" - Hooks can read PLAN_AI_VALIDATION_PATH (JSON report) if present");
            }
            AiCmd::Eval {} => {
                #[cfg(feature = "llm")]
                {
                    match rust_repo_template::llm::evaluate_with_llm(&repo_root) {
                        Ok(()) => println!("AI/LLM evaluation completed"),
                        Err(e) => {
                            eprintln!("AI/LLM evaluation error: {}", e);
                            std::process::exit(3);
                        }
                    }
                }
                #[cfg(not(feature = "llm"))]
                {
                    eprintln!("AI/LLM evaluation requested but not enabled in this build. Build with '--features llm' to enable.");
                    std::process::exit(3);
                }
            }
        },

        Commands::Plan { ai_validate, sub } => match sub {
            PlanCmd::List {} => {
                let todo = repo_root.join("plan/todo.toml");
                if !todo.exists() {
                    println!("plan/todo.toml not found");
                } else {
                    let s = fs::read_to_string(&todo).context("reading plan/todo.toml")?;
                    let plan: PlanTodo = toml::from_str(&s).context("parsing plan/todo.toml")?;
                    println!("Tasks ({}):", plan.task.len());
                    for t in plan.task {
                        println!(
                            " - {}: {} [{}]",
                            t.id,
                            t.title.unwrap_or_default(),
                            t.status.unwrap_or_default()
                        );
                    }
                }
            }
            PlanCmd::Validate { task: _, fix } => {
                println!("Running plan validation...");
                let issues = validate_plan(&repo_root)?;
                if issues.is_empty() {
                    println!("Plan validation OK");
                } else {
                    println!("Plan validation found {} issues:", issues.len());
                    for i in issues.iter() {
                        println!(" - {}", i);
                    }
                    if fix {
                        println!("Attempting to auto-fix plan issues...");
                        let fixes = auto_fix_repo(&repo_root)?;
                        for f in fixes.iter() {
                            println!(" - fixed: {}", f);
                        }
                        // re-run validation
                        let issues2 = validate_plan(&repo_root)?;
                        if issues2.is_empty() {
                            println!("Plan validation OK after fixes");
                        } else {
                            println!("Plan still has {} issues after fixes:", issues2.len());
                            for i in issues2 {
                                println!(" - {}", i);
                            }
                            std::process::exit(2);
                        }
                    } else {
                        std::process::exit(2);
                    }
                }
            }
            PlanCmd::Create { kind, title, content, assignee } => {
                let mut plan = read_todo(&repo_root)?;
                let next = read_next_id(&repo_root)?;
                let id = format!("{:04}", next);
                let task_dir = repo_root.join("plan/tasks").join(&id);
                fs::create_dir_all(&task_dir)?;
                let tf_path = task_dir.join("task.md");
                let body = content.unwrap_or_else(|| format!("# {}\n", title));
                fs::write(&tf_path, body)?;
                // create history dir
                let history_dir = task_dir.join("history");
                fs::create_dir_all(&history_dir)?;
                let task_file = format!("tasks/{}/task.md", id);
                let t = PlanTask {
                    id: id.clone(),
                    kind: Some(kind.clone()),
                    title: Some(title.clone()),
                    status: Some("pending_review".to_string()),
                    assignee,
                    task_file: Some(task_file),
                };
                plan.task.push(t);
                write_todo(&repo_root, &plan)?;
                write_next_id(&repo_root, next + 1)?;
                println!("Created task {}", id);
            }
            PlanCmd::Update { id, title, assignee, content } => {
                let mut plan = read_todo(&repo_root)?;
                let mut found = false;
                for t in plan.task.iter_mut() {
                    if t.id == id {
                        if let Some(tt) = title.clone() {
                            t.title = Some(tt);
                        }
                        if let Some(a) = assignee.clone() {
                            t.assignee = Some(a);
                        }
                        if let Some(c) = content.clone() {
                            // write into task file whether under tasks or archive
                            if let Some(tf) = &t.task_file {
                                let p = repo_root.join("plan").join(tf);
                                fs::write(&p, c)?;
                            }
                        }
                        found = true;
                        break;
                    }
                }
                if !found {
                    eprintln!("plan: task '{}' not found", id);
                    std::process::exit(2);
                }
                write_todo(&repo_root, &plan)?;
                println!("Updated task {}", id);
            }
            PlanCmd::Hooks { sub } => match sub {
                HooksCmd::Add { name } => {
                    let hooks_dir = repo_root.join("scripts/plan-hooks");
                    fs::create_dir_all(&hooks_dir)?;
                    let path = hooks_dir.join(format!("{}.py", name));
                    if path.exists() {
                        eprintln!("hook '{}' already exists at {}", name, path.display());
                        std::process::exit(2);
                    }
                    let template = r#"#!/usr/bin/env python3
"""Plan hook template.
Provide a unified entrypoint: `def run(ctx: dict) -> dict`.
When executed as a script it reads a JSON `ctx` from stdin and prints a JSON `res` to stdout.
Exiting with nonzero indicates failure and will block the transition.
"""
import json
import sys


def run(ctx: dict) -> dict:
    # implement checks here, e.g. ensure reports exist
    task_id = ctx.get('task_id')
    repo_root = ctx.get('repo_root')
    # return {'ok': True, 'message': 'ok'} or {'ok': False, 'message': 'explain'}
    return {'ok': True, 'message': 'ok'}


if __name__ == "__main__":
    try:
        data = json.load(sys.stdin)
    except Exception:
        data = {}
    res = run(data)
    print(json.dumps(res))
"#;
                    fs::write(&path, template)?;
                    // make executable
                    #[cfg(unix)]
                    {
                        use std::os::unix::fs::PermissionsExt;
                        let mut perm = fs::metadata(&path)?.permissions();
                        perm.set_mode(0o755);
                        fs::set_permissions(&path, perm)?;
                    }
                    println!("Created hook {} -> {}", name, path.display());
                }
                HooksCmd::List {} => {
                    let hooks_dir = repo_root.join("scripts/plan-hooks");
                    if !hooks_dir.exists() {
                        println!("No hooks directory found: {}", hooks_dir.display());
                    } else {
                        println!("Hooks:");
                        let mut names: Vec<String> = Vec::new();
                        for entry in fs::read_dir(&hooks_dir)?.filter_map(|e| e.ok()) {
                            let p = entry.path();
                            if p.is_file() && p.extension().and_then(|s| s.to_str()) == Some("py") {
                                if let Some(n) = p.file_stem().and_then(|s| s.to_str()) {
                                    names.push(n.to_string());
                                }
                            }
                        }
                        names.sort();
                        for n in names {
                            println!(" - {}", n);
                        }
                    }
                }
                HooksCmd::Check { name } => {
                    let hooks_dir = repo_root.join("scripts/plan-hooks");
                    let mut candidates: Vec<PathBuf> = Vec::new();
                    if let Some(nm) = name.clone() {
                        candidates.push(hooks_dir.join(format!("{}.py", nm)));
                    } else if hooks_dir.exists() {
                        for entry in fs::read_dir(&hooks_dir)?.filter_map(|e| e.ok()) {
                            let p = entry.path();
                            if p.is_file() && p.extension().and_then(|s| s.to_str()) == Some("py") {
                                candidates.push(p);
                            }
                        }
                    }
                    if candidates.is_empty() {
                        println!("No hooks to check");
                    }
                    for c in candidates {
                        if !c.exists() {
                            eprintln!("hook not found: {}", c.display());
                            std::process::exit(2);
                        }
                        // syntax check
                        let st =
                            Command::new("python3").arg("-m").arg("py_compile").arg(&c).output();
                        match st {
                            Ok(o) => {
                                if !o.status.success() {
                                    let mut msg = String::new();
                                    msg.push_str(&String::from_utf8_lossy(&o.stdout));
                                    msg.push_str(&String::from_utf8_lossy(&o.stderr));
                                    eprintln!("syntax error in {}: {}", c.display(), msg.trim());
                                    std::process::exit(2);
                                }
                            }
                            Err(e) => {
                                eprintln!("failed running python: {}", e);
                                std::process::exit(2);
                            }
                        }
                        // entrypoint check: module must define callable 'run'
                        let check_code = format!(
                            "import importlib.util,sys\nspec=importlib.util.spec_from_file_location('m','{}');m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m);print(hasattr(m,'run') and callable(m.run))",
                            c.display()
                        );
                        let out = Command::new("python3").arg("-c").arg(&check_code).output();
                        match out {
                            Ok(o) => {
                                let s = String::from_utf8_lossy(&o.stdout).trim().to_string();
                                if s != "True" {
                                    eprintln!(
                                        "entrypoint check failed for {}: missing callable 'run'",
                                        c.display()
                                    );
                                    std::process::exit(2);
                                }
                            }
                            Err(e) => {
                                eprintln!("failed running python: {}", e);
                                std::process::exit(2);
                            }
                        }
                        println!("OK: {}", c.display());
                    }
                }
            },
            PlanCmd::Review { id, decision, message, author } => {
                match decision.as_str() {
                    "accept" => {
                        // default pre-check: ensure task has acceptance criteria
                        if let Err(e) = default_check_review_accept(&repo_root, &id) {
                            eprintln!("{}", e);
                            eprintln!("Hint: Add 'Acceptance criteria' to task.md or provide a custom hook at scripts/plan-hooks/pre_review_accept.py");
                            std::process::exit(2);
                        }
                        let mut ai_path: Option<PathBuf> = None;
                        if ai_validate {
                            let (cur, _) = match get_task_context(&repo_root, &id) {
                                Ok(v) => v,
                                Err(e) => {
                                    eprintln!("{}", e);
                                    std::process::exit(2);
                                }
                            };
                            match ai_validate_transition(&repo_root, &id, &cur, "queued") {
                                Ok(report) => match write_ai_validation_report(
                                    &repo_root, &id, &cur, "queued", &report,
                                ) {
                                    Ok(p) => ai_path = Some(p),
                                    Err(e) => {
                                        eprintln!(
                                            "AI validation report write error (non-fatal): {}",
                                            e
                                        );
                                    }
                                },
                                Err(e) => {
                                    eprintln!("AI validation error (non-fatal): {}", e);
                                }
                            }
                        }
                        // run user pre-review hook if present
                        if let Err(e) = run_user_hook(
                            &repo_root,
                            "pre_review_accept",
                            &id,
                            Some("queued"),
                            ai_path.as_deref(),
                        ) {
                            eprintln!("{}", e);
                            std::process::exit(2);
                        }
                        if let Err(e) =
                            set_task_status(&repo_root, &id, Some("pending_review"), "queued")
                        {
                            eprintln!("{}", e);
                            eprintln!("Hint: Use 'cosmos plan review --id {} --decision accept' only when task is 'pending_review'", id);
                            std::process::exit(2);
                        }
                        append_history(&repo_root, &id, message.as_deref(), author.as_deref())?;
                        // run post-review hooks
                        let _ = run_user_hook(
                            &repo_root,
                            "post_review_accept",
                            &id,
                            Some("queued"),
                            ai_path.as_deref(),
                        );
                        println!("Task {} accepted and queued", id);
                    }
                    "reject" => {
                        append_history(&repo_root, &id, message.as_deref(), author.as_deref())?;
                        // run user hook for rejection
                        let _ = run_user_hook(&repo_root, "post_review_reject", &id, None, None);
                        println!("Task {} review recorded as rejected (still pending_review)", id);
                    }
                    other => {
                        eprintln!("plan: unknown decision '{}'. Use 'accept' or 'reject'", other);
                        std::process::exit(2);
                    }
                }
            }
            PlanCmd::Start { id, message, author } => {
                // default pre-check: ensure review accepted exists in history
                if let Err(e) = default_check_start(&repo_root, &id) {
                    eprintln!("{}", e);
                    eprintln!("Hint: Ensure the task was accepted during review (use 'cosmos plan review --id <id> --decision accept') or provide a custom hook at scripts/plan-hooks/pre_start.py");
                    std::process::exit(2);
                }
                let mut ai_path: Option<PathBuf> = None;
                if ai_validate {
                    let (cur, _) = match get_task_context(&repo_root, &id) {
                        Ok(v) => v,
                        Err(e) => {
                            eprintln!("{}", e);
                            std::process::exit(2);
                        }
                    };
                    match ai_validate_transition(&repo_root, &id, &cur, "working") {
                        Ok(report) => {
                            if let Ok(p) = write_ai_validation_report(
                                &repo_root, &id, &cur, "working", &report,
                            ) {
                                ai_path = Some(p);
                            }
                        }
                        Err(e) => eprintln!("AI validation error (non-fatal): {}", e),
                    }
                }
                if let Err(e) =
                    run_user_hook(&repo_root, "pre_start", &id, Some("working"), ai_path.as_deref())
                {
                    eprintln!("{}", e);
                    std::process::exit(2);
                }
                if let Err(e) = set_task_status(&repo_root, &id, Some("queued"), "working") {
                    eprintln!("{}", e);
                    eprintln!("Hint: task must be 'queued' to start. Consider 'cosmos plan review --id {} --decision accept' first.", id);
                    std::process::exit(2);
                }
                append_history(&repo_root, &id, message.as_deref(), author.as_deref())?;
                let _ = run_user_hook(
                    &repo_root,
                    "post_start",
                    &id,
                    Some("working"),
                    ai_path.as_deref(),
                );
                println!("Task {} started (working)", id);
            }
            PlanCmd::Test { id, message, author } => {
                if let Err(e) = default_check_test(&repo_root, &id) {
                    eprintln!("{}", e);
                    eprintln!("Hint: Ensure tests or test plan are present, or provide a custom hook at scripts/plan-hooks/pre_test.py");
                    std::process::exit(2);
                }
                let mut ai_path: Option<PathBuf> = None;
                if ai_validate {
                    let (cur, _) = match get_task_context(&repo_root, &id) {
                        Ok(v) => v,
                        Err(e) => {
                            eprintln!("{}", e);
                            std::process::exit(2);
                        }
                    };
                    match ai_validate_transition(&repo_root, &id, &cur, "testing") {
                        Ok(report) => {
                            if let Ok(p) = write_ai_validation_report(
                                &repo_root, &id, &cur, "testing", &report,
                            ) {
                                ai_path = Some(p);
                            }
                        }
                        Err(e) => eprintln!("AI validation error (non-fatal): {}", e),
                    }
                }
                if let Err(e) =
                    run_user_hook(&repo_root, "pre_test", &id, Some("testing"), ai_path.as_deref())
                {
                    eprintln!("{}", e);
                    std::process::exit(2);
                }
                if let Err(e) = set_task_status(&repo_root, &id, Some("working"), "testing") {
                    eprintln!("{}", e);
                    eprintln!(
                        "Hint: task must be 'working' to run tests (use 'cosmos plan start')."
                    );
                    std::process::exit(2);
                }
                append_history(&repo_root, &id, message.as_deref(), author.as_deref())?;
                let _ = run_user_hook(
                    &repo_root,
                    "post_test",
                    &id,
                    Some("testing"),
                    ai_path.as_deref(),
                );
                println!("Task {} moved to testing", id);
            }
            PlanCmd::Accept { id, message, author } => {
                if let Err(e) = default_check_accept(&repo_root, &id) {
                    eprintln!("{}", e);
                    eprintln!("Hint: Ensure tests and reports are present or provide a custom hook at scripts/plan-hooks/pre_accept.py");
                    std::process::exit(2);
                }
                let mut ai_path: Option<PathBuf> = None;
                if ai_validate {
                    let (cur, _) = match get_task_context(&repo_root, &id) {
                        Ok(v) => v,
                        Err(e) => {
                            eprintln!("{}", e);
                            std::process::exit(2);
                        }
                    };
                    match ai_validate_transition(&repo_root, &id, &cur, "under_acceptance") {
                        Ok(report) => {
                            if let Ok(p) = write_ai_validation_report(
                                &repo_root,
                                &id,
                                &cur,
                                "under_acceptance",
                                &report,
                            ) {
                                ai_path = Some(p);
                            }
                        }
                        Err(e) => eprintln!("AI validation error (non-fatal): {}", e),
                    }
                }
                if let Err(e) = run_user_hook(
                    &repo_root,
                    "pre_accept",
                    &id,
                    Some("under_acceptance"),
                    ai_path.as_deref(),
                ) {
                    eprintln!("{}", e);
                    std::process::exit(2);
                }
                if let Err(e) =
                    set_task_status(&repo_root, &id, Some("testing"), "under_acceptance")
                {
                    eprintln!("{}", e);
                    eprintln!("Hint: task must be 'testing' before acceptance.");
                    std::process::exit(2);
                }
                append_history(&repo_root, &id, message.as_deref(), author.as_deref())?;
                let _ = run_user_hook(
                    &repo_root,
                    "post_accept",
                    &id,
                    Some("under_acceptance"),
                    ai_path.as_deref(),
                );
                println!("Task {} marked under_acceptance", id);
            }
            PlanCmd::Finish { id, message, author } => {
                if let Err(e) = default_check_finish(&repo_root, &id) {
                    eprintln!("{}", e);
                    eprintln!("Hint: Ensure acceptance report is present or provide a custom hook at scripts/plan-hooks/pre_finish.py");
                    std::process::exit(2);
                }
                let mut ai_path: Option<PathBuf> = None;
                if ai_validate {
                    let (cur, _) = match get_task_context(&repo_root, &id) {
                        Ok(v) => v,
                        Err(e) => {
                            eprintln!("{}", e);
                            std::process::exit(2);
                        }
                    };
                    match ai_validate_transition(&repo_root, &id, &cur, "finished") {
                        Ok(report) => {
                            if let Ok(p) = write_ai_validation_report(
                                &repo_root, &id, &cur, "finished", &report,
                            ) {
                                ai_path = Some(p);
                            }
                        }
                        Err(e) => eprintln!("AI validation error (non-fatal): {}", e),
                    }
                }
                if let Err(e) = run_user_hook(
                    &repo_root,
                    "pre_finish",
                    &id,
                    Some("finished"),
                    ai_path.as_deref(),
                ) {
                    eprintln!("{}", e);
                    std::process::exit(2);
                }
                if let Err(e) =
                    set_task_status(&repo_root, &id, Some("under_acceptance"), "finished")
                {
                    eprintln!("{}", e);
                    eprintln!("Hint: task must be 'under_acceptance' before finishing.");
                    std::process::exit(2);
                }
                append_history(&repo_root, &id, message.as_deref(), author.as_deref())?;
                let _ = run_user_hook(
                    &repo_root,
                    "post_finish",
                    &id,
                    Some("finished"),
                    ai_path.as_deref(),
                );
                println!("Task {} finished and archived", id);
            }
            PlanCmd::Reopen { id, message, author } => {
                if ai_validate {
                    let (cur, _) = match get_task_context(&repo_root, &id) {
                        Ok(v) => v,
                        Err(e) => {
                            eprintln!("{}", e);
                            std::process::exit(2);
                        }
                    };
                    if let Ok(report) =
                        ai_validate_transition(&repo_root, &id, &cur, "pending_review")
                    {
                        let _ = write_ai_validation_report(
                            &repo_root,
                            &id,
                            &cur,
                            "pending_review",
                            &report,
                        );
                    }
                }
                if let Err(e) = set_task_status(&repo_root, &id, Some("finished"), "pending_review")
                {
                    eprintln!("{}", e);
                    eprintln!("Hint: only 'finished' tasks can be reopened.");
                    std::process::exit(2);
                }
                append_history(&repo_root, &id, message.as_deref(), author.as_deref())?;
                println!("Task {} reopened to pending_review", id);
            }
            PlanCmd::Delete { id, yes } => {
                let mut plan = read_todo(&repo_root)?;
                let idx = plan.task.iter().position(|t| t.id == id);
                if idx.is_none() {
                    eprintln!("plan: task '{}' not found", id);
                    std::process::exit(2);
                }
                let idx = idx.unwrap();
                if !yes {
                    use std::io::{stdin, stdout, Write};
                    print!("Delete task {} (y/N)? ", id);
                    stdout().flush().ok();
                    let mut input = String::new();
                    let _ = stdin().read_line(&mut input);
                    if !matches!(input.trim().to_lowercase().as_str(), "y" | "yes") {
                        println!("Aborted");
                        std::process::exit(0);
                    }
                }
                // remove task dir
                let dir = repo_root.join("plan/tasks").join(&id);
                if dir.exists() {
                    fs::remove_dir_all(&dir)
                        .with_context(|| format!("deleting task dir {:?}", dir))?;
                }
                // also remove archive dir if present
                let adir = repo_root.join("plan/archive").join(&id);
                if adir.exists() {
                    fs::remove_dir_all(&adir)
                        .with_context(|| format!("deleting archive dir {:?}", adir))?;
                }
                plan.task.remove(idx);
                write_todo(&repo_root, &plan)?;
                println!("Deleted task {}", id);
            }
            PlanCmd::Log { id, message, author } => {
                let plan = read_todo(&repo_root)?;
                let t = plan.task.iter().find(|x| x.id == id);
                if t.is_none() {
                    eprintln!("plan: task '{}' not found", id);
                    std::process::exit(2);
                }
                // prefer tasks dir, fallback to archive
                let mut history_dir = repo_root.join("plan/tasks").join(&id).join("history");
                if !history_dir.exists() {
                    history_dir = repo_root.join("plan/archive").join(&id).join("history");
                }
                fs::create_dir_all(&history_dir)?;
                let ts = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                let filename = format!("{}.md", ts);
                let path = history_dir.join(&filename);
                let mut contents = String::new();
                contents.push_str(&format!("time: {}\n", ts));
                if let Some(a) = author {
                    contents.push_str(&format!("author: {}\n", a));
                }
                contents.push_str("---\n");
                contents.push_str(&message);
                fs::write(&path, contents)?;
                println!("Logged event to {}", path.display());
            }
            PlanCmd::Show { id } => {
                let plan = read_todo(&repo_root)?;
                let t = plan.task.into_iter().find(|x| x.id == id);
                if t.is_none() {
                    eprintln!("plan: task '{}' not found", id);
                    std::process::exit(2);
                }
                let t = t.unwrap();
                println!("Task {}", t.id);
                println!(" Kind: {}", t.kind.clone().unwrap_or_default());
                println!(" Title: {}", t.title.unwrap_or_default());
                println!(" Status: {}", t.status.unwrap_or_default());
                println!(" Assignee: {}", t.assignee.unwrap_or_default());
                println!(" Task file: {}", t.task_file.clone().unwrap_or_default());
                // show main task content
                if let Some(tf) = &t.task_file {
                    let p = repo_root.join("plan").join(tf);
                    if p.exists() {
                        if let Ok(s) = fs::read_to_string(&p) {
                            println!("---");
                            println!("{}", s);
                        }
                    }
                }
                // show history entries
                let mut history_dir = repo_root.join("plan/tasks").join(&t.id).join("history");
                if !history_dir.exists() {
                    history_dir = repo_root.join("plan/archive").join(&t.id).join("history");
                }
                if history_dir.exists() {
                    println!("History:");
                    let mut entries: Vec<_> =
                        std::fs::read_dir(&history_dir)?.filter_map(|e| e.ok()).collect();
                    entries.sort_by_key(|e| e.path());
                    for e in entries {
                        if let Ok(s) = fs::read_to_string(e.path()) {
                            println!("---");
                            println!("{}", s);
                        }
                    }
                }
            }
        },

        Commands::Project { sub } => match sub {
            ProjectCmd::GhaOutputs {} => {
                emit_project_gha_outputs(&repo_root)?;
            }
            ProjectCmd::Validate { strict } => {
                let report = validate_project_manifest(&repo_root, strict)?;
                println!(
                    "Project validation summary: {} errors, {} warnings",
                    report.errors.len(),
                    report.warnings.len()
                );
                if !report.errors.is_empty() {
                    println!("\nErrors:");
                    for e in &report.errors {
                        println!(" - {}", e);
                    }
                }
                if !report.warnings.is_empty() {
                    println!("\nWarnings:");
                    for w in &report.warnings {
                        println!(" - {}", w);
                    }
                }

                if report.has_blocking_issues(strict) {
                    std::process::exit(2);
                }
            }
        },
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_auto_fix_repo_creates_contributing_and_fixes_plan() -> Result<()> {
        let td = tempdir()?;
        let root = td.path();

        // create docs/contributing.md
        fs::create_dir_all(root.join("docs"))?;
        fs::write(root.join("docs").join("contributing.md"), "contrib")?;

        // create plan dir and task file exists
        fs::create_dir_all(root.join("plan").join("tasks").join("0001"))?;
        fs::write(root.join("plan").join("tasks").join("0001").join("task.md"), "title: x")?;

        // write todo.toml with open status and wrong task_file
        let todo = r#"[meta]
owner = "maintainers"
created = "2025-12-18T12:00:00Z"

[[task]]
id = "0001"
title = "Template init"
status = "open"
assignee = "maintainer"
labels = ["template"]
task_file = "tasks/0001-template.md"
"#;
        fs::create_dir_all(root.join("plan"))?;
        fs::write(root.join("plan").join("todo.toml"), todo)?;

        let _fixes = auto_fix_repo(root)?;
        assert!(root.join("CONTRIBUTING.md").exists());
        let plan = read_todo(root)?;
        assert_eq!(plan.task[0].status.as_deref(), Some("pending_review"));
        assert_eq!(plan.task[0].task_file.as_deref(), Some("tasks/0001/task.md"));
        Ok(())
    }

    #[test]
    fn project_validate_collects_drift_errors_for_concrete_values() {
        let td = tempdir().unwrap();
        let root = td.path();

        let project_toml: toml::Value = toml::from_str(
            r#"[cosmos]
schema_version = 1

[project]
name = "myapp"
version = "1.0.0"

[artifact]
outputs = ["binary"]
"#,
        )
        .unwrap();
        let cargo_toml: toml::Value = toml::from_str(
            r#"[package]
name = "other"
version = "2.0.0"
edition = "2021"
"#,
        )
        .unwrap();

        let report = collect_project_validation_issues(root, &project_toml, &cargo_toml);
        assert!(!report.errors.is_empty());
        assert!(report.errors.iter().any(|e| e.contains("[project].name")));
        assert!(report.errors.iter().any(|e| e.contains("[project].version")));
    }

    #[test]
    fn project_validate_skips_drift_for_placeholders() {
        let td = tempdir().unwrap();
        let root = td.path();

        let project_toml: toml::Value = toml::from_str(
            r#"[cosmos]
schema_version = 1

[project]
name = "{{project-name}}"
version = "{{version}}"

[artifact]
outputs = ["binary"]
"#,
        )
        .unwrap();
        let cargo_toml: toml::Value = toml::from_str(
            r#"[package]
name = "rust-repo-template"
version = "0.2.0"
edition = "2021"
"#,
        )
        .unwrap();

        let report = collect_project_validation_issues(root, &project_toml, &cargo_toml);
        assert!(report
            .errors
            .iter()
            .all(|e| !e.contains("[project].name") && !e.contains("[project].version")));
    }
}
