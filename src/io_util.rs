use std::io::{stdout, Write, self};

pub fn write(s: &[u8]) -> io::Result<()> {
    stdout().write_all(s)
} 

pub fn write_line(s: &[u8]) -> io::Result<()> {
    write(s)?;
    stdout().write_all("\n".as_bytes())
}