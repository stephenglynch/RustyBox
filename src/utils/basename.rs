use std::ffi::{OsString, OsStr};
use std::os::unix::prelude::OsStrExt;
use std::path::Path;
use std::process::ExitCode;
use std::error::Error;
use crate::io_util::write_line;

fn strip_suffix<'a, 'b>(s: &'a OsStr, suffix: &'b OsStr) -> &'a OsStr {
    let s = s.as_bytes();
    let suffix = suffix.as_bytes();

    // If suffix equals string
    if s == suffix {
        return OsStr::from_bytes(s);
    }

    match s.strip_suffix(suffix) {
        None => OsStr::from_bytes(s),
        Some(x) => OsStr::from_bytes(x)
    }
}

fn get_basename<'a>(path_str: &'a OsStr, suffix: &OsStr) -> &'a OsStr {

    // Check null string
    if path_str == "" {
        return OsStr::new(".");
    }

    let path = Path::new(path_str);

    // Get filename
    let file_name = match path.file_name() {
        Some(fname) => fname,
        None => OsStr::new("/")
    };

    // Remove suffix
    strip_suffix(file_name, suffix)
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
    let suffix = args.get(1).unwrap_or(&empty);
    let basename = get_basename(&args[0], suffix);

    write_line(basename.as_bytes())?;
    Ok(ExitCode::SUCCESS)
}