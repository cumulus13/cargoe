# cargoe

[![Crates.io](https://img.shields.io/crates/v/cargoe.svg)](https://crates.io/crates/cargoe)
[![Documentation](https://docs.rs/cargoe/badge.svg)](https://docs.rs/cargoe)
[![License](https://img.shields.io/crates/l/cargoe.svg)](https://github.com/cumulus13/cargoe#license)

**cargoe** is a powerful CLI tool for managing `Cargo.toml` files with features that `cargo` doesn't provide out of the box.

## Features

- 📝 **Exclude/Include Management** - Easily add, remove, and list exclude/include patterns
- 🏷️ **Keywords & Categories** - Manage keywords and categories with crates.io validation
- 🎖️ **Badges** - Add and manage repository badges
- ⚙️ **Metadata** - Custom metadata management
- 🔍 **Validation** - Validate against crates.io requirements
- 📊 **Info Display** - Quick package information summary
- 🚀 **Init Wizard** - Interactive setup for publishing
- 🎨 **Format** - Format your Cargo.toml
- 🔄 **Dry Run** - Preview changes before applying

## Installation
```bash
cargo install cargoe
```

Or from source:
```bash
git clone https://github.com/cumulus13/cargoe
cd cargoe
cargo install --path .
```

## Quick Start
```bash
# Add exclude patterns
cargoe exclude add "*.log" ".env" "tmp/"

# Add keywords
cargoe keywords add cli cargo toml

# Add categories
cargoe categories add command-line-utilities development-tools::cargo-plugins

# Set repository URL
cargoe set repository https://github.com/user/repo

# Validate for publishing
cargoe validate --strict

# Show package info
cargoe info

# Initialize for publishing (interactive)
cargoe init
```

## Usage

### Exclude Patterns
```bash
cargoe exclude add "target/" "*.log"      # Add patterns
cargoe exclude remove "*.log"             # Remove patterns
cargoe exclude list                        # List all patterns
cargoe exclude clear                       # Clear all patterns
```

### Include Patterns
```bash
cargoe include add "src/" "Cargo.toml"
cargoe include list
```

### Keywords
```bash
cargoe keywords add cli cargo toml        # Add keywords (max 5)
cargoe keywords remove cli                # Remove keyword
cargoe keywords list                       # List all keywords
```

### Categories
```bash
cargoe categories add command-line-utilities
cargoe categories list
cargoe categories valid                    # Show valid crates.io categories
```

### Badges
```bash
cargoe badges add maintenance status=actively-developed
cargoe badges add github-actions workflow=ci.yml
cargoe badges list
cargoe badges remove maintenance
```

### Set Fields
```bash
cargoe set repository https://github.com/user/repo
cargoe set homepage https://example.com
cargoe set documentation https://docs.rs/crate
cargoe set license "MIT OR Apache-2.0"
```

### Get Fields
```bash
cargoe get repository
cargoe get version
```

### Validation
```bash
cargoe validate              # Basic validation
cargoe validate --strict     # Strict validation for publishing
```

### Other Commands
```bash
cargoe info                  # Show package summary
cargoe fmt                   # Format Cargo.toml
cargoe fmt --check           # Check if formatted
cargoe init                  # Interactive initialization
cargoe init --yes            # Non-interactive initialization
```

### Global Options
```bash
--manifest-path <PATH>       # Path to Cargo.toml
--dry-run                    # Preview changes without applying
--quiet                      # Suppress output except errors
```

## Examples

### Prepare for Publishing
```bash
# Validate current state
cargoe validate --strict

# Initialize missing fields interactively
cargoe init

# Add keywords and categories
cargoe keywords add cli parser rust
cargoe categories add command-line-utilities

# Add exclude patterns
cargoe exclude add "tests/" "benches/" ".github/"

# Final validation
cargoe validate --strict
```

### Batch Operations
```bash
# Add multiple keywords at once
cargoe keywords add cli parser async tokio rust

# Add multiple exclude patterns
cargoe exclude add "*.log" "*.tmp" ".env" "tmp/" "cache/"
```

### CI/CD Integration
```bash
# In your CI pipeline
cargoe validate --strict || exit 1
cargoe fmt --check || exit 1
```

## Configuration

cargoe respects your `Cargo.toml` formatting and comments. It uses `toml_edit` to preserve the original structure.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

## Author

**Hadi Cahyadi**
- Email: cumulus13@gmail.com
- GitHub: [cumulus13](https://github.com/cumulus13)

[![Buy Me a Coffee](https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png)](https://www.buymeacoffee.com/cumulus13)

[![Donate via Ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/cumulus13)
 
[Support me on Patreon](https://www.patreon.com/cumulus13)


## Acknowledgments

Built with:
- [clap](https://github.com/clap-rs/clap) - Command line argument parsing
- [toml_edit](https://github.com/ordian/toml_edit) - TOML manipulation
- [anyhow](https://github.com/dtolnay/anyhow) - Error handling