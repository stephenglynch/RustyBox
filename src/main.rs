use std::env::args_os;
use std::error::Error;
use std::path::Path;
use std::process;
use std::ffi::OsString;
use std::process::ExitCode;
use pico_args;
use log::*;

mod utils;
mod io_util;


#[cfg(feature = "basename-util")]
use utils::basename::basename_main;
#[cfg(feature = "cat-util")]
use utils::cat::cat_main;
#[cfg(feature = "echo-util")]
use utils::echo::echo_main;
#[cfg(feature = "false-util")]
use utils::_false::false_main;
#[cfg(feature = "mkdir-util")]
use utils::mkdir::mkdir_main;
#[cfg(feature = "pwd-util")]
use utils::pwd::pwd_main;
#[cfg(feature= "rm-util")]
use utils::rm::rm_main;
#[cfg(feature= "rmdir-util")]
use utils::rmdir::rmdir_main;
#[cfg(feature = "sh-util")]
use utils::sh::sh_main;
#[cfg(feature = "test-util")]
use utils::test::test_main;
#[cfg(feature = "touch-util")]
use utils::touch::touch_main;
#[cfg(feature = "true-util")]
use utils::_true::true_main;
#[cfg(feature = "yes-util")]
use utils::yes::yes_main;

static commands: &[(&str, fn(Vec<OsString>) -> Result<ExitCode, Box<(dyn std::error::Error + 'static)>>)] = &[
        #[cfg(feature = "false-util")]
        ("false", false_main),
        #[cfg(feature = "basename-util")]
        ("basename", basename_main),
        #[cfg(feature = "cat-util")]
        ("cat", cat_main),
        #[cfg(feature = "echo-util")]
        ("echo", echo_main),
        #[cfg(feature = "mkdir-util")]
        ("mkdir", mkdir_main),
        #[cfg(feature = "pwd-util")]
        ("pwd", pwd_main),
        #[cfg(feature= "rm-util")]
        ("rm", rm_main),
        #[cfg(feature= "rmdir-util")]
        ("rmdir", rmdir_main),
        #[cfg(feature = "sh-util")]
        ("sh", sh_main),
        #[cfg(feature = "test-util")]
        ("test", test_main),
        #[cfg(feature = "touch-util")]
        ("touch", touch_main),
        #[cfg(feature = "true-util")]
        ("true", true_main),
        #[cfg(feature = "yes-util")]
        ("yes", yes_main),
];

pub fn list_commands() {
    for (cmd, _) in commands {
        print!("{} ", cmd);
    }
    print!("\n");
}

pub fn exec_command(command_name: &str, args: Vec<OsString>) -> Result<ExitCode, Box<dyn Error>> {
    for (cmd, cmdf) in commands {
        if *cmd == command_name {
            return cmdf(args)
        }
    }
    Ok(ExitCode::from(127))
}

const VERSION: &str = env!("CARGO_PKG_VERSION");
const HELP: &str = "\
RustyBox 

usage: rustybox [--help | [COMMAND] [ARGUMENTS...]]

With no arguments, \"rustybox\" shows available COMMAND names. 

First argument is name of a COMMAND to run, followed by any ARGUMENTS
to that command. Most rustybox commands also understand:

--help          Show command help (only)
--version       Show rustybox version (only)

The filename \"-\" means stdin/stdout, and \"--\" stops argument parsing.
";

fn main() -> Result<ExitCode, Box<dyn Error>> {

    // Setup logging
    stderrlog::new()
        .module(module_path!())
        .verbosity(log::Level::Debug)
        .init()
        .unwrap();

    let mut args: Vec<OsString> = args_os().collect();

    let first = args[0].clone();
    let program_name = Path::new(&first).file_name().unwrap();
    let command_name;
    
    if program_name.to_str().unwrap() == "rustybox" {
        // Check if program name just contains "rustybox"
        if args.len() == 1 {
            list_commands();
            process::exit(0);
        }

        // Remove "rustybox" from argv
        args.remove(0);
        command_name = args[0].clone();

    } else {
        command_name = Path::new(&first).file_name().unwrap().to_owned();
    }

    let mut pargs = pico_args::Arguments::from_vec(args.clone());
    if pargs.contains("--help") {
        print!("{}", HELP);
        process::exit(0);
    }

    if pargs.contains("--version") {
        println!("rustybox {}", VERSION);
        process::exit(0);
    }

    // If we get to here we have a command
    args.remove(0);
    let res = exec_command(command_name.to_str().unwrap(), args);
    match res {
        Ok(code) => Ok(code),
        Err(err) => {
            error!("{}", err); 
            Ok(ExitCode::SUCCESS)}
    }
}