use assert_cmd::Command;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_help_if_no_args() {
    let mut cmd = Command::cargo_bin("viua").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicates::str::contains("Usage:"));
}

#[test]
fn test_missing_file() {
    let mut cmd = Command::cargo_bin("viua").unwrap();
    cmd.arg("non_existent_file.png");
    cmd.assert()
        .success()
        .stderr(predicates::str::contains("warning: file not found"));
}

#[test]
fn test_convert_camel_ascii() {
    let mut cmd = Command::cargo_bin("viua").unwrap();
    cmd.arg("img/camel.png")
        .arg("-M")
        .arg("ascii")
        .arg("-w")
        .arg("40");
    cmd.assert().success();
}

#[test]
fn test_recursive_directory() {
    let dir = tempdir().unwrap();
    let img_path = dir.path().join("test_img.png");
    fs::copy("img/camel.png", &img_path).unwrap();

    let mut cmd = Command::cargo_bin("viua").unwrap();
    cmd.arg(dir.path())
        .arg("-r")
        .arg("-M")
        .arg("ascii")
        .arg("-w")
        .arg("20");
    cmd.assert().success();
}

#[test]
fn test_height_resolution() {
    let mut cmd = Command::cargo_bin("viua").unwrap();
    cmd.arg("img/camel.png")
        .arg("-M")
        .arg("ascii")
        .arg("-H")
        .arg("10");
    cmd.assert().success();
}
