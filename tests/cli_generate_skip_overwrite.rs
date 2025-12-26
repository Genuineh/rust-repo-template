use std::fs;
use tempfile::tempdir;

#[test]
fn generate_skips_cargo_and_project_toml_by_default() -> Result<(), Box<dyn std::error::Error>> {
    let td = tempdir()?;
    let out = td.path().to_path_buf();

    // create existing Cargo.toml and project.toml with sentinel content
    fs::write(out.join("Cargo.toml"), "orig-cargo")?;
    fs::write(out.join("project.toml"), "orig-project")?;

    // run generate non-force
    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("cosmos");
    cmd.arg("generate")
        .arg("--template")
        .arg("default")
        .arg("--category")
        .arg("all")
        .arg("--apply")
        .arg("--yes")
        .arg("--out-dir")
        .arg(out.as_path());

    cmd.assert().success();

    // ensure original contents remain
    let c = fs::read_to_string(out.join("Cargo.toml"))?;
    assert_eq!(c, "orig-cargo");
    let p = fs::read_to_string(out.join("project.toml"))?;
    assert_eq!(p, "orig-project");

    Ok(())
}
