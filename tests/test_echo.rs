use assert_cmd::prelude::*;

mod common;

#[test]
fn echo_basic() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = common::get_cmd("echo");
    cmd.arg("foobar");
    cmd.assert()
        .success()
        .stdout("foobar\n");

    Ok(())
}

#[test]
fn echo_empty() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = common::get_cmd("echo");
    cmd.assert()
        .success()
        .stdout("\n");

    Ok(())
}