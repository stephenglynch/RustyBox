use std::ffi::{OsString, OsStr}; 
use std::fs;
use std::process::ExitCode;
use std::error::Error;
use std::path::Path;
use pico_args::{self, Arguments};
use crate::io_util::write_line_err;
use log::*;

// TODO: Does not handle -f
// TODO: Does not handle -i
// TODO: Does not handle -R
// TODO: Does not handle -r


fn rm_node(path: &Path, recurse: bool, force: bool) -> Result<(), Box<dyn Error>> {
    if recurse {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let ftype = entry.file_type()?;
            if ftype.is_dir() {
                
            }
        }
    }
    Ok(())
}

fn files_from_pargs(pargs: &mut Arguments) -> Vec<OsString> {
    let mut files = vec![];
    loop {
        let success = pargs.opt_free_from_os_str(|file_path| {
            Ok::<OsString,u8>(file_path.to_owned())
        }).unwrap();

        match success {
            None => break,
            Some(fl) => files.push(fl),
            _ => ()
        };
    }

    files
}

pub fn rm_main(args: Vec<OsString>) -> Result<ExitCode, Box<dyn Error>> {
    let mut pargs = pico_args::Arguments::from_vec(args);

    stderrlog::new().module(module_path!()).init().unwrap();

    let files = files_from_pargs(&mut pargs);
    if files.len() == 0 {
        error!("missing operand");
    }
    else {

    }

    // Handle more than one directory
    loop {
        let success = pargs.opt_free_from_os_str(|file_path| {
            fs::remove_dir(file_path)
        });

        match success {
            Ok(None) => break,
            _ => ()
        };
    }

    Ok(ExitCode::SUCCESS)
}