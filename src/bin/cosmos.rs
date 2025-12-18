use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use glob::glob;
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
        /// Template name (default: default)
        #[arg(long, default_value = "default")]
        template: String,
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
    },

    /// Plan-related commands
    Plan {
        #[command(subcommand)]
        sub: PlanCmd,
    },

    /// AI evaluation checks (rule-based or LLM)
    AiEval {
        /// mode: rule (default) or llm
        #[arg(short, long, default_value_t = String::from("rule"))]
        mode: String,
    },
}

#[derive(Subcommand)]
enum PlanCmd {
    /// List plan tasks
    List {},
    /// Validate plan structure and task files
    Validate { task: Option<String> },
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
        let s = fs::read_to_string(&path).with_context(|| format!("reading manifest {:?}", path))?;
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
        for entry in glob(&pattern).context("globbing pattern")? {
            if let Ok(p) = entry {
                // skip .git and target directories
                if p.components().any(|c| c.as_os_str() == ".git" || c.as_os_str() == "target") {
                    continue;
                }
                matches.push(p);
            }
        }
    }
    matches.sort();
    matches.dedup();
    Ok(matches)
}

fn copy_paths_to(repo_root: &Path, paths: &[PathBuf], dest: &Path) -> Result<()> {
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
            fs::copy(p, &destpath).with_context(|| format!("copy {:?} to {:?}", p, destpath))?;
        }
    }
    Ok(())
}

fn validate_repo(repo_root: &Path, _level: &str) -> Result<(Vec<String>, Vec<String>)> {
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
        for e in wf.read_dir().into_iter().flatten() {
            if let Ok(ent) = e {
                let p = ent.path();
                if p.extension().and_then(|s| s.to_str()).map(|s| s == "yml" || s == "yaml").unwrap_or(false) {
                    found = true;
                    break;
                }
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
    for p in plan_issues {
        errors.push(p);
    }

    // AI heuristics
    let ai_w = check_ai_heuristics(repo_root)?;
    for w in ai_w {
        warnings.push(w);
    }

    Ok((errors, warnings))
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct PlanTodo {
    meta: Option<toml::Value>,
    #[serde(default)]
    task: Vec<PlanTask>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct PlanTask {
    id: String,
    title: Option<String>,
    status: Option<String>,
    assignee: Option<String>,
    task_file: Option<String>,
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

    for t in &plan.task {
        if t.id.trim().is_empty() {
            issues.push(format!("plan: task with empty id: {:?}", t.title));
            continue;
        }
        if let Some(tf) = &t.task_file {
            let p = plan_dir.join(tf);
            if !p.exists() {
                issues.push(format!("plan: referenced task_file '{}' not found", tf));
            } else {
                // if task is done, ensure it is in archive/
                if let Some(st) = &t.status {
                    if st == "done" && !tf.starts_with("archive/") {
                        issues.push(format!("plan: task {} marked done but task_file '{}' is not in archive/", t.id, tf));
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
    if !repo_root.join(".github/copilot-instructions.md").exists() && !repo_root.join(".github/ai").exists() {
        warnings.push("No AI guidelines or .github/copilot-instructions.md found".to_string());
    }
    let readme = fs::read_to_string(repo_root.join("README.md")).unwrap_or_default();
    if !readme.to_lowercase().contains("ai") {
        warnings.push("README doesn't mention AI collaboration guidance".to_string());
    }
    Ok(warnings)
}

fn run_cmd_in_dir(cmd: &str, args: &[&str], dir: &Path) -> Result<(bool, String)> {
    let output = Command::new(cmd).args(args).current_dir(dir).output().context("running command")?;
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
    if !ok { println!("   {}",&out); all_ok = false; }

    // 2) cargo clippy --all-targets --all-features -- -D warnings
    let (ok, out) = run_cmd_in_dir("cargo", &["clippy", "--all-targets", "--all-features", "--", "-D", "warnings"], dest)?;
    println!(" - cargo clippy: {}", if ok { "OK" } else { "FAILED" });
    if !ok { println!("   {}",&out); all_ok = false; }

    // 3) cargo test --all --quiet
    let (ok, out) = run_cmd_in_dir("cargo", &["test", "--all", "--quiet"], dest)?;
    println!(" - cargo test: {}", if ok { "OK" } else { "FAILED" });
    if !ok { println!("   {}",&out); all_ok = false; }

    Ok(all_ok)
}

fn parse_key_val(s: &str) -> Result<(String, String), String> {
    let mut parts = s.splitn(2, '=');
    match (parts.next(), parts.next()) {
        (Some(k), Some(v)) => Ok((k.to_string(), v.to_string())),
        _ => Err(format!("invalid key=value: '{}'", s)),
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    // Assume repo root is current dir
    let repo_root = std::env::current_dir().context("current dir")?;

    match cli.command {
        Commands::Generate { category, out_dir, apply, template, project_name, vars, verify } => {
            let manifest = load_manifest(&repo_root, &template)?;
            let categories = if let Some(m) = manifest {
                m.categories
            } else {
                // fallback to reasonable defaults for this repo template
                let mut map = HashMap::new();
                map.insert(
                    "basis".to_string(),
                    vec!["Cargo.toml".into(), "README.md".into(), "LICENSE".into(), "CONTRIBUTING.md".into()],
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
            let template_dir = repo_root.join("templates").join(&template);
            if template_dir.exists() && template_dir.is_dir() {
                // collect files inside template_dir
                let mut src_files = Vec::new();
                for entry in walkdir::WalkDir::new(&template_dir).into_iter().filter_map(|e| e.ok()) {
                    let p = entry.path().to_path_buf();
                    if p.is_file() {
                        // compute relative path inside template
                        let rel = p.strip_prefix(&template_dir).unwrap().to_path_buf();
                        src_files.push((p, rel));
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
                    println!(" - {}", rel.display());
                }

                if apply {
                    let dest = out_dir.clone();
                    for (src, rel) in &src_files {
                        // render destination path
                        let rel_str = rel.to_string_lossy().to_string();
                        let dest_rel = match hb.render_template(&rel_str, &hb_ctx) {
                            Ok(s) => PathBuf::from(s),
                            Err(_) => rel.clone(),
                        };
                        let destpath = dest.join(dest_rel);
                        if let Some(parent) = destpath.parent() {
                            fs::create_dir_all(parent)?;
                        }

                        // Try to render file contents as UTF-8 text templates; fallback to binary copy
                        // Skip rendering for GitHub workflows and files that contain GitHub expressions or templating markers
                        let src_s = src.to_string_lossy().to_string();
                        let raw = fs::read_to_string(&src);
                        let should_skip = src_s.contains(".github/workflows/") || raw.as_ref().map(|s| s.contains("${{") ).unwrap_or(false);

                        match raw {
                            Ok(content) => {
                                if should_skip {
                                    // write raw content to destination to avoid handlebars parsing errors
                                    fs::write(&destpath, content)?;
                                } else {
                                    match hb.render_template(&content, &hb_ctx) {
                                        Ok(rendered) => fs::write(&destpath, rendered)?,
                                        Err(e) => {
                                            // if rendering fails, copy raw and warn
                                            eprintln!("warning: template render failed for {:?}: {}. copying raw.", src, e);
                                            fs::copy(src, &destpath)?;
                                        }
                                    }
                                }
                            }
                            Err(_) => {
                                fs::copy(src, &destpath)?;
                            }
                        }
                    }

                    println!("Template files written to {}", dest.display());

                    // Optional verification step: run fmt/clippy/test in the generated project
                    if verify {
                        println!("Running verification checks (fmt/clippy/test) in {}", dest.display());
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
                copy_paths_to(&repo_root, &matches, &dest)?;
                println!("Files written to {}", dest.display());
            } else {
                println!("Dry run (no files written). Use --apply to write files.");
            }
        }

        Commands::Validate { level } => {
            println!("Running {} validation...", level);
            let (errors, warnings) = validate_repo(&repo_root, &level)?;
            println!("\nValidation summary: {} errors, {} warnings", errors.len(), warnings.len());
            if !errors.is_empty() {
                println!("\nErrors:");
                for e in errors {
                    println!(" - {}", e);
                }
            }
            if !warnings.is_empty() {
                println!("\nWarnings:");
                for w in warnings {
                    println!(" - {}", w);
                }
            }
        }

        Commands::Plan { sub } => match sub {
            PlanCmd::List {} => {
                let todo = repo_root.join("plan/todo.toml");
                if !todo.exists() {
                    println!("plan/todo.toml not found");
                } else {
                    let s = fs::read_to_string(&todo).context("reading plan/todo.toml")?;
                    let plan: PlanTodo = toml::from_str(&s).context("parsing plan/todo.toml")?;
                    println!("Tasks ({}):", plan.task.len());
                    for t in plan.task {
                        println!(" - {}: {} [{}]", t.id, t.title.unwrap_or_default(), t.status.unwrap_or_default());
                    }
                }
            }
            PlanCmd::Validate { task: _ } => {
                println!("Running plan validation...");
                let issues = validate_plan(&repo_root)?;
                if issues.is_empty() {
                    println!("Plan validation OK");
                } else {
                    println!("Plan validation found {} issues:", issues.len());
                    for i in issues {
                        println!(" - {}", i);
                    }
                    std::process::exit(2);
                }
            }
        },

        Commands::AiEval { mode } => {
            match mode.as_str() {
                "rule" => {
                    println!("Running AI rule-based heuristics...");
                    let ws = check_ai_heuristics(&repo_root)?;
                    if ws.is_empty() {
                        println!("AI heuristics OK");
                    } else {
                        println!("AI heuristics found {} warnings:", ws.len());
                        for w in ws {
                            println!(" - {}", w);
                        }
                        std::process::exit(2);
                    }
                }
                "llm" => {
                    #[cfg(feature = "llm")]
                    {
                        match crate::llm::evaluate_with_llm(&repo_root) {
                            Ok(()) => println!("LLM evaluation completed (provider stub)"),
                            Err(e) => {
                                eprintln!("LLM evaluation error: {}", e);
                                std::process::exit(3);
                            }
                        }
                    }
                    #[cfg(not(feature = "llm"))]
                    {
                        eprintln!("LLM evaluation requested but not enabled in this build. Build with '--features llm' to enable.");
                        std::process::exit(3);
                    }
                }
                other => {
                    eprintln!("Unknown ai-eval mode: {}", other);
                    std::process::exit(2);
                }
            }
        },
    }

    Ok(())
}
