use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_exclude_add() {
    let temp = TempDir::new().unwrap();
    let manifest_path = temp.path().join("Cargo.toml");

    fs::write(
        &manifest_path,
        r#"[package]
name = "test"
version = "0.1.0"
"#,
    )
    .unwrap();

    Command::cargo_bin("cargoe")
        .unwrap()
        .args(&["-m", manifest_path.to_str().unwrap(), "exclude", "add", "*.log"])
        .assert()
        .success()
        .stdout(predicate::str::contains("+ *.log"));

    let content = fs::read_to_string(&manifest_path).unwrap();
    assert!(content.contains("exclude"));
    assert!(content.contains("*.log"));
}

#[test]
fn test_keywords_limit() {
    let temp = TempDir::new().unwrap();
    let manifest_path = temp.path().join("Cargo.toml");

    fs::write(
        &manifest_path,
        r#"[package]
name = "test"
version = "0.1.0"
"#,
    )
    .unwrap();

    Command::cargo_bin("cargoe")
        .unwrap()
        .args(&[
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
fn test_validate_missing_fields() {
    let temp = TempDir::new().unwrap();
    let manifest_path = temp.path().join("Cargo.toml");

    fs::write(
        &manifest_path,
        r#"[package]
name = "test"
version = "0.1.0"
"#,
    )
    .unwrap();

    Command::cargo_bin("cargoe")
        .unwrap()
        .args(&["-m", manifest_path.to_str().unwrap(), "validate", "--strict"])
        .assert()
        .failure()
        .stdout(predicate::str::contains("Missing 'description'"));
}