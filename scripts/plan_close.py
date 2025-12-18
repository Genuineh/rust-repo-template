#!/usr/bin/env python3
"""Close a plan task: mark done in todo.toml and move task file to archive"""
import sys
import os
from datetime import datetime
import shutil

if len(sys.argv) < 2:
    print("Usage: plan_close.py 0001 [--resolution text]")
    sys.exit(1)

task_id = sys.argv[1]
resolution = None
if '--resolution' in sys.argv:
    idx = sys.argv.index('--resolution')
    if idx + 1 < len(sys.argv):
        resolution = sys.argv[idx+1]

repo_root = os.path.dirname(os.path.dirname(__file__))
plan_dir = os.path.join(repo_root, 'plan')
todo_file = os.path.join(plan_dir, 'todo.toml')
archive_dir = os.path.join(plan_dir, 'archive')

# naive update of todo.toml
with open(todo_file, 'r') as f:
    contents = f.read()

old = f'id = "{task_id}"'
if old not in contents:
    print(f"Task {task_id} not found in todo.toml")
    sys.exit(1)

# add done & status change
lines = contents.splitlines()
out = []
in_task = False
for line in lines:
    out.append(line)
    if line.strip() == f'id = "{task_id}"':
        in_task = True
    elif in_task and line.startswith('status'):
        # replace status line
        out[-1] = 'status = "done"'
    elif in_task and line.startswith('task_file'):
        # move the referenced file to archive
        path = line.split('=',1)[1].strip().strip('"')
        src = os.path.join(plan_dir, path)
        if os.path.exists(src):
            dst = os.path.join(archive_dir, os.path.basename(src))
            shutil.move(src, dst)
            out.append(f'done = "{datetime.utcnow().isoformat()}Z"')
            if resolution:
                out.append(f'resolution = "{resolution}"')
        in_task = False

with open(todo_file, 'w') as f:
    f.write('\n'.join(out))

print(f"Closed task {task_id}")
