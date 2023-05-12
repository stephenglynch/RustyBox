use std::ffi::OsString; 
use std::io::{Read, Write, stdin, stderr};
use std::process::ExitCode;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::collections::HashMap;

use self::ast_nodes::ExecEnv;
use self::parser::{script, complete_command};

mod parser;
mod ast_nodes;

pub fn sh_main(_cmd_name: &str, args: Vec<OsString>) -> Result<ExitCode, Box<dyn Error>> {
    if args.len() == 0 {
        repl()?;
        return Ok(ExitCode::FAILURE);
    }

    if args.len() > 1 {
        println!("sh: too many operands");
        return Ok(ExitCode::FAILURE);
    }

    let script_path = Path::new(&args[0]);
    let mut script_file;
    match File::open(script_path) {
        Err(why) => {
            println!("sh: {}: {}", script_path.display(), why);
            return Ok(ExitCode::FAILURE)
        }
        Ok(file) => script_file = file
    }

    let mut script_contents = vec![];
    match script_file.read_to_end(&mut script_contents) {
        Err(why) => {
            println!("sh: {}: {}", script_path.display(), why);
            return Ok(ExitCode::FAILURE)
        }
        Ok(_) => ()
    }

    let stuff = script(&script_contents);
    let (_, cmds) = match stuff {
        Err(e) => {
            println!("sh: {}", e);
            return Ok(ExitCode::FAILURE)
        },
        Ok((input, cmds)) => (input, cmds)
    };

    let mut env = ExecEnv {
        env: HashMap::new()
    };

    for cmd in cmds {
        cmd.execute(&mut env)?;
    }

    Ok(ExitCode::SUCCESS)
}

fn print_ps1() -> Result<(), Box<dyn Error>> {
    eprint!("$ ");
    stderr().flush()?;
    Ok(())
}

fn repl() -> Result<ExitCode, Box<dyn Error>> {

    // Create execution environment
    // TODO: This needs to be inherited
    let mut env = ExecEnv {
        env: HashMap::new()
    };

    loop {
        // Print cursor
        // TODO: Need to get this from $PS1
        // TODO: Needs to use &[u8]
        print_ps1()?;

        // Read input
        // TODO: Needs to use &[u8] probably?
        let mut cmd_str = String::new();
        let num_bytes = stdin().read_line(&mut cmd_str)?;

        // EOF detected return
        if num_bytes == 0 {
            return Ok(ExitCode::SUCCESS)
        }

        // Parse command
        let r = complete_command(&cmd_str.as_bytes());
        let (_, cmd_ast) = match r {
            Err(e) => {
                println!("sh: {}", e);
                println!("input: {:?}", cmd_str);
                continue
            },
            Ok((input, cmd)) => (input, cmd)
        };

        match cmd_ast.execute(&mut env) {
            Err(e) => {
                println!("sh: {}", e);
                println!("input: {:?}", cmd_str);
                continue
            },
            _ => ()
        };
    }
}