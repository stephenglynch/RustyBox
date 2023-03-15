use std::env::args_os;
use std::error::Error;
use std::path::Path;
use std::process;
use std::ffi::OsString;
use std::process::ExitCode;
use pico_args;

mod utils;
mod io_util;


#[cfg(feature = "false-util")]
use utils::_false::false_main;
#[cfg(feature = "true-util")]
use utils::_true::true_main;
#[cfg(feature = "basename-util")]
use utils::basename::basename_main;
#[cfg(feature = "cat-util")]
use utils::cat::cat_main;
#[cfg(feature = "echo-util")]
use utils::echo::echo_main;
#[cfg(feature = "sh-util")]
use utils::sh::sh_main;
#[cfg(feature = "yes-util")]
use utils::yes::yes_main;

static commands: &[(&str, fn(Vec<OsString>) -> Result<ExitCode, Box<(dyn std::error::Error + 'static)>>)] = &[
        #[cfg(feature = "false-util")]
        ("false", false_main),
        #[cfg(feature = "true-util")]
        ("true", true_main),
        #[cfg(feature = "basename-util")]
        ("basename", basename_main),
        #[cfg(feature = "cat-util")]
        ("cat", cat_main),
        #[cfg(feature = "echo-util")]
        ("echo", echo_main),
        #[cfg(feature = "sh-util")]
        ("sh", sh_main),
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

    let mut args: Vec<OsString> = args_os().collect();

    // Check if program name contains "rustybox"
    let program_name = &args[0];
    let base = Path::new(program_name).file_name().unwrap();
    if base.to_str().unwrap() == "rustybox"
    {
        // Remove "rustybox" from argv
        args.remove(0);

        if args.len() == 0 {
            list_commands();
            process::exit(0);
        }

        let mut pargs = pico_args::Arguments::from_vec(args.clone());
        if pargs.contains("--help") {
            print!("{}", HELP);
            process::exit(0);
        }

        if pargs.contains("--version") {
            print!("rustybox {}", VERSION);
            process::exit(0);
        }
    }

    // If we get to here we have a command
    let command_name = args.remove(0).into_string().unwrap();
    exec_command(&command_name, args)
}