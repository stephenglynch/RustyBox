use std::ffi::OsString; 
use std::process::ExitCode;
use std::error::Error;

pub fn test_main(args: Vec<OsString>) -> Result<ExitCode, Box<dyn Error>> {
    Ok(ExitCode::SUCCESS)
}