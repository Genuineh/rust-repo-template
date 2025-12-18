#!/usr/bin/env bash
set -euo pipefail

if ! command -v mkdocs >/dev/null 2>&1; then
  echo "mkdocs not installed. Install via 'pip install mkdocs mkdocs-material'"
  exit 1
fi

mkdocs build --clean
echo "Site built to site/"