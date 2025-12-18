use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::tempdir;
use std::fs;

#[test]
fn generate_from_template_folder() -> Result<(), Box<dyn std::error::Error>> {
    let td = tempdir()?;
    let out = td.path().join("out");

    // dry-run: should list files
    let mut cmd = Command::cargo_bin("cosmos")?;
    cmd.arg("generate").arg("--template").arg("default");
    cmd.assert().success().stdout(predicate::str::contains("Template 'default' matched"));

    // apply to out dir with project name
    let mut cmd = Command::cargo_bin("cosmos")?;
    cmd.arg("generate").arg("--template").arg("default").arg("--apply").arg("--out-dir").arg(out.to_str().unwrap()).arg("--project-name").arg("myproj");
    cmd.assert().success().stdout(predicate::str::contains("Template files written to"));

    // check some files copied and rendered
    assert!(out.join("Cargo.toml").exists());
    let cargo = std::fs::read_to_string(out.join("Cargo.toml"))?;
    assert!(cargo.contains("name = \"myproj\""));
    assert!(out.join("docs/getting-started.md").exists());
    assert!(out.join("scripts/validate_plan.py").exists());

    // file name templating
    assert!(out.join("NOTES/myproj.md").exists());

    // additional template baseline files
    assert!(out.join(".github/workflows/ci.yml").exists());
    assert!(out.join(".github/copilot-instructions.md").exists());
    assert!(out.join(".githooks/pre-commit").exists());
    assert!(out.join("plan/tasks/0001-template.md").exists());
    assert!(out.join("plan/archive/0000-init-template.md").exists());
    assert!(out.join("rustfmt.toml").exists());
    assert!(out.join("clippy.toml").exists());
    assert!(out.join(".gitignore").exists());
    assert!(out.join(".editorconfig").exists());
    assert!(out.join("CHANGELOG.md").exists());
    assert!(out.join("LICENSE").exists());
    assert!(out.join("project.toml").exists());

    Ok(())
}
