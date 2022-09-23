use nom::{
    branch::alt,
    bytes::complete::{escaped, tag, take_while, take_until1},
    character::complete::{alphanumeric1 as alphanumeric, char, one_of},
    combinator::{cut, map, opt, value},
    error::{context, convert_error, ContextError, ErrorKind, ParseError, VerboseError},
    multi::separated_list0,
    number::complete::double,
    sequence::{delimited, preceded, separated_pair, terminated},
    Err, IResult, FindToken, FindSubstring, Offset
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

struct AssignmentWord {
    name: Vec<u8>,
    value: Vec<u8>
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
    c == b' ' || c == b'\t'
}

fn is_newline(c: u8) -> bool {
    c == b'\n'
}

fn is_comment(c: u8) -> bool {
    c == b'#'
}

fn after_comment(s: &[u8]) -> &[u8] {
    for (i, c) in s.into_iter().enumerate() {
        if is_newline(*c) {
            return &s[(i+1)..];
        }
    }

    return b"";
}

fn token(input: &[u8]) -> IResult<&[u8], &[u8]> {
    // Initialise with maximum size of token
    let mut is_operator = false;
    let mut tok_start = 0;
    let mut tok_len = 0;
    for (i, c) in input.into_iter().enumerate() {

        let active_tok = tok_len - tok_start != 0;

        // Tokenizer rule 2
        if is_operator {
            // Tokenizer rule 3
            if is_op_character(*c) {
                tok_len += 1;
                continue;
            } else {
                is_operator = false; // Finished processing operator
                let tok_end = tok_len - tok_start;
                return Ok((&input[i..], &input[tok_start..tok_end]));
            }
        }

        // TODO: Tokenizer rule 4
        // TODO: Tokenizer rule 5

        // Tokenizer rule 6
        if is_op_initial(*c) {
            is_operator = true;
            if active_tok {
                let tok_end = tok_len - tok_start;
                return Ok((&input[i..], &input[tok_start..tok_end]));
            } else {
                tok_len += 1;
                continue;
            }
        }

        // Tokenizer rule 7
        if is_newline(*c) {
            tok_len += 1;
            let tok_end = tok_len - tok_start;
            return Ok((&input[(i+1)..], &input[tok_start..tok_end]))
        }

        // Tokenizer rule 8
        if is_blank(*c) {
            if tok_len > 0 {
                let tok_end = tok_len - tok_start;
                return Ok((&input[(i+1)..], &input[tok_start..tok_end]))
            } else {
                continue;
            }
        } 

        // Tokenizer rule 9
        if tok_len > 0 {
            tok_len += 1;
            continue;
        }

        // TODO: Tokenizer rule 10
        if is_comment(*c) {
            let rest = after_comment(&input[i..]);
            return Ok((rest, b"\n"))
        }

        // Tokenizer rule 11
        tok_len += 1;
    }

    // Tokenizer rule 1
    let tok_end = tok_len - tok_start;
    Ok((b"", &input[tok_start..tok_end]))
}


// fn simple_command(input: &[u8]) -> IResult<&[u8], SimpleCommand> {
//     let (input, cmd_name) = token(input)?;
//     let args = Vec::new();
// }

//pub fn assignment_word<'a, E: ParseError<&'a [u8]>>(input: &'a [u8]) -> IResult<&'a [u8], AssignmentWord, E> {
// pub fn assignment_word(input: &Vec<u8>) -> IResult<&[u8], AssignmentWord> {
//     let (input, tok) = token(input)?;
//     let (_, name) = take_until1(b"=".as_ref())(tok.as_ref())?;
//     let value = &input[1..];
//     Ok((input, AssignmentWord {
//         name: name.to_vec(),
//         value: value.to_vec()
//     }))
// }

pub fn sh_main(args: Vec<OsString>) -> Result<ExitCode, Box<dyn Error>> {
    Ok(ExitCode::SUCCESS)
}

mod tests {
    use super::token;
    use std::str;

    macro_rules! test_token {
        ( $test_name:ident, $test_string:expr, $expected_tok:expr, $expected_remain:expr ) => {
            #[test]
            fn $test_name() {
                let test_string = $test_string.as_bytes().to_vec();
                let (remaining, tok) = token(&test_string).unwrap();
                let remaining = str::from_utf8(&remaining).unwrap();
                let tok = str::from_utf8(&tok).unwrap();
                assert_eq!(tok, $expected_tok);
                assert_eq!(remaining, $expected_remain);
            }
        }
    }

    test_token!(test_end, "foo", "foo", "");
    test_token!(test_space, "foo bar", "foo", "bar");
    test_token!(test_op, "foo|bar", "foo", "|bar");
    test_token!(test_op2, "|bar", "|", "bar");
    test_token!(test_op3, "foo&bar", "foo", "&bar");
    test_token!(test_op4, "&bar", "&", "bar");
    test_token!(test_op5, "foo;bar", "foo", ";bar");
    test_token!(test_op6, ";bar", ";", "bar");
    test_token!(test_op7, "foo&&bar", "foo", "&&bar");
    test_token!(test_op8, "&&bar", "&&", "bar");
    test_token!(test_op9, "foo||bar", "foo", "||bar");
    test_token!(test_op10, "||bar", "||", "bar");
    test_token!(test_op11, "foo || bar", "foo", "|| bar");
    test_token!(test_op12, "|| bar", "||", " bar");
    test_token!(test_newline, "foo\nbar", "foo\n", "bar");
    test_token!(test_newline2, "\n\nfoo", "\n", "\nfoo");
    test_token!(test_comment, "#foo\nbar", "\n", "bar");
    test_token!(test_comment2, "foo#bar\n", "foo#bar\n", "");
}