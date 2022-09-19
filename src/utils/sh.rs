use nom::{
    branch::alt,
    bytes::complete::{escaped, tag, take_while},
    character::complete::{alphanumeric1 as alphanumeric, char, one_of},
    combinator::{cut, map, opt, value},
    error::{context, convert_error, ContextError, ErrorKind, ParseError, VerboseError},
    multi::separated_list0,
    number::complete::double,
    sequence::{delimited, preceded, separated_pair, terminated},
    Err, IResult
};
use std::ffi::OsString; 
use std::process::ExitCode;
use std::error::Error;

type Script = Vec<CompleteCommand>;

struct CompleteCommand {
    expression: Expression,
    subshell: bool
}

enum LogicalOp {
    Or,
    And
}

struct Expression {
    seq: Vec<LogicalSeqElem>,
    term: PipeLine
}

struct LogicalSeqElem {
    op: LogicalOp,
    pipeline: PipeLine
}

struct PipeLine {
    bang: bool,
    pipesequence: Vec<Command>
}

enum Command {
    SimpleCommand(SimpleCommand)
}

struct SimpleCommand {
    command_name: Vec<u8>,
    args: Vec<Vec<u8>>
}

enum Operator {
    AndIf,
    OrIf,
    DSemi,
    DLess,
    DGreat,
    LessAnd,
    GreatAnd,
    LessGreat,
    DLessDash,
    Clobber
}

fn is_op_initial(c: u8) -> bool {
    b"&|;<>".contains(&c)
}

fn is_op_character(c: u8) -> bool {
    b"&|;<>-".contains(&c)
}

fn is_blank(c: u8) -> bool {
    match c {
        b' ' => true,
        b'\t' => true,
        default => false
    }
}

fn is_newline(c: u8) -> bool {
    match c {
        b'\n' => true,
        default => false
    }
}

fn token(input: &[u8]) -> IResult<&[u8], Vec<u8>> {
    // Initialise with maximum size of token
    let mut op_token: Vec<u8> = Vec::with_capacity(3);
    let mut current_token: Vec<u8> = Vec::new();
    for (i, c) in input.into_iter().enumerate() {

        // Tokenizer rule 2
        if op_token.len() > 0 {
            // Tokenizer rule 3
            if is_op_character(*c) {
                op_token.push(*c);
                continue;
            } else {
                return Ok((&input[i..], op_token));
            }
        }

        // TODO: Tokenizer rule 4
        // TODO: Tokenizer rule 5

        // Tokenizer rule 6
        if is_op_initial(*c) {
            if current_token.len() > 0 {
                return Ok((&input[i..], current_token));
            } else {
                op_token.push(*c);
                continue;
            }
        }

        // Tokenizer rule 7
        if is_newline(*c) {
            current_token.push(*c);
            break;
        }

        // Tokenizer rule 8
        if is_blank(*c) {
            break;
        }

        // Tokenizer rule 9
        if current_token.len() > 0 {
            current_token.push(*c);
            continue;
        }

        // TODO: Tokenizer rule 10

        // Tokenizer rule 11
        current_token.push(*c);
    }

    // Tokenizer rule 1
    Ok((b"", current_token))
}

// fn simple_command(input: &[u8]) -> IResult<&[u8], SimpleCommand> {
    
// }

pub fn sh_main(args: Vec<OsString>) -> Result<ExitCode, Box<dyn Error>> {
    Ok(ExitCode::SUCCESS)
}