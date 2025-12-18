#!/usr/bin/env python3
"""Basic validation: ensure tasks referenced by todo.toml exist and done tasks are archived."""
import os
import sys
import re
repo_root = os.path.dirname(os.path.dirname(__file__))
plan_dir = os.path.join(repo_root, 'plan')
todo_file = os.path.join(plan_dir, 'todo.toml')

if not os.path.exists(todo_file):
    print('todo.toml missing')
    sys.exit(2)

with open(todo_file, 'r') as f:
    content = f.read()

tasks = re.findall(r"\[\[task\]\]([\s\S]*?)(?=\[\[task\]\]|$)", content)
errors = 0
for t in tasks:
    idm = re.search(r'id\s*=\s*"(\d{4})"', t)
    tf = re.search(r'task_file\s*=\s*"([^"]+)"', t)
    st = re.search(r'status\s*=\s*"(\w+)"', t)
    if not (idm and tf and st):
        print('Invalid task block:', t[:80])
        errors += 1
        continue
    tid = idm.group(1)
    task_file = os.path.join(plan_dir, tf.group(1))
    status = st.group(1)
    if not os.path.exists(task_file):
        print(f'Missing task file for {tid}: {task_file}')
        errors += 1
        continue
    if status == 'done' and not os.path.commonpath([task_file, os.path.join(plan_dir, 'archive')]).startswith(os.path.join(plan_dir, 'archive')):
        print(f'Task {tid} is marked done but file not in archive: {task_file}')
        errors += 1

if errors:
    print(f'Validation failed: {errors} issues')
    sys.exit(1)

print('Plan validation OK')
