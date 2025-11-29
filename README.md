# gzy

**gzy** is a simple CLI tool that makes common Git operations faster and easier.
You can run commands like `add` / `commit` / `branch` / `push` with shorter syntax.

[![Crates.io](https://img.shields.io/crates/v/gzy.svg)](https://crates.io/crates/gzy)
[![Crates.io Downloads](https://img.shields.io/crates/d/gzy.svg)](https://crates.io/crates/gzy)
[![Docs.rs](https://docs.rs/gzy/badge.svg)](https://docs.rs/gzy)

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
