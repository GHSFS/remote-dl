use assert_cmd::Command;
use predicates::prelude::*;
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn test_version_bump_0_2_0() {
    let mut cmd = Command::cargo_bin("rdl").unwrap();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("0.2.0"));
}

#[test]
fn test_config_show_and_path() {
    let mut cmd = Command::cargo_bin("rdl").unwrap();
    cmd.args(["config", "show"])
        .assert()
        .success()
        .stdout(predicate::str::contains("rdl Configuration:"));

    let mut cmd_json = Command::cargo_bin("rdl").unwrap();
    cmd_json
        .args(["config", "show", "--json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"config_path\":"));
}

#[test]
fn test_batch_empty_file_handling() {
    let temp = NamedTempFile::new().unwrap();
    let mut cmd = Command::cargo_bin("rdl").unwrap();
    cmd.args(["batch", temp.path().to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("No valid URLs found"));
}

#[test]
fn test_batch_file_with_comments_and_blank_lines() {
    let mut temp = NamedTempFile::new().unwrap();
    writeln!(temp, "# Comment line").unwrap();
    writeln!(temp).unwrap();
    writeln!(temp, "// Another comment").unwrap();
    let mut cmd = Command::cargo_bin("rdl").unwrap();
    cmd.args(["batch", temp.path().to_str().unwrap(), "--json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"total\": 0"));
}

#[test]
fn test_inspect_help_and_arguments() {
    let mut cmd = Command::cargo_bin("rdl").unwrap();
    cmd.args(["inspect", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Pre-flight inspect remote URL"));
}

#[test]
fn test_ping_without_config_fails_cleanly() {
    let mut cmd = Command::cargo_bin("rdl").unwrap();
    // In CI or environment with no configured worker, ping errors cleanly
    let output = cmd.arg("ping").output().unwrap();
    // Either fails with config error or succeeds if worker configured
    assert!(
        !output.status.success() || String::from_utf8_lossy(&output.stdout).contains("Connected")
    );
}
