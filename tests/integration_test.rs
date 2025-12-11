// tests/integration_test.rs
#![allow(deprecated)]
#![allow(clippy::needless_borrows_for_generic_args)]

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

fn create_test_manifest(temp: &TempDir) -> std::path::PathBuf {
    let manifest_path = temp.path().join("Cargo.toml");
    fs::write(
        &manifest_path,
        r#"[package]
name = "test-package"
version = "0.1.0"
edition = "2021"
"#,
    )
    .unwrap();
    manifest_path
}

#[test]
fn test_exclude_add() {
    let temp = TempDir::new().unwrap();
    let manifest_path = create_test_manifest(&temp);

    Command::cargo_bin("cargoe")
        .unwrap()
        .args([
            "-m",
            manifest_path.to_str().unwrap(),
            "exclude",
            "add",
            "*.log",
            "target/",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("+ *.log"))
        .stdout(predicate::str::contains("+ target/"));

    let content = fs::read_to_string(&manifest_path).unwrap();
    assert!(content.contains("exclude"));
    assert!(content.contains("*.log"));
    assert!(content.contains("target/"));
}

#[test]
fn test_exclude_list() {
    let temp = TempDir::new().unwrap();
    let manifest_path = create_test_manifest(&temp);

    // Add patterns first
    Command::cargo_bin("cargoe")
        .unwrap()
        .args([
            "-m",
            manifest_path.to_str().unwrap(),
            "exclude",
            "add",
            "*.log",
        ])
        .assert()
        .success();

    // List them
    Command::cargo_bin("cargoe")
        .unwrap()
        .args(["-m", manifest_path.to_str().unwrap(), "exclude", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("*.log"));
}

#[test]
fn test_exclude_remove() {
    let temp = TempDir::new().unwrap();
    let manifest_path = create_test_manifest(&temp);

    // Add then remove
    Command::cargo_bin("cargoe")
        .unwrap()
        .args([
            "-m",
            manifest_path.to_str().unwrap(),
            "exclude",
            "add",
            "*.log",
        ])
        .assert()
        .success();

    Command::cargo_bin("cargoe")
        .unwrap()
        .args([
            "-m",
            manifest_path.to_str().unwrap(),
            "exclude",
            "remove",
            "*.log",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("- *.log"));
}

#[test]
fn test_keywords_add() {
    let temp = TempDir::new().unwrap();
    let manifest_path = create_test_manifest(&temp);

    Command::cargo_bin("cargoe")
        .unwrap()
        .args([
            "-m",
            manifest_path.to_str().unwrap(),
            "keywords",
            "add",
            "cli",
            "cargo",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("+ cli"))
        .stdout(predicate::str::contains("+ cargo"));

    let content = fs::read_to_string(&manifest_path).unwrap();
    assert!(content.contains("keywords"));
    assert!(content.contains("cli"));
}

#[test]
fn test_keywords_limit() {
    let temp = TempDir::new().unwrap();
    let manifest_path = create_test_manifest(&temp);

    Command::cargo_bin("cargoe")
        .unwrap()
        .args([
            "-m",
            manifest_path.to_str().unwrap(),
            "keywords",
            "add",
            "k1",
            "k2",
            "k3",
            "k4",
            "k5",
            "k6",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Maximum 5 keywords"));
}

#[test]
fn test_keywords_length_validation() {
    let temp = TempDir::new().unwrap();
    let manifest_path = create_test_manifest(&temp);

    let long_keyword = "a".repeat(21);
    Command::cargo_bin("cargoe")
        .unwrap()
        .args([
            "-m",
            manifest_path.to_str().unwrap(),
            "keywords",
            "add",
            &long_keyword,
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("exceeds 20 characters"));
}

#[test]
fn test_categories_add() {
    let temp = TempDir::new().unwrap();
    let manifest_path = create_test_manifest(&temp);

    Command::cargo_bin("cargoe")
        .unwrap()
        .args([
            "-m",
            manifest_path.to_str().unwrap(),
            "categories",
            "add",
            "command-line-utilities",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("+ command-line-utilities"));
}

#[test]
fn test_set_field() {
    let temp = TempDir::new().unwrap();
    let manifest_path = create_test_manifest(&temp);

    Command::cargo_bin("cargoe")
        .unwrap()
        .args([
            "-m",
            manifest_path.to_str().unwrap(),
            "set",
            "repository",
            "https://github.com/user/repo",
        ])
        .assert()
        .success();

    let content = fs::read_to_string(&manifest_path).unwrap();
    assert!(content.contains("repository"));
    assert!(content.contains("https://github.com/user/repo"));
}

#[test]
fn test_get_field() {
    let temp = TempDir::new().unwrap();
    let manifest_path = create_test_manifest(&temp);

    Command::cargo_bin("cargoe")
        .unwrap()
        .args(["-m", manifest_path.to_str().unwrap(), "get", "name"])
        .assert()
        .success()
        .stdout(predicate::str::contains("test-package"));
}

#[test]
fn test_validate_basic() {
    let temp = TempDir::new().unwrap();
    let manifest_path = create_test_manifest(&temp);

    Command::cargo_bin("cargoe")
        .unwrap()
        .args(["-m", manifest_path.to_str().unwrap(), "validate"])
        .assert()
        .success();
}

#[test]
fn test_validate_strict_missing_fields() {
    let temp = TempDir::new().unwrap();
    let manifest_path = create_test_manifest(&temp);

    Command::cargo_bin("cargoe")
        .unwrap()
        .args([
            "-m",
            manifest_path.to_str().unwrap(),
            "validate",
            "--strict",
        ])
        .assert()
        .failure()
        .stdout(predicate::str::contains("Missing 'description'"));
}

#[test]
fn test_dry_run() {
    let temp = TempDir::new().unwrap();
    let manifest_path = create_test_manifest(&temp);

    let original_content = fs::read_to_string(&manifest_path).unwrap();

    Command::cargo_bin("cargoe")
        .unwrap()
        .args([
            "-m",
            manifest_path.to_str().unwrap(),
            "--dry-run",
            "exclude",
            "add",
            "*.log",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("dry run"));

    let after_content = fs::read_to_string(&manifest_path).unwrap();
    assert_eq!(original_content, after_content);
}

#[test]
fn test_info_command() {
    let temp = TempDir::new().unwrap();
    let manifest_path = create_test_manifest(&temp);

    Command::cargo_bin("cargoe")
        .unwrap()
        .args(["-m", manifest_path.to_str().unwrap(), "info"])
        .assert()
        .success()
        .stdout(predicate::str::contains("test-package"))
        .stdout(predicate::str::contains("0.1.0"));
}

#[test]
fn test_badges_add() {
    let temp = TempDir::new().unwrap();
    let manifest_path = create_test_manifest(&temp);

    Command::cargo_bin("cargoe")
        .unwrap()
        .args([
            "-m",
            manifest_path.to_str().unwrap(),
            "badges",
            "add",
            "maintenance",
            "status=actively-developed",
        ])
        .assert()
        .success();

    let content = fs::read_to_string(&manifest_path).unwrap();
    assert!(content.contains("[badges.maintenance]"));
    assert!(content.contains("status"));
}

#[test]
fn test_metadata_add() {
    let temp = TempDir::new().unwrap();
    let manifest_path = create_test_manifest(&temp);

    Command::cargo_bin("cargoe")
        .unwrap()
        .args([
            "-m",
            manifest_path.to_str().unwrap(),
            "metadata",
            "add",
            "custom.key",
            "value",
        ])
        .assert()
        .success();

    let content = fs::read_to_string(&manifest_path).unwrap();
    assert!(content.contains("metadata"));
}

#[test]
fn test_quiet_flag() {
    let temp = TempDir::new().unwrap();
    let manifest_path = create_test_manifest(&temp);

    Command::cargo_bin("cargoe")
        .unwrap()
        .args([
            "-m",
            manifest_path.to_str().unwrap(),
            "--quiet",
            "exclude",
            "add",
            "*.log",
        ])
        .assert()
        .success()
        .stdout(predicate::str::is_empty());
}
