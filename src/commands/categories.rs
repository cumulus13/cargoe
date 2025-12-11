// src/commands/categories.rs
use crate::manifest::Manifest;
use crate::utils::{print_info, print_success, print_warning};
use crate::CategoriesCommands;
use anyhow::{Context, Result};
use once_cell::sync::Lazy;
use std::collections::HashSet;
use std::path::Path;
use toml_edit::{Array, Item, Value};

// Valid crates.io categories as of 2024
static VALID_CATEGORIES: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    HashSet::from([
        "algorithms",
        "api-bindings",
        "asynchronous",
        "authentication",
        "caching",
        "command-line-interface",
        "command-line-utilities",
        "compression",
        "concurrency",
        "config",
        "cryptography",
        "data-structures",
        "database",
        "database-implementations",
        "date-and-time",
        "development-tools",
        "development-tools::build-utils",
        "development-tools::cargo-plugins",
        "development-tools::debugging",
        "development-tools::ffi",
        "development-tools::profiling",
        "development-tools::testing",
        "email",
        "embedded",
        "emulators",
        "encoding",
        "filesystem",
        "game-development",
        "game-engines",
        "graphics",
        "gui",
        "hardware-support",
        "internationalization",
        "localization",
        "mathematics",
        "memory-management",
        "multimedia",
        "multimedia::audio",
        "multimedia::encoding",
        "multimedia::images",
        "multimedia::video",
        "network-programming",
        "no-std",
        "os",
        "os::freebsd-apis",
        "os::linux-apis",
        "os::macos-apis",
        "os::unix-apis",
        "os::windows-apis",
        "parser-implementations",
        "parsing",
        "rendering",
        "rendering::engine",
        "rendering::graphics-api",
        "rust-patterns",
        "science",
        "simulation",
        "template-engine",
        "text-editors",
        "text-processing",
        "value-formatting",
        "visualization",
        "wasm",
        "web-programming",
        "web-programming::http-client",
        "web-programming::http-server",
        "web-programming::websocket",
    ])
});

pub fn handle(path: &Path, cmd: CategoriesCommands, dry_run: bool, quiet: bool) -> Result<()> {
    match cmd {
        CategoriesCommands::Add { categories } => add(path, categories, dry_run, quiet),
        CategoriesCommands::Remove { categories } => remove(path, categories, dry_run, quiet),
        CategoriesCommands::List => list(path),
        CategoriesCommands::Clear => clear(path, dry_run, quiet),
        CategoriesCommands::Valid => show_valid_categories(),
    }
}

fn add(path: &Path, categories: Vec<String>, dry_run: bool, quiet: bool) -> Result<()> {
    let mut manifest = Manifest::load(path)?;
    let package = manifest.package_mut()?;

    let cats = package
        .entry("categories")
        .or_insert(Item::Value(Value::Array(Array::new())))
        .as_array_mut()
        .context("categories is not an array")?;

    let current_count = cats.len();

    for category in categories {
        if !VALID_CATEGORIES.contains(category.as_str()) {
            print_warning(&format!(
                "Category '{}' is not in crates.io's category list. Use 'cargoe categories valid' to see valid categories.",
                category
            ));
        }

        let exists = cats
            .iter()
            .any(|v| v.as_str().is_some_and(|s| s == category));

        if !exists {
            if current_count + cats.iter().filter(|v| v.as_str().is_some()).count() >= 5 {
                print_warning("Maximum 5 categories allowed for crates.io");
                break;
            }
            if !dry_run {
                cats.push(&category);
            }
            print_info(&format!("+ {}", category), quiet);
        } else {
            print_info(&format!("~ {} (already exists)", category), quiet);
        }
    }

    if !dry_run {
        manifest.save()?;
        print_success("Categories updated", quiet);
    } else {
        print_info("(dry run - no changes made)", false);
    }

    Ok(())
}

fn remove(path: &Path, categories: Vec<String>, dry_run: bool, quiet: bool) -> Result<()> {
    let mut manifest = Manifest::load(path)?;
    let package = manifest.package_mut()?;

    let cats = package
        .get_mut("categories")
        .context("categories field not found")?
        .as_array_mut()
        .context("categories is not an array")?;

    for category in categories {
        let mut indices = Vec::new();
        for (i, v) in cats.iter().enumerate() {
            if v.as_str().is_some_and(|s| s == category) {
                indices.push(i);
            }
        }

        if !indices.is_empty() {
            if !dry_run {
                for &i in indices.iter().rev() {
                    cats.remove(i);
                }
            }
            print_info(&format!("- {}", category), quiet);
        } else {
            print_info(&format!("? {} (not found)", category), quiet);
        }
    }

    if !dry_run {
        manifest.save()?;
        print_success("Categories updated", quiet);
    } else {
        print_info("(dry run - no changes made)", false);
    }

    Ok(())
}

fn list(path: &Path) -> Result<()> {
    let manifest = Manifest::load(path)?;

    if let Some(package) = manifest.package() {
        if let Some(categories) = package.get("categories") {
            if let Some(arr) = categories.as_array() {
                if arr.is_empty() {
                    println!("No categories");
                } else {
                    println!("Categories ({}/5):", arr.len());
                    for v in arr.iter() {
                        if let Some(s) = v.as_str() {
                            let valid = if VALID_CATEGORIES.contains(s) {
                                "✓"
                            } else {
                                "⚠"
                            };
                            println!("  {} {}", valid, s);
                        }
                    }
                }
                return Ok(());
            }
        }
    }
    println!("No categories");
    Ok(())
}

fn clear(path: &Path, dry_run: bool, quiet: bool) -> Result<()> {
    let mut manifest = Manifest::load(path)?;
    let package = manifest.package_mut()?;

    if package.contains_key("categories") {
        if !dry_run {
            package.remove("categories");
            manifest.save()?;
        }
        print_success("All categories cleared", quiet);
    } else {
        print_info("No categories to clear", quiet);
    }

    Ok(())
}

fn show_valid_categories() -> Result<()> {
    println!("Valid crates.io categories:\n");

    let mut categories: Vec<_> = VALID_CATEGORIES.iter().collect();
    categories.sort();

    let mut current_top = "";
    for cat in categories {
        if cat.contains("::") {
            let parts: Vec<&str> = cat.split("::").collect();
            if parts[0] != current_top {
                current_top = parts[0];
                println!("\n{}:", current_top);
            }
            println!("  • {}", cat);
        } else {
            if !current_top.is_empty() {
                current_top = "";
                println!();
            }
            println!("• {}", cat);
        }
    }

    Ok(())
}
