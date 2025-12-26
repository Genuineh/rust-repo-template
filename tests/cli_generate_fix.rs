use std::fs;
use tempfile::tempdir;

#[test]
fn generate_detects_and_prompts_fix_with_deletion() -> Result<(), Box<dyn std::error::Error>> {
    let td = tempdir()?;
    let out = td.path().to_path_buf();
    // create an extra file that should be deleted
    let extra = out.join("EXTRA_FILE_TO_DELETE.txt");
    fs::write(&extra, "remove me")?;

    // run cosmos generate for plan category with apply and feed confirmation
    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("cosmos");
    cmd.arg("generate")
        .arg("--template")
        .arg("default")
        .arg("--category")
        .arg("plan")
        .arg("--apply")
        .arg("--out-dir")
        .arg(out.as_path());

    // provide two lines: 'y' to confirm fixes, then 'DELETE' to confirm deletions
    cmd.write_stdin("y\nDELETE\n");

    cmd.assert().success();

    // extra file should be deleted
    assert!(!extra.exists(), "extra file should have been deleted");

    // template todo.toml should be present
    assert!(out.join("plan/todo.toml").exists(), "todo.toml should be created in out dir");

    Ok(())
}
