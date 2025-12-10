use crate::manifest::Manifest;
use crate::utils::{print_info, print_success};
use anyhow::Result;
use dialoguer::{Confirm, Input};
use std::path::Path;

pub fn handle(path: &Path, yes: bool, dry_run: bool) -> Result<()> {
    let mut manifest = Manifest::load(path)?;

    println!("Initializing recommended fields for publishing...\n");

    let package = manifest.package_mut()?;

    // Repository
    if !package.contains_key("repository") {
        if yes
            || Confirm::new()
                .with_prompt("Add repository field?")
                .default(true)
                .interact()?
        {
            let repo: String = Input::new()
                .with_prompt("Repository URL")
                .with_initial_text("https://github.com/user/repo")
                .interact_text()?;

            if !dry_run {
                package.insert("repository", toml_edit::value(repo.clone()));
            }
            print_info(&format!("+ repository: {}", repo), false);
        }
    }

    // Homepage
    if !package.contains_key("homepage") {
        if yes
            || Confirm::new()
                .with_prompt("Add homepage field?")
                .default(false)
                .interact()?
        {
            let homepage: String = Input::new()
                .with_prompt("Homepage URL")
                .interact_text()?;

            if !dry_run {
                package.insert("homepage", toml_edit::value(homepage.clone()));
            }
            print_info(&format!("+ homepage: {}", homepage), false);
        }
    }

    // Documentation
    if !package.contains_key("documentation") {
        if yes
            || Confirm::new()
                .with_prompt("Add documentation field? (defaults to docs.rs)")
                .default(false)
                .interact()?
        {
            let docs: String = Input::new()
                .with_prompt("Documentation URL")
                .interact_text()?;

            if !dry_run {
                package.insert("documentation", toml_edit::value(docs.clone()));
            }
            print_info(&format!("+ documentation: {}", docs), false);
        }
    }

    // README
    if !package.contains_key("readme") {
        if yes
            || Confirm::new()
                .with_prompt("Add readme field?")
                .default(true)
                .interact()?
        {
            let readme: String = Input::new()
                .with_prompt("README file path")
                .with_initial_text("README.md")
                .interact_text()?;

            if !dry_run {
                package.insert("readme", toml_edit::value(readme.clone()));
            }
            print_info(&format!("+ readme: {}", readme), false);
        }
    }

    if !dry_run {
        manifest.save()?;
        print_success("Initialization complete", false);
    } else {
        print_info("(dry run - no changes made)", false);
    }

    println!("\nNext steps:");
    println!("  • Add keywords: cargoe keywords add <keyword1> <keyword2>");
    println!("  • Add categories: cargoe categories add <category>");
    println!("  • Validate: cargoe validate --strict");

    Ok(())
}