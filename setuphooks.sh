#!/bin/sh
set -e

git config core.hooksPath .githooks
chmod +x .githooks/pre-commit
chmod +x .githooks/post-commit
echo "Git hooks configured."