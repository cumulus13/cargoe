// src/commands/set.rs
use crate::manifest::Manifest;
use crate::utils::{print_info, print_success, print_warning};
use anyhow::Result;
use std::path::Path;

const VALID_FIELDS: &[&str] = &[
    "repository",
    "homepage",
    "documentation",
    "readme",
    "license",
    "license-file",
    "description",
    "authors",
    "edition",
    "rust-version",
    "version",
];

pub fn handle(path: &Path, field: &str, value: &str, dry_run: bool, quiet: bool) -> Result<()> {
    if !VALID_FIELDS.contains(&field) {
        print_warning(&format!(
            "Field '{}' is not a commonly set package field",
            field
        ));
        println!("Valid fields: {}", VALID_FIELDS.join(", "));
    }

    let mut manifest = Manifest::load(path)?;

    // Check for hints before mutating
    let should_hint_homepage = field == "repository"
        && manifest
            .package()
            .is_some_and(|p| !p.contains_key("homepage"));
    let should_hint_license_file = field == "license"
        && manifest
            .package()
            .is_some_and(|p| !p.contains_key("license-file"));

    let package = manifest.package_mut()?;

    // Special handling for arrays
    let item_value = if field == "authors" {
        // Parse authors as array if comma-separated
        if value.contains(',') {
            let authors: Vec<&str> = value.split(',').map(|s| s.trim()).collect();
            let mut arr = toml_edit::Array::new();
            for author in authors {
                arr.push(author);
            }
            toml_edit::Item::Value(toml_edit::Value::Array(arr))
        } else {
            // Single author - still use array
            let mut arr = toml_edit::Array::new();
            arr.push(value);
            toml_edit::Item::Value(toml_edit::Value::Array(arr))
        }
    } else {
        toml_edit::value(value)
    };

    if !dry_run {
        package.insert(field, item_value);
        manifest.save()?;
    }

    print_info(&format!("{} = \"{}\"", field, value), quiet);

    if !dry_run {
        print_success(&format!("Field '{}' updated", field), quiet);
    } else {
        print_info("(dry run - no changes made)", false);
    }

    // Provide hints for related fields
    if should_hint_homepage {
        print_info(
            "Hint: Consider also setting 'homepage' with: cargoe set homepage <url>",
            false,
        );
    }
    if should_hint_license_file {
        print_info(
            "Hint: If using a custom license, set 'license-file' instead",
            false,
        );
    }

    Ok(())
}
