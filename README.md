# gzy

**gzy** is a simple CLI tool that makes common Git operations faster and easier.
You can run commands like `add` / `commit` / `branch` / `push` with shorter syntax.

---

## Installation

```bash
cargo install gzy

# Add file
gzy add <file>

# Commit
gzy commit "message"

# Create branch
gzy branch dev

# Add remote repository
gzy remote https://github.com/user/repo.git

# Push
gzy push main

# create new branch
gzy checkout develop
