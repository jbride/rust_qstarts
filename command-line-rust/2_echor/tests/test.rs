use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

#[test]
fn dies_no_args() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
}

#[test]
fn runs_one_arg() -> () {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.arg("wer").assert().success();
}

#[test]
fn run_read_fs() {
    let resolv_file_content_result = match fs::read_to_string("/etc/resolv.conf") {
        Ok(f) => f,
        Err(e) => panic!("Error = {}", e)
    };
    assert!(!resolv_file_content_result.is_empty());
    
}
