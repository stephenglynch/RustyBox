use std::ffi::OsString; 
use std::process::ExitCode;
use std::error::Error;


pub fn false_main(_args: Vec<OsString>) -> Result<ExitCode, Box<dyn Error>> {
    Ok(ExitCode::FAILURE)
}