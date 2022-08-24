use std::ffi::{OsString, OsStr};
use std::io::{stdout, Write, self};
use std::os::unix::prelude::OsStrExt;


fn write_line(s: &[u8]) -> io::Result<()> {
    stdout().write_all(s)?;
    stdout().write_all("\n".as_bytes())
}

pub fn yes_main(args: Vec<OsString>) -> i32 {
    let mut yes_val = OsString::from("y");
    if args.len() > 0 {
        let sep = OsStr::new(" ");
        yes_val = args.join(sep);
    }

    loop {
        write_line(yes_val.as_bytes()).expect("Yes ran into an issue.");
    }
}