---
id: 11
title: "Add enhanced filtering"
status: active
priority: medium
tags: ["filtering"]
project: mdtasks-cli
created: 2025-10-20
---
# Task Details

## Notes
Add --project, --due, date ranges, and better search capabilities

Updated documentation:

✅ README.md:
  - Updated Quick Start examples to use 'subtasks add' and 'subtasks list'
  - Updated Basic Commands section with new subtasks subcommand
  - Added dedicated 'Subtasks Management' section with complete examples
  - Updated feature description to mention individual completion control

✅ install.sh:
  - Updated usage examples to show 'subtasks add' and 'subtasks list'
  - Removed outdated 'checklist' command references

All documentation now reflects the unified subtasks subcommand structure.
Successfully refactored checklist/subtasks structure:

✅ REMOVED: 'checklist' command (redundant)
✅ ADDED: 'subtasks' subcommand with 4 actions:
  - subtasks add <id> <item>     # Add subtask
  - subtasks list <id>           # List subtasks  
  - subtasks complete <id> <n>   # Mark subtask #n complete
  - subtasks incomplete <id> <n> # Mark subtask #n incomplete

✅ UPDATED: Markdown section from '## Checklist' to '## Subtasks'
✅ IMPROVED: Individual subtask completion control
✅ CLEANER: Single unified interface for subtask management

This eliminates confusion between checklist/subtasks and provides better granular control.
## Checklist
- [ ] 
- [ ] Test new subtasks functionality
- [ ] Remove redundant checklist command
- [ ] Convert subtasks to subcommand structure
- [ ] Add individual subtask completion/incompletion
- [ ] Update markdown section name to Subtasks
- [ ] Update README.md examples
- [ ] Update install script examples
