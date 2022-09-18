use assert_cmd::prelude::*;
use std::process::Command;

pub fn get_cmd(rustbox_cmd: &str) -> Command {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg(rustbox_cmd);
    return cmd;
}