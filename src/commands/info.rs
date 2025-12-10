// src/commands/info.rs
use crate::manifest::Manifest;
use anyhow::Result;
use colored::*;
use std::path::Path;

pub fn handle(path: &Path) -> Result<()> {
    let manifest = Manifest::load(path)?;

    if let Some(package) = manifest.package() {
        println!("{}", "Package Information".bold().underline());
        println!();

        print_field("Name", package.get("name"));
        print_field("Version", package.get("version"));
        print_field("Edition", package.get("edition"));
        print_field("Rust Version", package.get("rust-version"));
        println!();

        print_field("Description", package.get("description"));
        print_field("License", package.get("license"));
        print_array_field_inline("Authors", package.get("authors"));
        println!();

        print_field("Homepage", package.get("homepage"));
        print_field("Repository", package.get("repository"));
        print_field("Documentation", package.get("documentation"));
        println!();

        print_array_field("Keywords", package.get("keywords"));
        print_array_field("Categories", package.get("categories"));
        print_array_field("Exclude", package.get("exclude"));
        print_array_field("Include", package.get("include"));
    } else {
        println!("{}", "No [package] section found".red());
    }

    Ok(())
}

fn print_field(name: &str, value: Option<&toml_edit::Item>) {
    if let Some(val) = value {
        if let Some(s) = val.as_str() {
            println!("{}: {}", name.cyan().bold(), s);
        }
    }
}

fn print_array_field_inline(name: &str, value: Option<&toml_edit::Item>) {
    if let Some(val) = value {
        if let Some(arr) = val.as_array() {
            if !arr.is_empty() {
                let items: Vec<String> = arr
                    .iter()
                    .filter_map(|item| item.as_str().map(|s| s.to_string()))
                    .collect();
                if !items.is_empty() {
                    println!("{}: {}", name.cyan().bold(), items.join(", "));
                }
            }
        }
    }
}

fn print_array_field(name: &str, value: Option<&toml_edit::Item>) {
    if let Some(val) = value {
        if let Some(arr) = val.as_array() {
            if !arr.is_empty() {
                println!("{} ({}):", name.cyan().bold(), arr.len());
                for item in arr.iter() {
                    if let Some(s) = item.as_str() {
                        println!("  • {}", s);
                    }
                }
            }
        }
    }
}