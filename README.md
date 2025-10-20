# mdtasks

A command-line task manager that uses markdown files for task storage.

## Features

- ✅ **Task Management**: Create, list, show, start, and complete tasks
- ✅ **Subtasks**: Add checklist items and track subtask progress
- ✅ **Filtering**: Filter tasks by status, priority, and tags
- ✅ **Markdown Storage**: Tasks stored as readable markdown files
- ✅ **Git Integration**: Version control your tasks with git

## Installation

### Option 1: Install from Source (Recommended)

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/mdtasks.git
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
cargo install --git https://github.com/yourusername/mdtasks.git
```

### Option 3: Build from Source

```bash
git clone https://github.com/yourusername/mdtasks.git
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

## Task File Format

Tasks are stored as markdown files in the `tasks/` directory with YAML front-matter:

```markdown
---
id: 1
title: "Implement new feature"
status: active
priority: high
tags: ["feature", "backend"]
project: my-project
created: 2025-10-20
due: 2025-10-25
---

# Task Details

## Notes
This is a detailed description of the task.

## Checklist
- [ ] Write tests
- [x] Update documentation
- [ ] Deploy to staging
```

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

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

MIT License - see LICENSE file for details.