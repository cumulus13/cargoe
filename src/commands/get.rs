// src/commands/get.rs
use crate::manifest::Manifest;
use anyhow::{Context, Result};
use std::path::Path;

pub fn handle(path: &Path, field: &str) -> Result<()> {
    let manifest = Manifest::load(path)?;
    let package = manifest
        .package()
        .context("No [package] section found")?;

    if let Some(value) = package.get(field) {
        // Pretty print the value
        if let Some(s) = value.as_str() {
            println!("{}", s);
        } else if let Some(arr) = value.as_array() {
            for item in arr.iter() {
                if let Some(s) = item.as_str() {
                    println!("{}", s);
                }
            }
        } else if let Some(table) = value.as_table() {
            println!("{:#?}", table);
        } else {
            println!("{}", value);
        }
    } else {
        anyhow::bail!("Field '{}' not found in [package]", field);
    }

    Ok(())
}