use std::process::Command;
use assert_cmd::Command as AssertCommand;

#[test]
fn runs() {
    let mut cmd = AssertCommand::cargo_bin("hello").unwrap();
    cmd.assert().success().stdout("Hello, world!!!\n");
}

#[test]
fn true_ok() {
    let mut cmd = AssertCommand::cargo_bin("true").unwrap();
    cmd.assert().success();
}

#[test]
fn false_not_ok() {
    let mut cmd = AssertCommand::cargo_bin("false").unwrap();
    cmd.assert().failure();
}