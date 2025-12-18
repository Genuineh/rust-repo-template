use std::fs;
use std::path::PathBuf;

#[test]
fn templates_github_files_match_repo() {
    let repo_root = PathBuf::from(".");
    let repo_github = repo_root.join(".github");
    let tmpl_github = repo_root.join("templates/default/.github");

    // list of files expected to be present in both locations
    let required = vec![
        "copilot-instructions.md",
        "copilot-setup-steps.yml",
        "pull_request_template.md",
        "ISSUE_TEMPLATE/bug_report.md",
        "ISSUE_TEMPLATE/bug_report.yml",
        "ISSUE_TEMPLATE/feature_request.md",
        "ISSUE_TEMPLATE/feature_request.yml",
    ];

    for p in &required {
        let repo_f = repo_github.join(p);
        let tmpl_f = tmpl_github.join(p);
        assert!(repo_f.exists(), "repo missing .github/{}", p);
        assert!(tmpl_f.exists(), "template missing .github/{}", p);
        // compare content - be tolerant: ensure key snippets exist in both files
        let r = fs::read_to_string(repo_f).expect("read repo file");
        let t = fs::read_to_string(tmpl_f).expect("read template file");
        // file-specific expectations
        if p.ends_with("copilot-instructions.md") {
            let snippets = vec!["Build: `cargo build --workspace`", "Test: `cargo test --workspace`", "Format check: `cargo fmt -- --check`", "copilot-setup-steps.yml"];
            for sn in snippets {
                assert!(r.contains(sn), "repo file .github/{} missing snippet: {}", p, sn);
                assert!(t.contains(sn), "template file .github/{} missing snippet: {}", p, sn);
            }
        } else if p.ends_with("copilot-setup-steps.yml") {
            let snippets = vec!["install-rustup", "install-cargo-tools", "verify"];
            for sn in snippets {
                assert!(r.contains(sn), "repo file .github/{} missing snippet: {}", p, sn);
                assert!(t.contains(sn), "template file .github/{} missing snippet: {}", p, sn);
            }
        } else if p.ends_with("pull_request_template.md") {
            assert!(r.contains("AI usage disclosure") || r.contains("AI usage"), "repo pull request template missing AI disclosure guidance");
            assert!(t.contains("AI usage disclosure") || t.contains("AI usage"), "template pull request template missing AI disclosure guidance");
        } else if p.contains("ISSUE_TEMPLATE") {
            // ensure fields exist
            assert!(r.len() > 20, "repo issue template seems empty: {}", p);
            assert!(t.len() > 20, "template issue template seems empty: {}", p);
        } else {
            assert!(r.len() > 20, "repo file seems empty: {}", p);
            assert!(t.len() > 20, "template file seems empty: {}", p);
        }
    }

    // workflows: ensure template includes at least the same workflow filenames as repo
    let repo_wf = repo_root.join(".github/workflows");
    let tmpl_wf = tmpl_github.join("workflows");
    let repo_files: Vec<_> = fs::read_dir(&repo_wf).unwrap().filter_map(|e| e.ok()).map(|e| e.file_name()).collect();
    let tmpl_files: Vec<_> = fs::read_dir(&tmpl_wf).unwrap().filter_map(|e| e.ok()).map(|e| e.file_name()).collect();

    for rf in repo_files {
        // skip files that are intentionally not part of template (none expected currently)
        assert!(tmpl_files.contains(&rf), "workflow {} missing in template", rf.to_string_lossy());
    }
}
