#!/usr/bin/env bash
# Example custom build hook
set -euo pipefail
echo "Running custom build hook (example)"
# Example: build an additional binary
# cargo build --bin extra-tool --release || true
exit 0