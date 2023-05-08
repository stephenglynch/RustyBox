use std::error::Error;
use std::process::ExitCode;
use std::ffi::{OsString, OsStr};
use std::os::unix::prelude::OsStrExt;

use crate::io_util::write_line;


pub fn yes_main(_cmd_name: &str, args: Vec<OsString>) -> Result<ExitCode, Box<dyn Error>> {
    let mut yes_val = OsString::from("y");
    if args.len() > 0 {
        let sep = OsStr::new(" ");
        yes_val = args.join(sep);
    }

    loop {
        write_line(yes_val.as_bytes())?;
    }
}