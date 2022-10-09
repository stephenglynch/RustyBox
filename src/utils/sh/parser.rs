use nom::{
    branch::alt,
    bytes::complete::{escaped, tag, take_while, take_until1},
    character::complete::{alphanumeric1 as alphanumeric, char, one_of},
    combinator::{cut, map, opt, value, fail},
    error::{context, convert_error, ContextError, ErrorKind, ParseError, VerboseError},
    multi::{separated_list0, many0, many1},
    number::complete::double,
    sequence::{delimited, preceded, separated_pair, terminated},
    Err, IResult, FindToken, FindSubstring, Offset
};
use std::ffi::OsString; 
use std::process::ExitCode;
use std::error::Error;

use crate::utils::sh::tokenizer::{word, op, Word};


fn simple_command(input: &[u8]) -> IResult<&[u8], SimpleCommand> {
    let (input, assignment_words) = many0(assignment_word)(input)?;
    let (input, cmd_name) = opt(word)(input)?;
    let (input, args) = many0(word)(input)?;

    Ok((input, SimpleCommand {
        assignment_words: assignment_words,
        command_name: cmd_name,
        args: args
    }))
}

//pub fn assignment_word<'a, E: ParseError<&'a [u8]>>(input: &'a [u8]) -> IResult<&'a [u8], AssignmentWord, E> {
fn assignment_word(input: &[u8]) -> IResult<&[u8], AssignmentWord> {
    let (input, tok) = word(input)?;
    let (remaining, name) = take_until1(b"=".as_ref())(tok.text)?;
    let value = &remaining[1..];
    Ok((input, AssignmentWord {
        name: name.to_vec(),
        value: value.to_vec()
    }))
}

fn word_with_name(word_name: &[u8]) -> impl Fn(&[u8]) -> IResult<&[u8], ()> + '_ {
    move |input: & [u8]| {
        let (input, tok) = op(input)?;
        if tok == word_name {
            Ok((input, ()))
        } else {
            fail(input)
        }
    }
}

fn op_with_name(op_name: &[u8]) -> impl Fn(&[u8]) -> IResult<&[u8], ()> + '_ {
    move |input: & [u8]| {
        let (input, tok) = op(input)?;
        if tok == op_name {
            Ok((input, ()))
        } else {
            fail(input)
        }
    }
}

fn pipeline_segment(input: &[u8]) -> IResult<&[u8], SimpleCommand> {
    let (input, _) = op_with_name(b"|")(input)?;
    let (input, cmd) = simple_command(input)?;
    Ok((input, cmd))
}

fn pipeline_sequence(input: &[u8]) -> IResult<&[u8], PipeLine> {
    let (input, bang) = opt(word_with_name(b"!"))(input)?;
    let (input, cmd0) = simple_command(input)?;
    let (input, mut cmds) = many0(pipeline_segment)(input)?;
    cmds.insert(0, cmd0);
    Ok((input, PipeLine {
        bang: bang.is_some(),
        pipesequence: cmds
    }))
} 

pub fn sh_main(args: Vec<OsString>) -> Result<ExitCode, Box<dyn Error>> {
    Ok(ExitCode::SUCCESS)
}

mod tests {
    use nom::IResult;
    use nom::error;
    use std::str;

    use super::{assignment_word, AssignmentWord};

    #[test]
    fn test_assignment_word() {
        let input = b"foo=bar";
        let expected = AssignmentWord {
            name: b"foo".to_vec(),
            value: b"bar".to_vec()
        };
        let (_, actual) = assignment_word(input).unwrap();
        assert_eq!(actual, expected);
    }

    use super::{simple_command, SimpleCommand};
    #[test]
    fn test_simple_cmd() {
        let input = b"echo hello world";
        let args = vec![
            b"hello".to_vec(),
            b"world".to_vec(),
        ];
        let expected = SimpleCommand {
            assignment_words: vec![],
            command_name: Some(b"echo".to_vec()),
            args: args
        };
        let (_, actual) = simple_command(input).unwrap();
        assert_eq!(actual, expected);
    }

    use super::{pipeline_sequence, PipeLine};
    #[test]
    fn test_pipeline() {
        let input = b"! ls | grep stuff | cat";
        let cmds = vec![
            simple_command(b"ls").unwrap().1,
            simple_command(b"grep stuff").unwrap().1,
            simple_command(b"cat").unwrap().1
        ];
        let expected = PipeLine {
            bang: true,
            pipesequence: cmds
        };
        let (_, actual) = pipeline_sequence(input).unwrap();
        assert_eq!(actual, expected);
    }

}