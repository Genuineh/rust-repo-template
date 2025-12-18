# Getting Started

## Create a new repository from this template

1. On GitHub, click **Use this template** and create a new repository.
2. Update `Cargo.toml` fields: `name`, `version`, `description`, `repository` and `license`.
3. Update `README.md` and `CONTRIBUTING.md` as appropriate for your project.

## Local development

```bash
git clone <your-new-repo>
cd <your-new-repo>
cargo build
cargo test
```

## Customization checklist

- Replace placeholders in `LICENSE` and `CODEOWNERS`.
- Review `.github/workflows/ci.yml` to add or remove CI jobs.
- Add any custom hooks in `.github/custom/` (see CI Hooks docs).
- If you want published documentation, enable GitHub Pages and/or add a deployment workflow.
