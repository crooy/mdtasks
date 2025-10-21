---
id: 7
title: "Implement note command"
status: done
priority: medium
tags: ["cli"]
project: mdtasks-cli
created: 2025-10-20
completed: 2025-10-21
---

# Task Details

## Notes
Implemented `mdtasks add-note <id> <note>` command to add notes to existing tasks. The command:
- Finds the task by ID
- Adds the note to the Notes section of the task file
- Creates a Notes section if it doesn't exist
- Preserves existing content and formatting

## Checklist
- [x] Add `AddNote` command to CLI interface
- [x] Implement note parsing and insertion logic
- [x] Handle cases where Notes section doesn't exist
- [x] Preserve existing task content and formatting
- [x] Add proper error handling for invalid task IDs
