// build.rs

use std::env;
use std::fs;
use std::path::Path;
use std::process::ExitCode;
use regex;

fn get_utils() -> Vec<String> {
    let re = regex::Regex::new(r"CARGO_FEATURE_([A-Z]+)_UTIL").unwrap();
    let mut utils = Vec::new();
    for (key, _) in env::vars() {
        if let Some(c) = re.captures(&key) {
            utils.push(c[1].to_lowercase().to_owned());
        }
    }

    return utils;
}

fn create_mods_preamble(utils: &Vec<String>) -> String {
    let entries: Vec<String> = utils.into_iter().map(|util|
        format!("
        #[path = \"{1}/src/utils/{0}.rs\"]
        #[cfg(feature = \"{0}-util\")]
        mod {0}_util;", 
        util, std::env!("CARGO_MANIFEST_DIR"))).collect();
    entries.join("\n\n")
}

fn create_exec_function(utils: &Vec<String>) -> String {
    let entries: String = utils.into_iter().map(|util|
        format!("#[cfg(feature = \"{0}-util\")]
\"{0}\" => {0}_util::{0}_main(args),", util)
    ).collect::<Vec<String>>().join("\n\n");
    
    format!("
    pub fn exec_command(command_name: &str, args: Vec<OsString>) -> Result<ExitCode, Box<dyn Error>> {{
        match command_name {{
            {}
            _ => Ok(ExitCode::from(127)) // Command not found
        }}
    }}", entries)
}

fn main() -> () {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("exec_command.rs");
    let utils = get_utils();
    let mods_preamble = create_mods_preamble(&utils);
    let exec_function = create_exec_function(&utils);
    
    let body = format!("{}\n\n{}", mods_preamble, exec_function);
    fs::write(&dest_path, body).unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}