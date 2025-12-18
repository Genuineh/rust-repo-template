#!/usr/bin/env bash
# Example custom security hook
set -euo pipefail
echo "Running custom security hook (example)"
# Example: run additional scanners e.g. trivy, snyk (requires setup)
# trivy fs --exit-code 1 --severity HIGH,CRITICAL . || true
exit 0