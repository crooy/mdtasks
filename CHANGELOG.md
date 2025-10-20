# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- GitHub Actions CI/CD pipeline
- Cross-platform builds for Linux and macOS
- Universal installer with automatic OS/architecture detection
- Release automation with GitHub releases

## [0.1.0] - 2025-10-20

### Added
- Initial release of mdtasks
- Task management commands: `add`, `list`, `show`, `start`, `done`
- Subtasks support with `checklist` and `subtasks` commands
- Filtering by status, priority, and tags
- Markdown-based task storage
- Comprehensive installer script
- MIT license

### Fixed
- Fixed bug where `done` command didn't mark subtasks as complete
- Improved data consistency between task status and subtask status

### Changed
- Enhanced task management workflow
- Improved error handling and user feedback
- Added professional installation system

## [0.0.1] - 2025-10-20

### Added
- Basic MVP implementation
- Core task management functionality
- Markdown file format for tasks