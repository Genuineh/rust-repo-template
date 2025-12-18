#!/usr/bin/env bash
set -euo pipefail

echo "Example after-fmt hook: run extra formatting checks or commit formatting fixes (careful with CI commits)"
# Example: check for differences in generated files
# git diff --exit-code generated/ || true
exit 0