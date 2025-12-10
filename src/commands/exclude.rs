// src/commands/exclude.rs
use crate::manifest::Manifest;
use crate::utils::{print_info, print_success};
use crate::ExcludeCommands;
use anyhow::{Context, Result};
use std::path::Path;
use toml_edit::{Array, Item, Value};

pub fn handle(path: &Path, cmd: ExcludeCommands, dry_run: bool, quiet: bool) -> Result<()> {
    match cmd {
        ExcludeCommands::Add { patterns } => add(path, patterns, dry_run, quiet),
        ExcludeCommands::Remove { patterns } => remove(path, patterns, dry_run, quiet),
        ExcludeCommands::List => list(path),
        ExcludeCommands::Clear => clear(path, dry_run, quiet),
    }
}

fn add(path: &Path, patterns: Vec<String>, dry_run: bool, quiet: bool) -> Result<()> {
    let mut manifest = Manifest::load(path)?;
    let package = manifest.package_mut()?;

    let exclude = package
        .entry("exclude")
        .or_insert(Item::Value(Value::Array(Array::new())))
        .as_array_mut()
        .context("exclude is not an array")?;

    for pattern in patterns {
        let exists = exclude
            .iter()
            .any(|v| v.as_str().map_or(false, |s| s == pattern));

        if !exists {
            if !dry_run {
                exclude.push(&pattern);
            }
            print_info(&format!("+ {}", pattern), quiet);
        } else {
            print_info(&format!("~ {} (already exists)", pattern), quiet);
        }
    }

    if !dry_run {
        manifest.save()?;
        print_success("Exclude patterns updated", quiet);
    } else {
        print_info("(dry run - no changes made)", false);
    }

    Ok(())
}

fn remove(path: &Path, patterns: Vec<String>, dry_run: bool, quiet: bool) -> Result<()> {
    let mut manifest = Manifest::load(path)?;
    let package = manifest.package_mut()?;

    let exclude = package
        .get_mut("exclude")
        .context("exclude field not found")?
        .as_array_mut()
        .context("exclude is not an array")?;

    for pattern in patterns {
        let mut indices = Vec::new();
        for (i, v) in exclude.iter().enumerate() {
            if v.as_str().map_or(false, |s| s == pattern) {
                indices.push(i);
            }
        }

        if !indices.is_empty() {
            if !dry_run {
                for &i in indices.iter().rev() {
                    exclude.remove(i);
                }
            }
            print_info(&format!("- {}", pattern), quiet);
        } else {
            print_info(&format!("? {} (not found)", pattern), quiet);
        }
    }

    if !dry_run {
        manifest.save()?;
        print_success("Exclude patterns updated", quiet);
    } else {
        print_info("(dry run - no changes made)", false);
    }

    Ok(())
}

fn list(path: &Path) -> Result<()> {
    let manifest = Manifest::load(path)?;

    if let Some(package) = manifest.package() {
        if let Some(exclude) = package.get("exclude") {
            if let Some(arr) = exclude.as_array() {
                if arr.is_empty() {
                    println!("No exclude patterns");
                } else {
                    println!("Exclude patterns:");
                    for v in arr.iter() {
                        if let Some(s) = v.as_str() {
                            println!("  • {}", s);
                        }
                    }
                }
                return Ok(());
            }
        }
    }
    println!("No exclude patterns");
    Ok(())
}

fn clear(path: &Path, dry_run: bool, quiet: bool) -> Result<()> {
    let mut manifest = Manifest::load(path)?;
    let package = manifest.package_mut()?;

    if package.contains_key("exclude") {
        if !dry_run {
            package.remove("exclude");
            manifest.save()?;
        }
        print_success("All exclude patterns cleared", quiet);
    } else {
        print_info("No exclude patterns to clear", quiet);
    }

    Ok(())
}