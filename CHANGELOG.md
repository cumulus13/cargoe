# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2024-12-10

### Added
- Initial release
- Exclude pattern management (add, remove, list, clear)
- Include pattern management (add, remove, list, clear)
- Keywords management with crates.io validation (max 5, max 20 chars each)
- Categories management with crates.io validation (max 5)
- Valid crates.io categories reference
- Badges management (add, remove, list, clear)
- Custom metadata management with nested key support
- Set/Get package fields (repository, homepage, documentation, etc.)
- Validation command with strict mode for crates.io requirements
- Format command to format Cargo.toml
- Info command to display package summary
- Init command for interactive setup
- Dry-run mode (`--dry-run`) to preview changes
- Quiet mode (`--quiet`) to suppress output
- Custom manifest path support (`--manifest-path`)
- Comprehensive integration tests
- CI/CD with GitHub Actions
- Cross-platform support (Linux, macOS, Windows)

### Features
- Preserves Cargo.toml formatting and comments
- Prevents duplicate entries
- Input validation for keywords and categories
- Helpful hints and warnings
- Colored output for better readability
- Production-ready error handling

[Unreleased]: https://github.com/cumulus13/cargoe/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/cumulus13/cargoe/releases/tag/v0.1.0