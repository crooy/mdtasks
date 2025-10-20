#!/bin/bash

# Release script for mdtasks
# Usage: ./scripts/release.sh <version>
# Example: ./scripts/release.sh v0.2.0

set -e

if [ $# -eq 0 ]; then
    echo "Usage: $0 <version>"
    echo "Example: $0 v0.2.0"
    exit 1
fi

VERSION=$1

# Validate version format
if [[ ! $VERSION =~ ^v[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    echo "❌ Invalid version format. Use semantic versioning (e.g., v0.2.0)"
    exit 1
fi

echo "🚀 Creating release $VERSION..."

# Check if we're on main branch
CURRENT_BRANCH=$(git branch --show-current)
if [ "$CURRENT_BRANCH" != "main" ]; then
    echo "❌ You must be on the main branch to create a release"
    echo "Current branch: $CURRENT_BRANCH"
    exit 1
fi

# Check if working directory is clean
if [ -n "$(git status --porcelain)" ]; then
    echo "❌ Working directory is not clean. Please commit or stash changes first."
    git status --short
    exit 1
fi

# Check if version already exists
if git tag -l | grep -q "^$VERSION$"; then
    echo "❌ Tag $VERSION already exists"
    exit 1
fi

# Update version in Cargo.toml
echo "📝 Updating version in Cargo.toml..."
sed -i.bak "s/^version = \".*\"/version = \"${VERSION#v}\"/" Cargo.toml
rm Cargo.toml.bak

# Update CHANGELOG.md
echo "📝 Updating CHANGELOG.md..."
TODAY=$(date +%Y-%m-%d)
sed -i.bak "s/## \[Unreleased\]/## \[Unreleased\]\n\n## \[${VERSION#v}\] - $TODAY/" CHANGELOG.md
rm CHANGELOG.md.bak

# Commit changes
echo "📝 Committing version changes..."
git add Cargo.toml CHANGELOG.md
git commit -m "chore: bump version to $VERSION"

# Create tag
echo "🏷️  Creating tag $VERSION..."
git tag -a "$VERSION" -m "Release $VERSION"

# Push changes and tag
echo "📤 Pushing changes and tag..."
git push origin main
git push origin "$VERSION"

echo "✅ Release $VERSION created successfully!"
echo ""
echo "The GitHub Actions workflow will now:"
echo "  - Build binaries for Linux and macOS"
echo "  - Create installers for each platform"
echo "  - Create a GitHub release with all artifacts"
echo ""
echo "You can monitor the progress at:"
echo "  https://github.com/$(git config --get remote.origin.url | sed 's/.*github.com[:/]\([^.]*\).*/\1/')/actions"