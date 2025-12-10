// src/commands/badges.rs
use crate::manifest::Manifest;
use crate::utils::{print_info, print_success, print_warning};
use crate::BadgesCommands;
use anyhow::{Context, Result};
use std::path::Path;
use toml_edit::{Item, Table};

pub fn handle(path: &Path, cmd: BadgesCommands, dry_run: bool, quiet: bool) -> Result<()> {
    match cmd {
        BadgesCommands::Add {
            badge_type,
            attributes,
        } => add(path, &badge_type, attributes, dry_run, quiet),
        BadgesCommands::Remove { badge_type } => remove(path, &badge_type, dry_run, quiet),
        BadgesCommands::List => list(path),
        BadgesCommands::Clear => clear(path, dry_run, quiet),
    }
}

fn add(
    path: &Path,
    badge_type: &str,
    attributes: Vec<(String, String)>,
    dry_run: bool,
    quiet: bool,
) -> Result<()> {
    let mut manifest = Manifest::load(path)?;

    let badges = manifest
        .doc
        .entry("badges")
        .or_insert(Item::Table(Table::new()))
        .as_table_mut()
        .context("badges is not a table")?;

    if badges.contains_key(badge_type) {
        print_warning(&format!("Badge '{}' already exists, updating...", badge_type));
    }

    let badge_table = badges
        .entry(badge_type)
        .or_insert(Item::Table(Table::new()))
        .as_table_mut()
        .context("badge entry is not a table")?;

    for (key, value) in attributes {
        if !dry_run {
            badge_table.insert(&key, toml_edit::value(value.clone()));
        }
        print_info(&format!("  {} = {}", key, value), quiet);
    }

    if !dry_run {
        manifest.save()?;
        print_success(&format!("Badge '{}' added", badge_type), quiet);
    } else {
        print_info("(dry run - no changes made)", false);
    }

    Ok(())
}

fn remove(path: &Path, badge_type: &str, dry_run: bool, quiet: bool) -> Result<()> {
    let mut manifest = Manifest::load(path)?;

    let badges = manifest
        .doc
        .get_mut("badges")
        .context("badges section not found")?
        .as_table_mut()
        .context("badges is not a table")?;

    if badges.contains_key(badge_type) {
        if !dry_run {
            badges.remove(badge_type);
            manifest.save()?;
        }
        print_success(&format!("Badge '{}' removed", badge_type), quiet);
    } else {
        print_info(&format!("Badge '{}' not found", badge_type), quiet);
    }

    Ok(())
}

fn list(path: &Path) -> Result<()> {
    let manifest = Manifest::load(path)?;

    if let Some(badges) = manifest.doc.get("badges") {
        if let Some(table) = badges.as_table() {
            if table.is_empty() {
                println!("No badges defined");
            } else {
                println!("Badges:");
                for (badge_type, badge_config) in table.iter() {
                    println!("\n  [{}]", badge_type);
                    if let Some(config_table) = badge_config.as_table() {
                        for (key, value) in config_table.iter() {
                            if let Some(val_str) = value.as_str() {
                                println!("    {} = \"{}\"", key, val_str);
                            }
                        }
                    }
                }
            }
            return Ok(());
        }
    }
    println!("No badges defined");
    Ok(())
}

fn clear(path: &Path, dry_run: bool, quiet: bool) -> Result<()> {
    let mut manifest = Manifest::load(path)?;

    if manifest.doc.contains_key("badges") {
        if !dry_run {
            manifest.doc.remove("badges");
            manifest.save()?;
        }
        print_success("All badges cleared", quiet);
    } else {
        print_info("No badges to clear", quiet);
    }

    Ok(())
}