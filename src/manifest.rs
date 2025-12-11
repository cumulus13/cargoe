// src/manifest.rs
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;
use toml_edit::DocumentMut;

pub struct Manifest {
    pub doc: DocumentMut,
    pub path: std::path::PathBuf,
}

impl Manifest {
    pub fn load(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read {}", path.display()))?;

        let doc = content
            .parse::<DocumentMut>()
            .context("Failed to parse Cargo.toml")?;

        Ok(Self {
            doc,
            path: path.to_path_buf(),
        })
    }

    pub fn save(&self) -> Result<()> {
        fs::write(&self.path, self.doc.to_string())
            .with_context(|| format!("Failed to write {}", self.path.display()))
    }

    pub fn package_mut(&mut self) -> Result<&mut toml_edit::Table> {
        self.doc["package"]
            .or_insert(toml_edit::table())
            .as_table_mut()
            .context("package is not a table")
    }

    pub fn package(&self) -> Option<&toml_edit::Table> {
        self.doc.get("package")?.as_table()
    }

    #[allow(dead_code)]
    pub fn get_package_name(&self) -> Option<String> {
        self.package()?.get("name")?.as_str().map(|s| s.to_string())
    }

    #[allow(dead_code)]
    pub fn get_package_version(&self) -> Option<String> {
        self.package()?
            .get("version")?
            .as_str()
            .map(|s| s.to_string())
    }
}
