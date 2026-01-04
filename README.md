# gzy

**gzy** is a simple CLI tool designed to make everyday Git operations faster and easier.  
It allows you to run common Git commands using shorter and more intuitive syntax.

## Supported Commands
`init`, `add`, `commit`, `branch`, `push`, `remote`, `clone`,  
`checkout`, `status`, `log`, `conflict`, `sync`

[![Crates.io](https://img.shields.io/crates/v/gzy.svg)](https://crates.io/crates/gzy)
[![Crates.io Downloads](https://img.shields.io/crates/d/gzy.svg)](https://crates.io/crates/gzy)
[![Docs.rs](https://docs.rs/gzy/badge.svg)](https://docs.rs/gzy)

---

## Usage

```bash
# Initialize a repository
gzy init

# Add a file
gzy add <file>

# Commit changes
gzy commit "message"

# Create a branch
gzy branch dev

# Add a remote repository
gzy remote https://github.com/user/repo.git

# Push changes
gzy push main
gzy push main
gzy push main -u
gzy push main --force

# Clone a repository
gzy clone https://github.com/user/repo.git

# Create and switch to a new branch
gzy checkout develop

# Resolve conflicts by keeping your changes
gzy conflict ours

# Resolve conflicts by keeping their changes
gzy conflict theirs

# Abort the merge
gzy conflict abort

# Sync the current branch with upstream/develop
gzy sync develop

# Specify a branch to sync
gzy sync develop feature/xxx

# Show repository status
gzy status

# Show commit log
gzy log
