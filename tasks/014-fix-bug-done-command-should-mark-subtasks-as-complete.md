---
id: 14
title: "Fix bug: done command should mark subtasks as complete"
status: done
priority: high
tags: ["bug"]
created: 2025-10-20
completed: 2025-10-20
---

# Task Details

## Description
**Bug**: When a task is marked as "done" using `mdtasks done <id>`, the subtasks/checklist items remain marked as incomplete (`- [ ]`) instead of being marked as complete (`- [x]`).

**Current Behavior**: 
- Task status changes to "done" in front-matter
- Checklist items remain as incomplete checkboxes
- This creates inconsistency between task status and subtask status

**Expected Behavior**:
- When a task is marked as done, all subtasks should be marked as complete
- Subtasks display should show completed items with ✅ instead of ⏳

**Impact**: High - affects data consistency and user experience

## Checklist
- [x] 
- [x] Analyze current done command behavior
- [x] Update mark_task_done function to mark all subtasks as complete
- [x] Test the fix with existing completed tasks
- [x] Verify subtasks display correctly for done tasks
- [x] Analyze current done command behavior
- [x] Update mark_task_done function to mark all subtasks as complete
- [x] Test the fix with existing completed tasks
- [x] Verify subtasks display correctly for done tasks
