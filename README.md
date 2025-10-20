# mdtasks

A minimal Rust CLI tool for managing tasks stored in markdown files with front-matter.

## Features

- **List tasks** with filtering by status, priority, and tags
- **Show task details** including full markdown content
- **Front-matter parsing** from YAML metadata
- **Git-friendly** - all tasks stored as markdown files
- **Nix development environment** with flake.nix

## Installation

### Using Nix (Recommended)

```bash
# Enter development environment
nix develop

# Build and run
cargo run -- list
```

### Manual Installation

```bash
# Clone and build
git clone <repository>
cd mdtasks
cargo build --release

# Install globally (optional)
cargo install --path .
```

## Usage

### List Tasks

```bash
# List all tasks
mdtasks list

# Filter by status
mdtasks list --status pending

# Filter by priority
mdtasks list --priority high

# Filter by tag
mdtasks list --tag cli

# Combine filters
mdtasks list --status pending --priority high
```

### Show Task Details

```bash
# Show specific task
mdtasks show 1
```

## Task File Format

Tasks are stored as markdown files with YAML front-matter:

```markdown
---
id: 001
title: "Implement MVP of mdtasks"
priority: high
status: pending
tags: [cli, rust]
project: mdtasks-cli
created: 2025-10-20
due: 2025-10-21
---

# Task Details

## Checklist
- [ ] basic project setup using rust and flake.nix
- [ ] read, list, query tasks

## Notes
Focus on the minimum we need to have task manager
```

### Front-matter Fields

- `id`: Task identifier (string or number)
- `title`: Task title (required)
- `status`: Task status (pending, active, done, etc.)
- `priority`: Task priority (low, medium, high)
- `tags`: Array of tags
- `project`: Project name
- `created`: Creation date
- `due`: Due date

## Project Structure

```
mdtasks/
├── Cargo.toml          # Rust project configuration
├── flake.nix           # Nix development environment
├── .gitignore          # Git ignore rules
├── src/
│   └── main.rs         # Main CLI application
└── tasks/              # Task files directory
    └── 001-mvp.md      # Example task file
```

## Development

### Prerequisites

- Rust toolchain
- Nix (for development environment)

### Running Tests

```bash
# Run tests
cargo test

# Run with output
cargo test -- --nocapture
```

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run directly
cargo run -- list
```

## Future Features

- [ ] `mdtasks add` - Create new tasks
- [ ] `mdtasks done <id>` - Mark tasks as done
- [ ] `mdtasks note <id>` - Add notes to tasks
- [ ] Git integration for task tracking
- [ ] GitHub issues sync
- [ ] Configuration file support
- [ ] Multiple task directories
- [ ] Task templates

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

MIT License - see LICENSE file for details.
