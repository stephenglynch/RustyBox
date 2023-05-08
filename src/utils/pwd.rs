use std::error::Error;
use std::process::ExitCode;
use std::ffi::OsString;
use std::os::unix::prelude::OsStrExt;
use std::env::current_dir;

use crate::io_util::write_line;

// TODO: Does not handle -L
// TODO: Does not handle -P

pub fn pwd_main(_cmd_name: &str, args: Vec<OsString>) -> Result<ExitCode, Box<dyn Error>> {
    write_line(current_dir().expect("Fatal: could not current directory").as_os_str().as_bytes())?;
    Ok(ExitCode::SUCCESS)
}