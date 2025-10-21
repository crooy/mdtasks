---
id: 9
title: "Add Git integration"
status: done
priority: medium
tags: ["git"]
project: mdtasks-cli
created: 2025-10-20
completed: 2025-10-21
---

# Task Details

## Notes
Implemented comprehensive Git integration with three main commands:

**`mdtasks git-start <id>`**:
- Creates feature branch from task title (feature/ID-title)
- Auto-stashes and pulls latest changes from main
- Switches to new branch and marks task as active
- Validates git repository and branch state

**`mdtasks git-done [options]`**:
- Commits all changes with task-based message
- Pushes branch to remote
- Creates GitHub PR with task details in body
- Supports draft PRs, reviewers, labels
- Optionally switches back to main branch

**`mdtasks git-status`**:
- Shows current branch and associated task
- Displays task status and priority
- Shows git status summary

## Checklist
- [x] Implement `git-start` command for branch creation
- [x] Implement `git-done` command for PR creation
- [x] Implement `git-status` command for current state
- [x] Add GitHub CLI integration for PR creation
- [x] Add configuration support for Git settings
- [x] Handle edge cases (unstaged changes, branch conflicts)
- [x] Add proper error handling and user feedback
