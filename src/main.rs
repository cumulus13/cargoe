// src/main.rs
#![allow(clippy::unnecessary_map_or)]
#![allow(clippy::collapsible_if)]

use anyhow::{Context, Result};
use clap::{Parser, Subcommand, ArgAction};
use clap_version_flag::colorful_version;
use std::path::PathBuf;

mod commands;
mod manifest;
mod utils;

use commands::*;

#[derive(Parser)]
#[command(
    name = "cargoe",
    // version,
    about = "Advanced Cargo.toml management tool by Hadi Cahyadi <cumulus13@gmail.com>",
    long_about = "A powerful CLI tool for managing Cargo.toml fields that cargo doesn't handle directly.\nSupports exclude/include patterns, keywords, categories, badges, and more.",
    disable_version_flag = true
)]
struct Cli {
    #[arg(short = 'V', long = "version", action = ArgAction::SetTrue)]  // HAPUS short = 'v'
    version: bool,

    #[command(subcommand)]
    command: Commands,

    /// Path to Cargo.toml file
    #[arg(short, long, global = true, default_value = "Cargo.toml")]
    manifest_path: PathBuf,

    /// Dry run - show changes without applying them
    #[arg(long, global = true)]
    dry_run: bool,

    /// Suppress output except errors
    #[arg(short, long, global = true)]
    quiet: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Manage exclude patterns
    #[command(subcommand)]
    Exclude(ExcludeCommands),

    /// Manage include patterns
    #[command(subcommand)]
    Include(IncludeCommands),

    /// Manage keywords
    #[command(subcommand)]
    Keywords(KeywordsCommands),

    /// Manage categories
    #[command(subcommand)]
    Categories(CategoriesCommands),

    /// Manage badges
    #[command(subcommand)]
    Badges(BadgesCommands),

    /// Manage metadata
    #[command(subcommand)]
    Metadata(MetadataCommands),

    /// Set package fields (repository, homepage, documentation, etc.)
    Set {
        /// Field to set (repository, homepage, documentation, readme, license-file)
        field: String,
        /// Value to set
        value: String,
    },

    /// Get package field value
    Get {
        /// Field to get
        field: String,
    },

    /// Validate Cargo.toml
    Validate {
        /// Check against crates.io requirements
        #[arg(long)]
        strict: bool,
    },

    /// Format Cargo.toml
    Fmt {
        /// Check if formatted without modifying
        #[arg(long)]
        check: bool,
    },

    /// Show package information summary
    Info,

    /// Initialize recommended fields for publishing
    Init {
        /// Skip interactive prompts
        #[arg(short, long)]
        yes: bool,
    },
}

#[derive(Subcommand)]
enum ExcludeCommands {
    /// Add exclude pattern(s)
    Add { patterns: Vec<String> },
    /// Remove exclude pattern(s)
    Remove { patterns: Vec<String> },
    /// List exclude patterns
    List,
    /// Clear all exclude patterns
    Clear,
}

#[derive(Subcommand)]
enum IncludeCommands {
    /// Add include pattern(s)
    Add { patterns: Vec<String> },
    /// Remove include pattern(s)
    Remove { patterns: Vec<String> },
    /// List include patterns
    List,
    /// Clear all include patterns
    Clear,
}

#[derive(Subcommand)]
enum KeywordsCommands {
    /// Add keyword(s) (max 5 for crates.io)
    Add { keywords: Vec<String> },
    /// Remove keyword(s)
    Remove { keywords: Vec<String> },
    /// List keywords
    List,
    /// Clear all keywords
    Clear,
}

#[derive(Subcommand)]
enum CategoriesCommands {
    /// Add category/categories (max 5 for crates.io)
    Add { categories: Vec<String> },
    /// Remove category/categories
    Remove { categories: Vec<String> },
    /// List categories
    List,
    /// Clear all categories
    Clear,
    /// Show valid crates.io categories
    Valid,
}

#[derive(Subcommand)]
enum BadgesCommands {
    /// Add a badge
    Add {
        /// Badge type (e.g., maintenance, github-actions)
        badge_type: String,
        /// Key-value pairs for badge configuration
        #[arg(value_parser = utils::parse_key_val)]
        attributes: Vec<(String, String)>,
    },
    /// Remove a badge
    Remove {
        /// Badge type to remove
        badge_type: String,
    },
    /// List all badges
    List,
    /// Clear all badges
    Clear,
}

#[derive(Subcommand)]
enum MetadataCommands {
    /// Add custom metadata
    Add {
        /// Key path (dot-separated for nested keys)
        key: String,
        /// Value to set
        value: String,
        /// Treat value as JSON
        #[arg(long)]
        json: bool,
    },
    /// Remove metadata
    Remove {
        /// Key path to remove
        key: String,
    },
    /// List all metadata
    List,
    /// Clear all metadata
    Clear,
}

// fn print_version() {
//     println!(
//         "cargoe v{} by Hadi Cahyadi <cumulu13@gmail.com>",
//         env!("CARGO_PKG_VERSION")
//     );
// }

fn main() -> Result<()> {
    // Check for version flag BEFORE parsing
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 2 && (args[1] == "-V" || args[1] == "--version") {
        let version = colorful_version!();
        version.print_and_exit();
    }

    let cli = Cli::parse();

    // if cli.version {
    //     print_version();
    //     return Ok(());
    // }

    // if cli.version {
    //     let version = colorful_version!(); 
    //     version.print();
    //     return Ok(());
    // }

    if !cli.manifest_path.exists() {
        anyhow::bail!("Cargo.toml not found at: {}", cli.manifest_path.display());
    }

    // Handle case where no subcommand is provided
    // let command = cli.command.ok_or_else(|| {
    //     anyhow::anyhow!("No subcommand provided. Use --help to see available commands.")
    // })?;

    let result = match cli.command {
        Commands::Exclude(cmd) => exclude::handle(&cli.manifest_path, cmd, cli.dry_run, cli.quiet),
        Commands::Include(cmd) => include::handle(&cli.manifest_path, cmd, cli.dry_run, cli.quiet),
        Commands::Keywords(cmd) => {
            keywords::handle(&cli.manifest_path, cmd, cli.dry_run, cli.quiet)
        }
        Commands::Categories(cmd) => {
            categories::handle(&cli.manifest_path, cmd, cli.dry_run, cli.quiet)
        }
        Commands::Badges(cmd) => badges::handle(&cli.manifest_path, cmd, cli.dry_run, cli.quiet),
        Commands::Metadata(cmd) => {
            metadata::handle(&cli.manifest_path, cmd, cli.dry_run, cli.quiet)
        }
        Commands::Set { field, value } => {
            set::handle(&cli.manifest_path, &field, &value, cli.dry_run, cli.quiet)
        }
        Commands::Get { field } => get::handle(&cli.manifest_path, &field),
        Commands::Validate { strict } => validate::handle(&cli.manifest_path, strict),
        Commands::Fmt { check } => fmt::handle(&cli.manifest_path, check, cli.dry_run),
        Commands::Info => info::handle(&cli.manifest_path),
        Commands::Init { yes } => init::handle(&cli.manifest_path, yes, cli.dry_run),
    };

    result.context("Failed to execute command")
    
}
