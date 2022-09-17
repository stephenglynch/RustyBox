use std::ffi::OsString; 
use std::process::ExitCode;
use std::error::Error;


pub fn true_main(_args: Vec<OsString>) -> Result<ExitCode, Box<dyn Error>> {
    Ok(ExitCode::SUCCESS)
}