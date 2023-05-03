use std::error::Error;
use std::process::ExitCode;
use std::ffi::OsString;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
use crate::io_util::write;

// TODO: Does not handle -u (may just need to ignore -u as this seems to be standard)

fn path_opt_to_reader(path: Option<OsString>) -> Result<Box<dyn BufRead>, Box<dyn Error>> {
    match path {
        Some(p) => {
            if p == OsString::from("-") {
                Ok(Box::new(BufReader::new(stdin())))
            } else {
                Ok(Box::new(BufReader::new(File::open(&p)?)))
            }
        },
        None => Ok(Box::new(BufReader::new(stdin())))
    }
}

pub fn cat_main(args: Vec<OsString>) -> Result<ExitCode, Box<dyn Error>> {
    let mut pargs = pico_args::Arguments::from_vec(args);

    let path = pargs.opt_free_from_os_str(|file_path| {
        Ok::<OsString,u8>(file_path.to_owned())
    }).unwrap();

    let mut reader = path_opt_to_reader(path)?;

    loop {
        let buf = reader.fill_buf()?;
        write(buf)?;

        // Have we reached end of file?
        let n = buf.len();
        if n == 0 {
            return Ok(ExitCode::SUCCESS)
        }
        reader.consume(n);
    }
}