use assert_cmd::prelude::*;

mod common;


#[test]
fn false_failure() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = common::get_cmd("true");

    cmd.assert()
        .success();

    Ok(())
}