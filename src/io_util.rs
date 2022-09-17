use std::io::{stdout, Write, self};

pub fn write_line(s: &[u8]) -> io::Result<()> {
    stdout().write_all(s)?;
    stdout().write_all("\n".as_bytes())
}