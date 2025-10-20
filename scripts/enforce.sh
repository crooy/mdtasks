#!/bin/bash
# mdtasks-enforce.sh - Script to enforce mdtasks usage

set -e

echo "🔍 Checking for TODO/FIXME comments in code..."

# Check for TODO/FIXME comments in Rust files
if grep -r "TODO\|FIXME\|HACK" src/ --include="*.rs" 2>/dev/null; then
    echo "❌ Found TODO/FIXME comments in code!"
    echo "   Use 'mdtasks add' instead of TODO comments"
    echo "   Example: mdtasks add \"Fix authentication bug\" --priority high --tags bug"
    exit 1
fi

echo "✅ No TODO/FIXME comments found"

echo "🔍 Checking for active tasks..."
ACTIVE_TASKS=$(cargo run -- list --status active 2>/dev/null | grep -c "active" || echo "0")

if [ "$ACTIVE_TASKS" -gt 0 ]; then
    echo "📋 Found $ACTIVE_TASKS active task(s):"
    cargo run -- list --status active
    echo ""
    echo "💡 Remember to mark tasks as done when completed:"
    echo "   mdtasks done <id>"
fi

echo "🔍 Checking git status..."
if ! git diff --quiet; then
    echo "📝 You have uncommitted changes"
    echo "💡 Consider committing with: git commit -m \"feat: description (task #X)\""
fi

echo "✅ Pre-commit checks completed"
