# mdtasks

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)

A **Git-powered, Markdown-based Task Manager** that produces **human-readable, AI-friendly, and Git-web-UI-compatible** task files.

Perfect for developers who want their tasks to be:
- 🤖 **LLM/AI-friendly**: Works seamlessly with Cursor AI, GitHub Copilot, ChatGPT, and other coding agents
- 🌐 **Git web UI compatible**: Beautiful rendering in GitHub, GitLab, Bitbucket, and other Git platforms
- 👥 **Human-readable**: Easy to read, edit, and collaborate on without special tools
- 📝 **Version controlled**: Full Git history, branching, and merge conflict resolution
- 🔍 **Searchable**: Find tasks with standard text search tools (`grep`, `ripgrep`, etc.)

## Features

- ✅ **Task Management**: Create, list, show, start, and complete tasks
- ✅ **Subtasks**: Add checklist items and track subtask progress
- ✅ **Filtering**: Filter tasks by status, priority, and tags
- ✅ **Markdown Storage**: Tasks stored as readable markdown files
- ✅ **Git Integration**: Version control your tasks with git

## Quick Start

```bash
# Install mdtasks (latest release)
curl -sSL https://raw.githubusercontent.com/crooy/mdtasks/main/install | bash

# Create your first task
mdtasks add "Learn mdtasks" --priority high

# Start working on it
mdtasks start 1

# Add some subtasks
mdtasks checklist 1 "Read the documentation"
mdtasks checklist 1 "Try the examples"

# View your progress
mdtasks subtasks 1

# Mark as complete
mdtasks done 1
```

## Installation

### Option 1: Install from Source (Recommended)

1. Clone the repository:
   ```bash
   git clone https://github.com/crooy/mdtasks.git
   cd mdtasks
   ```

2. Run the installer:
   ```bash
   ./scripts/install.sh
   ```

3. Verify installation:
   ```bash
   mdtasks --help
   ```

### Option 2: Install with Cargo

If you have Rust installed:

```bash
# Install from GitHub
cargo install --git https://github.com/crooy/mdtasks.git

# Or install from crates.io (when published)
cargo install mdtasks
```

### Option 3: Build from Source

```bash
git clone https://github.com/crooy/mdtasks.git
cd mdtasks
cargo build --release
# Binary will be in target/release/mdtasks
```

## Usage

### Basic Commands

```bash
# List all tasks
mdtasks list

# Add a new task
mdtasks add "Implement new feature" --priority high --tags feature

# Start working on a task
mdtasks start 1

# Add subtasks/checklist items
mdtasks checklist 1 "Write tests"
mdtasks checklist 1 "Update documentation"

# View subtasks
mdtasks subtasks 1

# Mark task as done
mdtasks done 1

# Show task details
mdtasks show 1
```

### Filtering

```bash
# List only active tasks
mdtasks list --status active

# List high priority tasks
mdtasks list --priority high

# List tasks with specific tag
mdtasks list --tag feature
```

## Why Markdown-Based Tasks?

### 🤖 **Perfect for LLM Coding Agents**

Your tasks are stored in plain markdown files that AI assistants can easily read and understand:

- **Cursor AI**: Can read your task files directly and help implement features
- **GitHub Copilot**: Understands task context when working on related code
- **ChatGPT/Claude**: Can analyze your task list and provide suggestions
- **Custom AI tools**: Easy to parse and process with any AI system

### 🌐 **Beautiful Git Web UI Rendering**

Your tasks look great in any Git web interface:

- **GitHub**: Tasks display with proper markdown formatting, checkboxes, and syntax highlighting
- **GitLab**: Full markdown support with task metadata visible
- **Bitbucket**: Clean, readable task display
- **Self-hosted Git**: Works with Gitea, Forgejo, and other Git platforms

### 📝 **Human-Friendly Format**

No special tools required - anyone can read and edit your tasks:

```markdown
---
id: 1
title: "Implement user authentication"
status: active
priority: high
tags: ["backend", "security"]
project: my-app
created: 2025-10-20
due: 2025-10-25
---

# Task Details

## Notes
Add JWT-based authentication with proper security measures.

## Checklist
- [ ] Design authentication flow
- [x] Set up JWT library
- [ ] Implement login endpoint
- [ ] Add password hashing
- [ ] Write authentication tests
```

## Integration Examples

### With Cursor AI
```bash
# Cursor AI can read your tasks and help implement them
mdtasks list --status active
# Cursor AI: "I see you have 3 active tasks. Let me help implement the authentication system..."
```

### With Git Web UI
When you push your tasks to GitHub, they automatically render beautifully:
- ✅ Checkboxes show progress visually
- 📊 Metadata is clearly displayed
- 🔍 Full-text search works across all tasks
- 📝 Easy to edit directly in the web interface

## Uninstallation

To uninstall mdtasks:

```bash
./scripts/install.sh --uninstall
```

Or if installed via cargo:

```bash
cargo uninstall mdtasks
```

## Development

### Prerequisites

- Rust 1.70+
- Cargo

### Building

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

### Development Installation

```bash
cargo install --path .
```

### Creating a Release

```bash
# Create a new release (this will trigger GitHub Actions)
./scripts/release.sh v0.2.0
```

The release script will:
- Update version numbers
- Create a git tag
- Push to GitHub
- Trigger automated builds for Linux and macOS
- Create GitHub release with installers

### CI/CD

This project uses GitHub Actions for:
- **CI**: Automated testing on Linux, macOS, and Windows
- **Release**: Automated building and packaging for releases
- **Cross-platform**: Builds for Linux x86_64, macOS x86_64, and macOS ARM64

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

MIT License - see LICENSE file for details.
# Test change
