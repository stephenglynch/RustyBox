use std::ffi::{OsString, OsStr};
use std::os::unix::prelude::OsStrExt;
use std::path::Path;
use std::process::ExitCode;
use std::error::Error;
use crate::io_util::write_line;

fn strip_suffix<'a, 'b>(s: &'a OsStr, suffix: &'b OsStr) -> &'a OsStr {
    let s = s.as_bytes();
    let suffix = suffix.as_bytes();
    match s.strip_suffix(suffix) {
        None => OsStr::from_bytes(s),
        Some(x) => OsStr::from_bytes(x)
    }
}

pub fn basename_main(args: Vec<OsString>) -> Result<ExitCode, Box<dyn Error>> {
    if args.len() == 0 {
        println!("basename: missing operand");
        return Ok(ExitCode::FAILURE);
    }

    if args.len() > 2 {
        println!("basename: too many operands");
        return Ok(ExitCode::FAILURE);
    }

    let empty = OsString::from("");
    let path = Path::new(&args[0]);
    let suffix = args.get(1).unwrap_or(&empty);

    // Get filename
    let file_name = match path.file_name() {
        Some(fname) => fname,
        None => OsStr::new("..")
    };

    // Remove suffix
    let basename = strip_suffix(file_name, suffix);

    write_line(basename.as_bytes())?;
    Ok(ExitCode::SUCCESS)
}