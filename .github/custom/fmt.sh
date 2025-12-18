#!/usr/bin/env bash
# Example custom fmt hook
set -euo pipefail
echo "Running custom fmt hook (example)"
# example: check for generated code formatting
if ! git ls-files | grep -q "^generated/"; then
  echo "No generated directory"
else
  echo "Check formatting in generated/"
  cargo fmt --manifest-path=./Cargo.toml --all -- --check
fi
exit 0