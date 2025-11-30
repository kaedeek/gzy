# gzy

**gzy** is a simple CLI tool that makes common Git operations faster and easier.
You can run commands like `init` / `add` / `commit` / `branch` / `push` / `remote` / `clone` / `checkout` / `status` / `log`  with shorter syntax.

[![Crates.io](https://img.shields.io/crates/v/gzy.svg)](https://crates.io/crates/gzy)
[![Crates.io Downloads](https://img.shields.io/crates/d/gzy.svg)](https://crates.io/crates/gzy)
[![Docs.rs](https://docs.rs/gzy/badge.svg)](https://docs.rs/gzy)

---

## Usage

```bash
# install
cargo install gzy

# Init
gzy init

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

# Clone
gzy clone https://github.com/user/repo.git

# create new branch
gzy checkout develop

# Status
gzy status

# Log
gzy log