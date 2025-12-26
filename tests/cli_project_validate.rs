use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;
use tempfile::tempdir;

fn write_file(path: &std::path::Path, contents: &str) {
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, contents).unwrap();
}

#[test]
fn project_validate_ok_for_concrete_manifest() {
    let td = tempdir().unwrap();
    let root = td.path();

    write_file(
        &root.join("Cargo.toml"),
        r#"[package]
name = "myapp"
version = "1.2.3"
edition = "2021"
"#,
    );

    write_file(&root.join("src/main.rs"), "fn main() {}\n");

    write_file(
        &root.join("project.toml"),
        r#"[cosmos]
schema_version = 1

[project]
name = "myapp"
version = "1.2.3"
type = "service"

[artifact]
outputs = ["binary"]

[docker]
enabled = false
"#,
    );

    let mut cmd = cargo_bin_cmd!("cosmos");
    cmd.current_dir(root)
        .args(["project", "validate"])
        .assert()
        .success()
        .stdout(predicate::str::contains("0 errors"));
}

#[test]
fn project_validate_blocks_on_version_drift() {
    let td = tempdir().unwrap();
    let root = td.path();

    write_file(
        &root.join("Cargo.toml"),
        r#"[package]
name = "myapp"
version = "1.2.3"
edition = "2021"
"#,
    );

    write_file(&root.join("src/main.rs"), "fn main() {}\n");

    write_file(
        &root.join("project.toml"),
        r#"[cosmos]
schema_version = 1

[project]
name = "myapp"
version = "9.9.9"
type = "service"

[artifact]
outputs = ["binary"]
"#,
    );

    let mut cmd = cargo_bin_cmd!("cosmos");
    cmd.current_dir(root)
        .args(["project", "validate"])
        .assert()
        .failure()
        .code(2)
        .stdout(predicate::str::contains("[project].version"));
}

#[test]
fn project_validate_blocks_on_docker_enabled_without_output() {
    let td = tempdir().unwrap();
    let root = td.path();

    write_file(
        &root.join("Cargo.toml"),
        r#"[package]
name = "myapp"
version = "1.2.3"
edition = "2021"
"#,
    );

    write_file(&root.join("src/main.rs"), "fn main() {}\n");

    write_file(
        &root.join("project.toml"),
        r#"[cosmos]
schema_version = 1

[project]
name = "myapp"
version = "1.2.3"
type = "service"

[artifact]
outputs = ["binary"]

[docker]
enabled = true
image = "ghcr.io/example/myapp"
"#,
    );

    let mut cmd = cargo_bin_cmd!("cosmos");
    cmd.current_dir(root)
        .args(["project", "validate"])
        .assert()
        .failure()
        .code(2)
        .stdout(predicate::str::contains("requires [artifact].outputs"));
}

#[test]
fn project_validate_skips_drift_for_template_placeholders() {
    let td = tempdir().unwrap();
    let root = td.path();

    write_file(
        &root.join("Cargo.toml"),
        r#"[package]
name = "rust-repo-template"
version = "0.2.0"
edition = "2021"
"#,
    );

    write_file(&root.join("src/main.rs"), "fn main() {}\n");

    write_file(
        &root.join("project.toml"),
        r#"[cosmos]
schema_version = 1

[project]
name = "{{project-name}}"
version = "{{version}}"
type = "service"

[artifact]
outputs = ["binary"]
"#,
    );

    let mut cmd = cargo_bin_cmd!("cosmos");
    cmd.current_dir(root).args(["project", "validate"]).assert().success();
}
