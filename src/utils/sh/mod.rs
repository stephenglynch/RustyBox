use std::ffi::OsString; 
use std::io::Read;
use std::process::ExitCode;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::collections::HashMap;

use self::ast_nodes::ExecEnv;
use self::parser::script;

mod parser;
mod tokenizer;
mod ast_nodes;

pub fn sh_main(args: Vec<OsString>) -> Result<ExitCode, Box<dyn Error>> {
    if args.len() == 0 {
        println!("sh: missing operand");
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

    let env = ExecEnv {
        argv: vec![],
        env: HashMap::new()
    };

    for cmd in cmds {
        cmd.execute(&env);
    }

    Ok(ExitCode::SUCCESS)
}