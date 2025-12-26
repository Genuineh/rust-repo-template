use std::fs;
use tempfile::tempdir;

#[test]
fn generate_skips_git_dir_from_deletion() -> Result<(), Box<dyn std::error::Error>> {
    let td = tempdir()?;
    let out = td.path().to_path_buf();

    // create a .git folder with a HEAD file (should be protected from deletion)
    let git_dir = out.join(".git");
    fs::create_dir_all(git_dir.join("hooks"))?;
    fs::write(git_dir.join("HEAD"), "ref: refs/heads/main")?;
    fs::write(git_dir.join("config"), "[core]")?;

    // create an extra file that should be deletable
    let extra = out.join("EXTRA_FILE_TO_DELETE.txt");
    fs::write(&extra, "remove me")?;

    // run generate with --yes --allow-delete so it would delete extras if not protected
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
    // .git files should remain
    assert!(out.join(".git/HEAD").exists(), ".git/HEAD should be preserved");
    assert!(out.join(".git/config").exists(), ".git/config should be preserved");

    Ok(())
}
