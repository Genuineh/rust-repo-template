use predicates::prelude::*;
use tempfile::tempdir;

#[test]
fn generate_in_empty_dir_uses_embedded_template() -> Result<(), Box<dyn std::error::Error>> {
    let td = tempdir()?;
    let dir = td.path().to_path_buf();

    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("cosmos");
    cmd.current_dir(&dir);
    cmd.arg("generate")
        .arg("--project-name")
        .arg("cosmos-demo")
        .arg("--verify")
        .arg("--apply")
        .arg("--yes")
        .arg("-o")
        .arg(".");

    cmd.assert().success().stdout(predicate::str::contains("Template 'default' matched"));

    // check that Cargo.toml was created (either at root or under template subdir)
    let ok = dir.join("Cargo.toml").exists() || dir.join("default/Cargo.toml").exists();
    assert!(ok, "Cargo.toml should exist in generated output");
    Ok(())
}
