Custom CI hooks

This directory allows repository owners to insert custom CI logic into fixed pipeline stages.

Naming convention:
 - `.github/custom/fmt.sh`      -> runs during `fmt` stage after `cargo fmt --check`
 - `.github/custom/clippy.sh`   -> runs during `clippy` stage after `cargo clippy`
 - `.github/custom/build.sh`    -> runs during `build` stage after `cargo build`
 - `.github/custom/test.sh`     -> runs during `test` stage after `cargo test`
 - `.github/custom/security.sh` -> runs during `security` stage after `cargo audit`
 - `.github/custom/docs.sh`     -> runs during `docs` stage after `cargo doc`

Requirements for hook scripts:
 - Must be non-interactive and return a non-zero exit code on failure.
 - Should be executable, but CI will chmod the file automatically if present.
 - Keep scripts short and fast; long-running or interactive tasks are not recommended.

If a hook file does not exist, the CI will skip that hook silently.

Examples:
 - Add `.github/custom/test.sh` to run additional integration tests or test-matrix logic.
 - Add `.github/custom/security.sh` to run extra scanning tools.

Security note: custom scripts run in the CI context and will have access to repository contents and runner environment. Avoid storing secrets directly in the scripts; use repository secrets instead.