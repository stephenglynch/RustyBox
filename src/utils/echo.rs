use std::ffi::{OsString, OsStr}; 
use std::os::unix::prelude::OsStrExt;
use std::process::ExitCode;
use std::error::Error;
use crate::io_util::write_line;

pub fn echo_main(args: Vec<OsString>) -> Result<ExitCode, Box<dyn Error>> {

    let sep = OsStr::new(" ");
    write_line(args.join(sep).as_bytes())?;
    Ok(ExitCode::SUCCESS)
}