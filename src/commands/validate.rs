// src/commands/validate.rs
use crate::manifest::Manifest;
use crate::utils::{print_error, print_success, print_warning};
use anyhow::Result;
use colored::*;
use std::path::Path;

pub fn handle(path: &Path, strict: bool) -> Result<()> {
    let manifest = Manifest::load(path)?;
    let mut errors = Vec::new();
    let mut warnings = Vec::new();

    println!("{}", "Validating Cargo.toml...".bold());
    println!();

    // Check required fields
    if let Some(package) = manifest.package() {
        check_required_field(package, "name", &mut errors);
        check_required_field(package, "version", &mut errors);

        if strict {
            // Additional checks for publishing
            check_required_field(package, "description", &mut errors);
            check_required_field(package, "license", &mut errors);

            // Recommended fields
            if !package.contains_key("repository") {
                warnings.push("Missing 'repository' field (recommended for crates.io)".to_string());
            }
            if !package.contains_key("readme") {
                warnings.push("Missing 'readme' field (recommended for crates.io)".to_string());
            }
            if !package.contains_key("keywords") {
                warnings.push("Missing 'keywords' field (recommended for crates.io)".to_string());
            }
            if !package.contains_key("categories") {
                warnings.push("Missing 'categories' field (recommended for crates.io)".to_string());
            }

            // Check description length
            if let Some(desc) = package.get("description").and_then(|v| v.as_str()) {
                if desc.len() > 160 {
                    warnings.push(
                        "Description exceeds 160 characters (crates.io will truncate)".to_string(),
                    );
                }
                if desc.len() < 10 {
                    warnings.push("Description is very short (consider expanding)".to_string());
                }
            }

            // Check keywords count and length
            if let Some(keywords) = package.get("keywords").and_then(|v| v.as_array()) {
                if keywords.len() > 5 {
                    errors.push("Too many keywords (max 5 for crates.io)".to_string());
                }
                for kw in keywords.iter() {
                    if let Some(s) = kw.as_str() {
                        if s.len() > 20 {
                            errors.push(format!("Keyword '{}' exceeds 20 characters", s));
                        }
                    }
                }
            }

            // Check categories count
            if let Some(categories) = package.get("categories").and_then(|v| v.as_array()) {
                if categories.len() > 5 {
                    errors.push("Too many categories (max 5 for crates.io)".to_string());
                }
            }

            // Check license format
            if let Some(license) = package.get("license").and_then(|v| v.as_str()) {
                if !is_valid_spdx_license(license) {
                    warnings.push(format!(
                        "License '{}' may not be a valid SPDX expression",
                        license
                    ));
                }
            }
        }

        // Check version format
        if let Some(version) = package.get("version").and_then(|v| v.as_str()) {
            if !is_valid_semver(version) {
                errors.push(format!("Invalid version format: '{}'", version));
            }
        }
    } else {
        errors.push("Missing [package] section".to_string());
    }

    // Print results
    println!("{}", "Results:".bold());
    println!();

    if errors.is_empty() && warnings.is_empty() {
        print_success("All checks passed!", false);
        Ok(())
    } else {
        if !errors.is_empty() {
            println!("{}", "Errors:".red().bold());
            for err in &errors {
                print_error(err);
            }
            println!();
        }

        if !warnings.is_empty() {
            println!("{}", "Warnings:".yellow().bold());
            for warn in &warnings {
                print_warning(warn);
            }
            println!();
        }

        if !errors.is_empty() {
            anyhow::bail!("Validation failed with {} error(s)", errors.len());
        }

        Ok(())
    }
}

fn check_required_field(package: &toml_edit::Table, field: &str, errors: &mut Vec<String>) {
    if !package.contains_key(field) {
        errors.push(format!("Missing required field: '{}'", field));
    }
}

fn is_valid_semver(version: &str) -> bool {
    let parts: Vec<&str> = version.split('.').collect();
    if parts.len() != 3 {
        return false;
    }
    parts.iter().all(|p| p.parse::<u32>().is_ok())
}

fn is_valid_spdx_license(license: &str) -> bool {
    // Basic SPDX validation - common licenses
    let common = [
        "MIT",
        "Apache-2.0",
        "GPL-3.0",
        "BSD-3-Clause",
        "ISC",
        "MPL-2.0",
        "MIT OR Apache-2.0",
        "MIT AND Apache-2.0",
    ];
    // common.iter().any(|&l| license == l) || license.contains(" OR ") || license.contains(" AND ")
    common.contains(&license) || license.contains(" OR ") || license.contains(" AND ")
}
