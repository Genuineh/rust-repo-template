use std::collections::HashMap;
use std::fs;

use tempfile::tempdir;

#[test]
fn project_gha_outputs_writes_expected_keys() -> Result<(), Box<dyn std::error::Error>> {
    let td = tempdir()?;
    let root = td.path();

    fs::write(
        root.join("project.toml"),
        r#"
[project]
name = "demo"
type = "service"
version = "1.2.3"

[ci]
run_build = false
run_tests = true
run_security = false
run_docs = true
quick_gate = ["pre-commit"]

[artifact]
outputs = ["docker", "binary"]

[docker]
enabled = true
image = "ghcr.io/acme/demo"
"#,
    )?;

    let out_path = root.join("gha_output.txt");
    fs::write(&out_path, "")?;

    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("cosmos");
    cmd.current_dir(root)
        .arg("project")
        .arg("gha-outputs")
        .env("GITHUB_OUTPUT", out_path.to_str().unwrap());

    cmd.assert().success();

    let contents = fs::read_to_string(&out_path)?;
    let mut map: HashMap<String, String> = HashMap::new();
    for line in contents.lines() {
        if let Some((k, v)) = line.split_once('=') {
            map.insert(k.to_string(), v.to_string());
        }
    }

    assert_eq!(map.get("project_type").map(String::as_str), Some("service"));
    assert_eq!(map.get("run_build").map(String::as_str), Some("false"));
    assert_eq!(map.get("run_tests").map(String::as_str), Some("true"));
    assert_eq!(map.get("run_security").map(String::as_str), Some("false"));
    assert_eq!(map.get("run_docs").map(String::as_str), Some("true"));
    assert_eq!(map.get("quick_gate_precommit").map(String::as_str), Some("true"));
    assert_eq!(map.get("outputs_list").map(String::as_str), Some("docker,binary"));
    assert_eq!(map.get("outputs_contains_docker").map(String::as_str), Some("true"));
    assert_eq!(map.get("docker_enabled").map(String::as_str), Some("true"));
    assert_eq!(map.get("docker_image").map(String::as_str), Some("ghcr.io/acme/demo"));
    assert_eq!(map.get("project_name").map(String::as_str), Some("demo"));
    assert_eq!(map.get("project_version").map(String::as_str), Some("1.2.3"));

    Ok(())
}
