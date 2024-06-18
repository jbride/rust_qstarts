use assert_cmd::Command;

#[test]
fn testfunc_os_program() {
    let mut cmd = std::process::Command::new("ls");
    let res = cmd.output();
    assert!(res.is_ok());
}

#[test]
fn testfunc_project_program() {
    let mut cmd = assert_cmd::Command::cargo_bin("hello").unwrap();
    cmd.assert().success();
}

#[test]
fn testfunc_project_bin_success() {
    let mut cmd = assert_cmd::Command::cargo_bin("true").unwrap();
    cmd.assert().success();
}

#[test]
fn testfunc_project_bin_failure() {
    let mut cmd = assert_cmd::Command::cargo_bin("false").unwrap();
    cmd.assert().failure();
}

#[test]
fn testfunc_project_program_string_eval() {
    let mut cmd = assert_cmd::Command::cargo_bin("hello").unwrap();

    // Call Command:output to execute the hello command
    // Use Result::expect to get the output of the command or die w/ the message "fail"
    let output = cmd.output().expect("fail");

    assert!(output.status.success());

    // Convert the output to UTF-8
    let stdout = String::from_utf8(output.stdout).expect("invalid UTF-8");

    assert_eq!(stdout, "Hello, world!\n");

}
