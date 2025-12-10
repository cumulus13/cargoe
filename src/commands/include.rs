// src/commands/include.rs
use crate::manifest::Manifest;
use crate::utils::{print_info, print_success};
use crate::IncludeCommands;
use anyhow::{Context, Result};
use std::path::Path;
use toml_edit::{Array, Item, Value};

pub fn handle(path: &Path, cmd: IncludeCommands, dry_run: bool, quiet: bool) -> Result<()> {
    match cmd {
        IncludeCommands::Add { patterns } => add(path, patterns, dry_run, quiet),
        IncludeCommands::Remove { patterns } => remove(path, patterns, dry_run, quiet),
        IncludeCommands::List => list(path),
        IncludeCommands::Clear => clear(path, dry_run, quiet),
    }
}

fn add(path: &Path, patterns: Vec<String>, dry_run: bool, quiet: bool) -> Result<()> {
    let mut manifest = Manifest::load(path)?;
    let package = manifest.package_mut()?;

    let include = package
        .entry("include")
        .or_insert(Item::Value(Value::Array(Array::new())))
        .as_array_mut()
        .context("include is not an array")?;

    for pattern in patterns {
        let exists = include
            .iter()
            .any(|v| v.as_str().map_or(false, |s| s == pattern));

        if !exists {
            if !dry_run {
                include.push(&pattern);
            }
            print_info(&format!("+ {}", pattern), quiet);
        } else {
            print_info(&format!("~ {} (already exists)", pattern), quiet);
        }
    }

    if !dry_run {
        manifest.save()?;
        print_success("Include patterns updated", quiet);
    } else {
        print_info("(dry run - no changes made)", false);
    }

    Ok(())
}

fn remove(path: &Path, patterns: Vec<String>, dry_run: bool, quiet: bool) -> Result<()> {
    let mut manifest = Manifest::load(path)?;
    let package = manifest.package_mut()?;

    let include = package
        .get_mut("include")
        .context("include field not found")?
        .as_array_mut()
        .context("include is not an array")?;

    for pattern in patterns {
        let mut indices = Vec::new();
        for (i, v) in include.iter().enumerate() {
            if v.as_str().map_or(false, |s| s == pattern) {
                indices.push(i);
            }
        }

        if !indices.is_empty() {
            if !dry_run {
                for &i in indices.iter().rev() {
                    include.remove(i);
                }
            }
            print_info(&format!("- {}", pattern), quiet);
        } else {
            print_info(&format!("? {} (not found)", pattern), quiet);
        }
    }

    if !dry_run {
        manifest.save()?;
        print_success("Include patterns updated", quiet);
    } else {
        print_info("(dry run - no changes made)", false);
    }

    Ok(())
}

fn list(path: &Path) -> Result<()> {
    let manifest = Manifest::load(path)?;

    if let Some(package) = manifest.package() {
        if let Some(include) = package.get("include") {
            if let Some(arr) = include.as_array() {
                if arr.is_empty() {
                    println!("No include patterns");
                } else {
                    println!("Include patterns:");
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
    println!("No include patterns");
    Ok(())
}

fn clear(path: &Path, dry_run: bool, quiet: bool) -> Result<()> {
    let mut manifest = Manifest::load(path)?;
    let package = manifest.package_mut()?;

    if package.contains_key("include") {
        if !dry_run {
            package.remove("include");
            manifest.save()?;
        }
        print_success("All include patterns cleared", quiet);
    } else {
        print_info("No include patterns to clear", quiet);
    }

    Ok(())
}