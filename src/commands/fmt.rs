// src/commands/fmt.rs
use crate::manifest::Manifest;
use crate::utils::{print_error, print_success};
use anyhow::Result;
use std::fs;
use std::path::Path;

pub fn handle(path: &Path, check: bool, dry_run: bool) -> Result<()> {
    let manifest = Manifest::load(path)?;
    let formatted = manifest.doc.to_string();

    if check {
        // Check mode - verify if formatted
        let original = fs::read_to_string(path)?;
        if original == formatted {
            print_success("Cargo.toml is already formatted", false);
            Ok(())
        } else {
            print_error("Cargo.toml is not formatted");
            println!("\nRun 'cargoe fmt' to format the file");
            anyhow::bail!("Formatting check failed");
        }
    } else {
        // Format mode
        if !dry_run {
            fs::write(path, formatted)?;
            print_success("Cargo.toml formatted successfully", false);
        } else {
            println!("Would format Cargo.toml");
            println!("(dry run - no changes made)");
        }
        Ok(())
    }
}