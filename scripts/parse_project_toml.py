#!/usr/bin/env python3
"""Parse project.toml and write outputs to $GITHUB_OUTPUT for GitHub Actions.
If GITHUB_OUTPUT isn't present (local run), print results to stdout for debugging.
"""
import os
import sys

try:
    # Python 3.11+: tomllib
    import tomllib as toml
except Exception:
    try:
        import toml
    except Exception:
        print("Please install 'toml' (pip install toml) or use Python 3.11+", file=sys.stderr)
        sys.exit(2)

ROOT = os.path.dirname(os.path.dirname(__file__))
PATH = os.path.join(ROOT, "project.toml")

def get(d, path, default=None):
    cur = d
    for p in path.split('.'):
        if isinstance(cur, dict) and p in cur:
            cur = cur[p]
        else:
            return default
    return cur

if not os.path.exists(PATH):
    print(f"project.toml not found at {PATH}")
    sys.exit(1)

with open(PATH, 'rb') as f:
    data = toml.loads(f.read().decode('utf-8'))

ci = get(data, 'ci', {}) or {}
artifact = get(data, 'artifact', {}) or {}
project = get(data, 'project', {}) or {}
docker = get(data, 'docker', {}) or {}

outputs = {
    'project_type': project.get('type', 'library'),
    'run_build': str(bool(ci.get('run_build', True))).lower(),
    'run_tests': str(bool(ci.get('run_tests', True))).lower(),
    'run_security': str(bool(ci.get('run_security', True))).lower(),
    'run_docs': str(bool(ci.get('run_docs', True))).lower(),
    'quick_gate_precommit': str('pre-commit' in ci.get('quick_gate', [])).lower(),
    'outputs_list': ",".join(artifact.get('outputs', [])),
    'outputs_contains_docker': str('docker' in artifact.get('outputs', [])).lower(),
    'docker_enabled': str(bool(docker.get('enabled', False))).lower(),
    'docker_image': docker.get('image', ''),
    'project_name': project.get('name', ''),
    'project_version': project.get('version', ''),
}

# Write to GITHUB_OUTPUT if available
gh_out = os.environ.get('GITHUB_OUTPUT')
if gh_out:
    with open(gh_out, 'a') as f:
        for k, v in outputs.items():
            f.write(f"{k}={v}\n")
else:
    # Print for debugging locally
    for k, v in outputs.items():
        print(f"{k}={v}")

# exit 0
