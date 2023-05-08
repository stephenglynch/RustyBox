use std::ffi::OsString; 
use std::fs::File;
use std::process::ExitCode;
use std::error::Error;
use pico_args;

// TODO: Does not handle -a
// TODO: Does not handle -c
// TODO: Does not handle -m
// TODO: Does not handle -r
// TODO: Does not handle -t
// TODO: Does not handle -d
// TODO: Does not handle multiple files


pub fn touch_main(_cmd_name: &str, args: Vec<OsString>) -> Result<ExitCode, Box<dyn Error>> {
    let mut pargs = pico_args::Arguments::from_vec(args);
    pargs.free_from_os_str(|filename| {
        File::create(filename)
    })?;
    Ok(ExitCode::SUCCESS)
}