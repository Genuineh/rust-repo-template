# Add cosmos project validate (drift + consistency checks)

## Goal
Provide a first-class `cosmos project validate` command that checks `project.toml` consistency with `Cargo.toml` and with enabled artifacts.

## Scope (v1)
- Parse `project.toml` (schema_version aware).
- Read `Cargo.toml` package metadata.
- Validate (warnings vs errors policy to be defined):
  - `project.name` matches Cargo package name (or document allowed mismatch)
  - `project.version` matches Cargo version
  - if `docker.enabled=true` then `artifact.outputs` contains `docker` and `docker.image` is non-empty
  - if `artifact.outputs` contains `binary`, ensure at least one bin is defined (from Cargo or `build.bins`)

## Acceptance criteria
- `cosmos validate` (repo validator) can optionally call `cosmos project validate`, or documentation points users to run it.
- Clear error messages pointing at the failing field.

## Tests
- Unit tests for validation logic.
- Integration tests with fixture `project.toml` + `Cargo.toml` combinations.