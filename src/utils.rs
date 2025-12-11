// src/utils.rs
use anyhow::{Context, Result};
use colored::*;

pub fn parse_key_val(s: &str) -> Result<(String, String)> {
    let pos = s
        .find('=')
        .context("Invalid KEY=VALUE format: no `=` found")?;
    Ok((s[..pos].to_string(), s[pos + 1..].to_string()))
}

pub fn print_success(msg: &str, quiet: bool) {
    if !quiet {
        println!("{} {}", "✓".green().bold(), msg);
    }
}

pub fn print_info(msg: &str, quiet: bool) {
    if !quiet {
        println!("{}", msg);
    }
}

pub fn print_warning(msg: &str) {
    eprintln!("{} {}", "⚠".yellow().bold(), msg);
}

pub fn print_error(msg: &str) {
    eprintln!("{} {}", "✗".red().bold(), msg);
}
