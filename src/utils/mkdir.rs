use std::ffi::{OsString, OsStr}; 
use std::fs;
use std::io;
use std::process::ExitCode;
use std::error::Error;
use pico_args;

// TODO: Does not handle -m

// Helper function
pub fn get_os_string(dir_name: &OsStr) -> Result<OsString, u8> {
    Ok(OsString::from(dir_name))
}

pub fn mkdir_main(args: Vec<OsString>) -> Result<ExitCode, Box<dyn Error>> {
    let mut pargs = pico_args::Arguments::from_vec(args);
    let mut dir_names = vec![];

    let nested = pargs.contains("-p");

    dir_names.push(pargs.free_from_os_str::<OsString, u8>(get_os_string)?);

    // Handle more than one directory    
    loop {
        let success = pargs.opt_free_from_os_str::<OsString, u8>(get_os_string)?;
        match success {
            None => break,
            Some(dir_name) => dir_names.push(dir_name)
        };
    }

    for dir_name in dir_names {
        if nested {
            fs::create_dir_all(dir_name)?;
        } else {
            fs::create_dir(dir_name)?;
        }
    }

    Ok(ExitCode::SUCCESS)
}