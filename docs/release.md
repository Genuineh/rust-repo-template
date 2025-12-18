# Release guide

This document describes the recommended process to cut a release for this repository.

1. Update `CHANGELOG.md`:
   - Move noteworthy changes from `Unreleased` to a new heading for the version being released (e.g. `## 0.2.0 - 2025-12-18`).
   - Add any final notes or upgrade instructions.

2. Bump version in `Cargo.toml` (root package) to the release version.

3. Commit the changes with clear message:

```bash
git add CHANGELOG.md Cargo.toml
git commit -m "chore(release): 0.2.0"
```

4. Tag and push the tag:

```bash
git tag -a v0.2.0 -m "Release v0.2.0"
git push origin main --tags
```

The repository has a GitHub Action (`.github/workflows/release.yml`) that will respond to tag pushes matching `v*`:
- It builds release artifacts (the `cosmos` binary) and attaches them to the GitHub Release.
- If the repo contains crates and the `CARGO_REGISTRY_TOKEN` secret is configured, the workflow will also attempt to `cargo publish` (configured in `release.yml`).

5. Install / Verify the binary (embedded templates):
   - Install locally for testing:

```bash
# Build and install the binary on your system
cargo install --path . --bin cosmos
```

   - Or install directly from the repository tag (once pushed):

```bash
# Install a specific release tag from GitHub (will build from source)
cargo install --git https://github.com/<org>/<repo> --tag v0.2.0 --bin cosmos
```

   - After installation, the binary includes embedded templates. Example usage:

```bash
# Create a new project from the embedded default template in current dir
cosmos generate --template default --apply --out-dir ./myproj --project-name myproj --verify
```

6. (Optional) Publish additional artifacts or cross-compile for other platforms and attach to the release via GitHub UI or workflow updates.

7. Verify the Release page on GitHub and optionally follow-up with docs / announcement.

Notes:
- If you want to make releases reproducible and sign artifacts, consider adding signed tags and additional CI steps to produce checksums and signature files.
- For publishing to crates.io, ensure `CARGO_REGISTRY_TOKEN` is stored in repository secrets and that the version is unique.
