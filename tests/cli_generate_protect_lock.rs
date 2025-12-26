use std::fs;
use tempfile::tempdir;

#[test]
fn generate_preserves_cargo_lock() -> Result<(), Box<dyn std::error::Error>> {
    let td = tempdir()?;
    let out = td.path().to_path_buf();

    // create a Cargo.lock file that should be protected from deletion
    fs::write(out.join("Cargo.lock"), "# lockfile")?;

    // create an extra file that should be deleted
    let extra = out.join("EXTRA_FILE_TO_DELETE2.txt");
    fs::write(&extra, "remove me")?;

    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("cosmos");
    cmd.arg("generate")
        .arg("--template")
        .arg("default")
        .arg("--category")
        .arg("all")
        .arg("--apply")
        .arg("--yes")
        .arg("--allow-delete")
        .arg("--out-dir")
        .arg(out.as_path());

    cmd.assert().success();

    // extra file should be deleted
    assert!(!extra.exists(), "extra file should have been deleted");
    // Cargo.lock should remain
    assert!(out.join("Cargo.lock").exists(), "Cargo.lock should be preserved");

    Ok(())
}
