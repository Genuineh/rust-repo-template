#!/usr/bin/env bash
set -euo pipefail

# Install git hooks by setting core.hooksPath to .githooks
ROOT=$(git rev-parse --show-toplevel 2>/dev/null || echo ".")
cd "$ROOT"

echo "Setting git core.hooksPath to .githooks in repository: $ROOT"

git config core.hooksPath .githooks

echo "Done. To undo: git config --unset core.hooksPath"

# Make sure hooks are executable
if [ -d .githooks ]; then
  chmod +x .githooks/* || true
fi

# Optional: install pre-commit and install hooks
if command -v python >/dev/null 2>&1 || command -v python3 >/dev/null 2>&1; then
  echo "Python found. Installing pre-commit and registering hooks..."
  if ! command -v pre-commit >/dev/null 2>&1; then
    if command -v pip >/dev/null 2>&1; then
      pip install --user pre-commit || pip3 install --user pre-commit || true
    else
      echo "pip not found; please install pre-commit manually (pip install pre-commit)"
    fi
  fi
  if command -v pre-commit >/dev/null 2>&1; then
    pre-commit install --install-hooks || true
    echo "pre-commit installed and hooks registered"
  fi
else
  echo "Python not found; skipping pre-commit installation. Install Python and run 'pip install pre-commit' to enable pre-commit hooks."
fi

echo "You can bypass hooks with: git commit --no-verify"