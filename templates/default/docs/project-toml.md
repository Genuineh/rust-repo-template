# project.toml (Project Manifest)

This template includes a `project.toml` at the repository root.

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

## Supported fields (schema v1)

Only a small subset is treated as stable API.

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
