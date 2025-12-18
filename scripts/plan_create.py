#!/usr/bin/env python3
"""Simple plan_create helper: generate next task id, create task file, append to todo.toml"""
import sys
import os
from datetime import datetime

repo_root = os.path.dirname(os.path.dirname(__file__))
plan_dir = os.path.join(repo_root, 'plan')
next_id_file = os.path.join(plan_dir, 'next_id.txt')
todo_file = os.path.join(plan_dir, 'todo.toml')
tasks_dir = os.path.join(plan_dir, 'tasks')

if len(sys.argv) < 2:
    print("Usage: plan_create.py \"Short title\" [--assignee alice] [--labels comma,separated]")
    sys.exit(1)

# parse args
title = sys.argv[1]
assignee = 'unassigned'
labels = []
for i,a in enumerate(sys.argv[2:]):
    if a == '--assignee' and i+3 <= len(sys.argv):
        assignee = sys.argv[i+3]
    if a == '--labels' and i+3 <= len(sys.argv):
        labels = sys.argv[i+3].split(',')

# read next id
with open(next_id_file, 'r') as f:
    next_id = f.read().strip()

task_id = next_id
filename = f"{task_id}-{title.lower().replace(' ','-')}.md"
file_path = os.path.join(tasks_dir, filename)
created = datetime.utcnow().isoformat() + 'Z'

# write task file
with open(file_path, 'w') as f:
    f.write('---\n')
    f.write(f'id: "{task_id}"\n')
    f.write(f'title: "{title}"\n')
    f.write(f'created: "{created}"\n')
    f.write(f'assignee: "{assignee}"\n')
    f.write('status: "open"\n')
    f.write('---\n\n')
    f.write('## 目标\n\n')

# append to todo.toml
entry = '\n[[task]]\n'
entry += f'id = "{task_id}"\n'
entry += f'title = "{title}"\n'
entry += f'status = "open"\n'
entry += f'assignee = "{assignee}"\n'
entry += f'created = "{created}"\n'
if labels:
    entry += f'labels = {labels}\n'
entry += f'task_file = "tasks/{filename}"\n'
with open(todo_file, 'a') as f:
    f.write(entry)

# bump next_id
next_num = int(task_id) + 1
with open(next_id_file, 'w') as f:
    f.write(f"{next_num:04d}\n")

print(f"Created task {task_id}: {file_path}")
