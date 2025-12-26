# project.toml (Project Manifest)

This repository includes a `project.toml` at the repo root.

It is intended to be a small, **versioned manifest** that can drive:
- CI behavior (which jobs run)
- artifact/release outputs (e.g. docker vs binary)
- future `cosmos` project-aware commands

## Schema version

`project.toml` must include:

```toml
[cosmos]
schema_version = 1
```

If this section is missing, tooling should assume `schema_version = 0` (legacy) and apply conservative defaults.

## Supported fields (schema v1)

Today, only a small subset is treated as stable API.

### Project

```toml
[project]
name = "myproj"
version = "0.1.0"
# type = "service" | "library" | "application"
```

### CI switches

```toml
[ci]
run_build = true
run_tests = true
run_security = true
run_docs = true
quick_gate = ["pre-commit"]
```

### Artifacts

```toml
[artifact]
outputs = ["docker", "binary"]
```

### Docker

```toml
[docker]
enabled = true
image = "ghcr.io/<org>/myproj"
```

## Source-of-truth policy (recommended)

- `Cargo.toml` is the source of truth for Rust package metadata.
- `project.toml` is the source of truth for CI/artifact/release intent.
- When fields overlap (e.g. `project.name` / `project.version`), tooling should validate and warn/error on drift.

## Notes

- CI currently reads `project.toml` to decide which jobs to run.
- Future work will move parsing/validation into `cosmos` to avoid split-brain logic.
