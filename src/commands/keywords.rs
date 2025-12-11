// src/commands/keywords.rs
use crate::manifest::Manifest;
use crate::utils::{print_info, print_success, print_warning};
use crate::KeywordsCommands;
use anyhow::{Context, Result};
use std::path::Path;
use toml_edit::{Array, Item, Value};

pub fn handle(path: &Path, cmd: KeywordsCommands, dry_run: bool, quiet: bool) -> Result<()> {
    match cmd {
        KeywordsCommands::Add { keywords } => add(path, keywords, dry_run, quiet),
        KeywordsCommands::Remove { keywords } => remove(path, keywords, dry_run, quiet),
        KeywordsCommands::List => list(path),
        KeywordsCommands::Clear => clear(path, dry_run, quiet),
    }
}

fn add(path: &Path, keywords: Vec<String>, dry_run: bool, quiet: bool) -> Result<()> {
    let mut manifest = Manifest::load(path)?;
    let package = manifest.package_mut()?;

    let kw = package
        .entry("keywords")
        .or_insert(Item::Value(Value::Array(Array::new())))
        .as_array_mut()
        .context("keywords is not an array")?;

    let current_count = kw.len();

    for keyword in keywords {
        // Validate keyword length
        if keyword.len() > 20 {
            print_warning(&format!(
                "Keyword '{}' exceeds 20 characters (crates.io limit)",
                keyword
            ));
            continue;
        }

        // Check for non-alphanumeric characters (except hyphens and underscores)
        if !keyword
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
        {
            print_warning(&format!(
                "Keyword '{}' contains invalid characters. Only alphanumeric, hyphens, and underscores are allowed.",
                keyword
            ));
            continue;
        }

        let exists = kw.iter().any(|v| v.as_str().is_some_and(|s| s == keyword));

        if !exists {
            if current_count >= 5 {
                print_warning("Maximum 5 keywords allowed for crates.io");
                print_info(
                    &format!("Skipping '{}' and remaining keywords", keyword),
                    quiet,
                );
                break;
            }
            if !dry_run {
                kw.push(&keyword);
            }
            print_info(&format!("+ {}", keyword), quiet);
        } else {
            print_info(&format!("~ {} (already exists)", keyword), quiet);
        }
    }

    if !dry_run {
        manifest.save()?;
        print_success("Keywords updated", quiet);
    } else {
        print_info("(dry run - no changes made)", false);
    }

    Ok(())
}

fn remove(path: &Path, keywords: Vec<String>, dry_run: bool, quiet: bool) -> Result<()> {
    let mut manifest = Manifest::load(path)?;
    let package = manifest.package_mut()?;

    let kw = package
        .get_mut("keywords")
        .context("keywords field not found")?
        .as_array_mut()
        .context("keywords is not an array")?;

    for keyword in keywords {
        let mut indices = Vec::new();
        for (i, v) in kw.iter().enumerate() {
            if v.as_str().is_some_and(|s| s == keyword) {
                indices.push(i);
            }
        }

        if !indices.is_empty() {
            if !dry_run {
                for &i in indices.iter().rev() {
                    kw.remove(i);
                }
            }
            print_info(&format!("- {}", keyword), quiet);
        } else {
            print_info(&format!("? {} (not found)", keyword), quiet);
        }
    }

    if !dry_run {
        manifest.save()?;
        print_success("Keywords updated", quiet);
    } else {
        print_info("(dry run - no changes made)", false);
    }

    Ok(())
}

fn list(path: &Path) -> Result<()> {
    let manifest = Manifest::load(path)?;

    if let Some(package) = manifest.package() {
        if let Some(keywords) = package.get("keywords") {
            if let Some(arr) = keywords.as_array() {
                if arr.is_empty() {
                    println!("No keywords");
                } else {
                    println!("Keywords ({}/5):", arr.len());
                    for v in arr.iter() {
                        if let Some(s) = v.as_str() {
                            let len_indicator = if s.len() > 20 { " ⚠ (too long)" } else { "" };
                            println!("  • {}{}", s, len_indicator);
                        }
                    }
                }
                return Ok(());
            }
        }
    }
    println!("No keywords");
    Ok(())
}

fn clear(path: &Path, dry_run: bool, quiet: bool) -> Result<()> {
    let mut manifest = Manifest::load(path)?;
    let package = manifest.package_mut()?;

    if package.contains_key("keywords") {
        if !dry_run {
            package.remove("keywords");
            manifest.save()?;
        }
        print_success("All keywords cleared", quiet);
    } else {
        print_info("No keywords to clear", quiet);
    }

    Ok(())
}
