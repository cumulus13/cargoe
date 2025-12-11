// src/commands/metadata.rs
use crate::manifest::Manifest;
use crate::utils::{print_info, print_success};
use crate::MetadataCommands;
use anyhow::{Context, Result};
use std::path::Path;
use toml_edit::{Item, Table};

pub fn handle(path: &Path, cmd: MetadataCommands, dry_run: bool, quiet: bool) -> Result<()> {
    match cmd {
        MetadataCommands::Add { key, value, json } => add(path, &key, &value, json, dry_run, quiet),
        MetadataCommands::Remove { key } => remove(path, &key, dry_run, quiet),
        MetadataCommands::List => list(path),
        MetadataCommands::Clear => clear(path, dry_run, quiet),
    }
}

fn add(path: &Path, key: &str, value: &str, json: bool, dry_run: bool, quiet: bool) -> Result<()> {
    let mut manifest = Manifest::load(path)?;
    let package = manifest.package_mut()?;

    let metadata = package
        .entry("metadata")
        .or_insert(Item::Table(Table::new()))
        .as_table_mut()
        .context("metadata is not a table")?;

    let keys: Vec<&str> = key.split('.').collect();
    let mut current_table = metadata;

    // Navigate/create nested structure
    for (i, k) in keys.iter().enumerate() {
        if i == keys.len() - 1 {
            // Last key - set the value
            let item_value = if json {
                let parsed: serde_json::Value =
                    serde_json::from_str(value).context("Invalid JSON value")?;
                json_to_toml_item(&parsed)?
            } else {
                toml_edit::value(value)
            };

            if !dry_run {
                current_table.insert(k, item_value);
            }
            print_info(&format!("+ {} = {}", key, value), quiet);
        } else {
            // Intermediate key - ensure it's a table
            current_table = current_table
                .entry(k)
                .or_insert(Item::Table(Table::new()))
                .as_table_mut()
                .context(format!("'{}' is not a table", k))?;
        }
    }

    if !dry_run {
        manifest.save()?;
        print_success("Metadata updated", quiet);
    } else {
        print_info("(dry run - no changes made)", false);
    }

    Ok(())
}

fn remove(path: &Path, key: &str, dry_run: bool, quiet: bool) -> Result<()> {
    let mut manifest = Manifest::load(path)?;
    let package = manifest.package_mut()?;

    let metadata = package
        .get_mut("metadata")
        .context("metadata section not found")?
        .as_table_mut()
        .context("metadata is not a table")?;

    let keys: Vec<&str> = key.split('.').collect();
    let mut current_table = metadata;

    // Navigate to parent of target key
    for (i, k) in keys.iter().enumerate() {
        if i == keys.len() - 1 {
            // Last key - remove it
            if current_table.contains_key(k) {
                if !dry_run {
                    current_table.remove(k);
                    manifest.save()?;
                }
                print_success(&format!("Metadata '{}' removed", key), quiet);
            } else {
                print_info(&format!("Metadata '{}' not found", key), quiet);
            }
            return Ok(());
        } else {
            current_table = current_table
                .get_mut(k)
                .context(format!("Key '{}' not found", k))?
                .as_table_mut()
                .context(format!("'{}' is not a table", k))?;
        }
    }

    Ok(())
}

fn list(path: &Path) -> Result<()> {
    let manifest = Manifest::load(path)?;

    if let Some(package) = manifest.package() {
        if let Some(metadata) = package.get("metadata") {
            if let Some(table) = metadata.as_table() {
                if table.is_empty() {
                    println!("No metadata defined");
                } else {
                    println!("Metadata:");
                    print_table(table, 1);
                }
                return Ok(());
            }
        }
    }
    println!("No metadata defined");
    Ok(())
}

fn print_table(table: &toml_edit::Table, indent: usize) {
    let indent_str = "  ".repeat(indent);
    for (key, value) in table.iter() {
        if let Some(subtable) = value.as_table() {
            println!("{}[{}]", indent_str, key);
            print_table(subtable, indent + 1);
        } else if let Some(s) = value.as_str() {
            println!("{}{} = \"{}\"", indent_str, key, s);
        } else if let Some(arr) = value.as_array() {
            println!("{}{} = {:?}", indent_str, key, arr);
        } else {
            println!("{}{} = {}", indent_str, key, value);
        }
    }
}

fn clear(path: &Path, dry_run: bool, quiet: bool) -> Result<()> {
    let mut manifest = Manifest::load(path)?;
    let package = manifest.package_mut()?;

    if package.contains_key("metadata") {
        if !dry_run {
            package.remove("metadata");
            manifest.save()?;
        }
        print_success("All metadata cleared", quiet);
    } else {
        print_info("No metadata to clear", quiet);
    }

    Ok(())
}

fn json_to_toml_item(value: &serde_json::Value) -> Result<Item> {
    match value {
        serde_json::Value::String(s) => Ok(toml_edit::value(s.clone())),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(toml_edit::value(i))
            } else if let Some(f) = n.as_f64() {
                Ok(toml_edit::value(f))
            } else {
                anyhow::bail!("Unsupported number type")
            }
        }
        serde_json::Value::Bool(b) => Ok(toml_edit::value(*b)),
        serde_json::Value::Array(arr) => {
            let mut toml_arr = toml_edit::Array::new();
            for item in arr {
                if let Some(s) = item.as_str() {
                    toml_arr.push(s);
                } else if let Some(i) = item.as_i64() {
                    toml_arr.push(i);
                } else if let Some(f) = item.as_f64() {
                    toml_arr.push(f);
                } else if let Some(b) = item.as_bool() {
                    toml_arr.push(b);
                }
            }
            Ok(Item::Value(toml_edit::Value::Array(toml_arr)))
        }
        serde_json::Value::Object(obj) => {
            let mut toml_table = Table::new();
            for (k, v) in obj {
                toml_table.insert(k, json_to_toml_item(v)?);
            }
            Ok(Item::Table(toml_table))
        }
        serde_json::Value::Null => Ok(toml_edit::value("")),
    }
}
