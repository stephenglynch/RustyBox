use std::ffi::OsString; 
use std::fs;
use std::process::ExitCode;
use std::error::Error;
use std::path::Path;
use pico_args::{self, Arguments};
use log::*;

// TODO: Does not handle -i

fn rm_node(path: &Path, recurse: bool, force: bool) -> u8 {
    // Handle -f flag
    if !path.exists() && force {
        return 0;
    }

    let res = if path.is_dir() && recurse {
        fs::remove_dir_all(path)
    } else {
        fs::remove_file(path) // Equivalent to unlink
    };

    if let Err(e) = res {
        error!("{}", e);
        return 1;
    }

    return 0;
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
    let recurse = pargs.contains("-r") || pargs.contains("-R");
    let force = pargs.contains("-f");
    let files = files_from_pargs(&mut pargs);
    let mut exit_code = 0;

    if files.len() == 0 {
        error!("missing operand");
        return Ok(ExitCode::FAILURE)
    }
    else {
        for f in files {
            let path = Path::new(&f);
            exit_code |= rm_node(path, recurse, force);
        }
    }

    Ok(ExitCode::from(exit_code))
}