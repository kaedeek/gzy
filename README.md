# gzy

**gzy** is a simple CLI tool that makes common Git operations faster and easier.  
It provides shorter and more convenient commands for everyday Git workflows.

Supported commands include:  
`init`, `add`, `commit`, `branch`, `push`, `remote`, `clone`, `checkout`, `status`, `log`

[![Crates.io](https://img.shields.io/crates/v/gzy.svg)](https://crates.io/crates/gzy)
[![Crates.io Downloads](https://img.shields.io/crates/d/gzy.svg)](https://crates.io/crates/gzy)
[![Docs.rs](https://docs.rs/gzy/badge.svg)](https://docs.rs/gzy)

---

## 🚀 Installation

```bash
cargo install gzy
```

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
gzy push
gzy push main
gzy push main -u
gzy push main --force

# Clone
gzy clone https://github.com/user/repo.git

# Switch to existing branch
gzy checkout main

# Create and switch to a new branch
gzy checkout dev -b


# Status
gzy status

# Log
gzy log