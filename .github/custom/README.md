Custom CI hooks

This directory allows repository owners to insert custom CI logic into fixed pipeline stages.

Naming convention:
 - Before/after hooks use `before-<stage>.sh` and `after-<stage>.sh` so you can run logic either before or after a fixed step.
 - Examples:
   - `.github/custom/before-fmt.sh` -> runs before `cargo fmt --check`
   - `.github/custom/after-fmt.sh`  -> runs after `cargo fmt --check`
   - `.github/custom/before-clippy.sh` -> runs before `cargo clippy`
   - `.github/custom/after-clippy.sh`  -> runs after `cargo clippy`
   - `.github/custom/before-build.sh` -> runs before `cargo build`
   - `.github/custom/after-build.sh`  -> runs after `cargo build`
   - `.github/custom/before-test.sh`  -> runs before `cargo test`
   - `.github/custom/after-test.sh`   -> runs after `cargo test`
   - `.github/custom/before-security.sh` -> runs before `cargo audit`
   - `.github/custom/after-security.sh`  -> runs after `cargo audit`
   - `.github/custom/before-docs.sh` -> runs before `cargo doc`
   - `.github/custom/after-docs.sh`  -> runs after `cargo doc`
 - Backward compatibility: existing single-name hooks like `fmt.sh` / `clippy.sh` are still supported; the CI will run the new before/after hooks in preference to the single hook when both exist.

Requirements for hook scripts:
 - Must be non-interactive and return a non-zero exit code on failure.
 - Should be executable, but CI will chmod the file automatically if present.
 - Keep scripts short and fast; long-running or interactive tasks are not recommended.

If a hook file does not exist, the CI will skip that hook silently.

Examples:
 - Add `.github/custom/test.sh` to run additional integration tests or test-matrix logic.
 - Add `.github/custom/security.sh` to run extra scanning tools.

Security note: custom scripts run in the CI context and will have access to repository contents and runner environment. Avoid storing secrets directly in the scripts; use repository secrets instead.