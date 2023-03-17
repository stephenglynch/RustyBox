use std::ffi::OsString; 
use std::fs;
use std::process::ExitCode;
use std::error::Error;
use pico_args;

// TODO: Does not handle -p
// TODO: Does not handle -m

pub fn mkdir_main(args: Vec<OsString>) -> Result<ExitCode, Box<dyn Error>> {
    let mut pargs = pico_args::Arguments::from_vec(args);
    pargs.free_from_os_str(|dirname| {
        fs::create_dir(dirname)
    })?;
    Ok(ExitCode::SUCCESS)
}