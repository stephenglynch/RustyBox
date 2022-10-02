use std::ffi::OsString; 
use std::process::ExitCode;
use std::error::Error;

mod parser;
mod tokenizer;

pub fn sh_main(args: Vec<OsString>) -> Result<ExitCode, Box<dyn Error>> {
    Ok(ExitCode::SUCCESS)
}