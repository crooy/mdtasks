---
id: 23
title: "Rename git-finish to git-done and add PR creation"
status: done
priority: high
tags: ["feature", "git"]
created: 2025-10-21
completed: 2025-10-21
---

# Task Details

## Notes
Successfully renamed `git-finish` to `git-done` and implemented comprehensive PR creation functionality:

**Key Features Implemented**:
- Renamed command from `git-finish` to `git-done` for better clarity
- Added GitHub CLI integration for automatic PR creation
- PR body includes full task details (title, status, priority, tags, project)
- Support for draft PRs, reviewers, and labels
- Configurable PR settings via mdtasks.toml
- Auto-switch back to main branch option
- Proper error handling and user feedback

**Configuration Options**:
- `pr_enabled`: Enable/disable PR creation
- `pr_draft`: Create draft PRs by default
- `pr_auto_assign`: Auto-assign reviewers
- `pr_switch_to_main`: Auto-switch to main after PR
- `pr_default_reviewers`: Default reviewers list
- `pr_default_labels`: Default labels list

## Checklist
- [x] Rename `git-finish` command to `git-done`
- [x] Implement GitHub CLI integration
- [x] Add PR body with task details
- [x] Support draft PR creation
- [x] Add reviewer and label support
- [x] Add configuration file support
- [x] Implement auto-switch to main option
- [x] Add proper error handling
- [x] Test with GitHub CLI
