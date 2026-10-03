#!/bin/bash
set -eu

# # Codex
# curl -fsSL https://chatgpt.com/codex/install.sh | sh

# # OpenCode
# curl -fsSL https://opencode.ai/v2/install | bash

# Git
git config --global --add safe.directory /workspaces/dozens

# Run the application
# docker compose -f apps/compose.yml up -d