#!/bin/sh
set -e

git config core.hooksPath .githooks
chmod +x .githooks/pre-commit
echo "Git hooks configured."