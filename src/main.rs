use std::env::args_os;
use std::path::Path;
use std::process;
use std::ffi::OsString;
use pico_args;

#[cfg(feature = "yes")]
mod yes;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const HELP: &str = "\
RustyBox 

usage: toybox [--help | [COMMAND] [ARGUMENTS...]]

With no arguments, \"rustybox\" shows available COMMAND names. 

First argument is name of a COMMAND to run, followed by any ARGUMENTS
to that command. Most toybox commands also understand:

--help          Show command help (only)
--version       Show toybox version (only)

The filename \"-\" means stdin/stdout, and \"--\" stops argument parsing.
";


fn exec_command(command_name: &str, args: Vec<OsString>) -> i32 {
    return match command_name {
        #[cfg(feature = "yes")]
        "yes" => yes::yes_main(args),
        _ => 127 // Command not found
    };
}


fn main() {

    #[cfg(feature = "yes")]
    println!("hello world");

    let mut args: Vec<OsString> = args_os().collect();

    // Check if program name contains "rustybox"
    let program_name = &args[0];
    let base = Path::new(program_name).file_name().unwrap();
    if base.to_str().unwrap() == "rustybox"
    {
        // Remove "rustybox" from argv
        args.remove(0);

        if args.len() == 0 {
            print!("TODO: Print list of commands");
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
    let ret = exec_command(&command_name, args);
    process::exit(ret);
}
