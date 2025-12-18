#!/usr/bin/env bash
# 小工具：以统一格式生成 issue / prompt 并运行测试
set -e
if [ "$1" = "prompt" ]; then
  echo "Prompt 模板："
  sed -n '1,120p' .github/ai/prompt_templates.md
  exit 0
fi
if [ "$1" = "test" ]; then
  echo "Running tests..."
  cargo test
  exit 0
fi
cat <<'USAGE'
Usage: ./scripts/ai-assist.sh [prompt|test]
  prompt - show AI prompt templates
  test   - run cargo test
USAGE