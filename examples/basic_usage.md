# Basic Usage Examples

## Quick Start

### 1. Add Exclude Patterns
```bash
# Add common build artifacts
cargoe exclude add "target/" "*.log" ".env"

# Add test and CI files
cargoe exclude add "tests/" ".github/" "benches/"
```

### 2. Manage Keywords
```bash
# Add up to 5 keywords
cargoe keywords add cli cargo toml rust development-tools

# List current keywords
cargoe keywords list

# Remove a keyword
cargoe keywords remove toml
```

### 3. Set Categories
```bash
# Add valid crates.io categories
cargoe categories add command-line-utilities
cargoe categories add development-tools::cargo-plugins

# Check valid categories
cargoe categories valid
```

### 4. Set Package Fields
```bash
# Set repository
cargoe set repository https://github.com/user/cargoe

# Set homepage
cargoe set homepage https://cargoe.dev

# Set documentation URL
cargoe set documentation https://docs.rs/cargoe

# Set license
cargoe set license "MIT OR Apache-2.0"

# Set multiple authors (comma-separated)
cargoe set authors "John Doe <john@example.com>, Jane Smith <jane@example.com>"
```

## Advanced Usage

### Badges
```bash
# Add maintenance badge
cargoe badges add maintenance status=actively-developed

# Add GitHub Actions badge
cargoe badges add github-actions workflow=ci.yml branch=main

# List all badges
cargoe badges list

# Remove a badge
cargoe badges remove maintenance
```

### Custom Metadata
```bash
# Add simple metadata
cargoe metadata add my-tool.version 1.0.0

# Add nested metadata with JSON
cargoe metadata add my-tool.config '{"enabled": true, "level": 5}' --json

# List all metadata
cargoe metadata list

# Remove metadata
cargoe metadata remove my-tool.version
```

### Validation
```bash
# Basic validation
cargoe validate

# Strict validation for publishing to crates.io
cargoe validate --strict
```

### Initialization
```bash
# Interactive initialization
cargoe init

# Non-interactive (uses defaults)
cargoe init --yes
```

## Complete Workflow Example

```bash
# 1. Start with a new project
cargo new my-awesome-crate
cd my-awesome-crate

# 2. Set basic information
cargoe set description "An awesome Rust crate"
cargoe set repository https://github.com/user/my-awesome-crate
cargoe set homepage https://my-awesome-crate.dev
cargoe set license "MIT OR Apache-2.0"

# 3. Add keywords and categories
cargoe keywords add awesome rust cli performance utility
cargoe categories add command-line-utilities

# 4. Configure exclusions
cargoe exclude add "tests/" ".github/" "*.log" "tmp/"

# 5. Add badges
cargoe badges add maintenance status=actively-developed
cargoe badges add github-actions workflow=ci.yml

# 6. Validate before publishing
cargoe validate --strict

# 7. View final configuration
cargoe info

# 8. Ready to publish!
cargo publish --dry-run
```

## Tips and Tricks

### Preview Changes
Use `--dry-run` to see what would change without modifying files:
```bash
cargoe --dry-run exclude add "*.tmp"
cargoe --dry-run keywords add new-keyword
```

### Quiet Mode
Suppress output for scripting:
```bash
cargoe --quiet exclude add "target/"
```

### Multiple Operations
Chain operations in scripts:
```bash
#!/bin/bash
cargoe exclude add "*.log" "target/" ".env"
cargoe keywords add cli rust awesome
cargoe categories add command-line-utilities
cargoe set repository https://github.com/user/repo
cargoe validate --strict
```

### CI/CD Integration
```yaml
# .github/workflows/validate.yml
name: Validate
on: [push, pull_request]
jobs:
  validate:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo install cargoe
      - run: cargoe validate --strict
      - run: cargoe fmt --check
```

### Format Check in Pre-commit Hook
```bash
#!/bin/bash
# .git/hooks/pre-commit
cargoe fmt --check || {
    echo "Cargo.toml is not formatted. Run 'cargoe fmt' to fix."
    exit 1
}
```

## Common Patterns

### Library Crate Setup
```bash
cargoe exclude add "examples/" "tests/" "benches/"
cargoe keywords add library rust api
cargoe categories add api-bindings
cargoe set readme README.md
```

### CLI Tool Setup
```bash
cargoe exclude add "*.log" ".env" "tmp/"
cargoe keywords add cli command-line tool
cargoe categories add command-line-utilities
```

### Web Framework Setup
```bash
cargoe keywords add web framework http async
cargoe categories add web-programming web-programming::http-server
```

### Game Development Setup
```bash
cargoe keywords add game gamedev engine
cargoe categories add game-development game-engines
```