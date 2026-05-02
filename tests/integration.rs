//! End-to-end CLI tests using a mock HTTP server.

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn version_flag_prints_version() {
    Command::cargo_bin("rdl")
        .unwrap()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn help_flag_lists_subcommands() {
    Command::cargo_bin("rdl")
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("get"))
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("status"))
        .stdout(predicate::str::contains("config"))
        .stdout(predicate::str::contains("auth"));
}

#[test]
fn get_without_config_fails_cleanly() {
    let temp = tempfile::tempdir().unwrap();
    Command::cargo_bin("rdl")
        .unwrap()
        .env("APPDATA", temp.path())
        .env("XDG_CONFIG_HOME", temp.path())
        .arg("get")
        .arg("https://example.com/x")
        .assert()
        .failure()
        .stderr(predicate::str::contains("error"));
}

#[test]
fn config_set_and_get_roundtrip() {
    let temp = tempfile::tempdir().unwrap();
    let appdata = temp.path();

    Command::cargo_bin("rdl")
        .unwrap()
        .env("APPDATA", appdata)
        .env("XDG_CONFIG_HOME", appdata)
        .args(["config", "set", "worker", "https://example.workers.dev"])
        .assert()
        .success();

    Command::cargo_bin("rdl")
        .unwrap()
        .env("APPDATA", appdata)
        .env("XDG_CONFIG_HOME", appdata)
        .args(["config", "get", "worker"])
        .assert()
        .success()
        .stdout(predicate::str::contains("https://example.workers.dev"));
}
