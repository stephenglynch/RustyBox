use nom::{
    bytes::complete::take_until1,
    combinator::{opt, fail},
    multi::many0,
    IResult
};
use std::ffi::OsString; 
use std::process::ExitCode;
use std::error::Error;

use super::ast_nodes::*;
use super::tokenizer::{word, pipe_op, newline};


pub fn script(input: &[u8]) -> IResult<&[u8], Script> {
    many0(complete_command)(input)
}

pub fn complete_command(input: &[u8]) -> IResult<&[u8], CompleteCommand> {
    let (input, pipeline) = pipeline_sequence(input)?;
    let (input, _) = newline(input)?;

    let expr = Expression {
        seq: vec![],
        term: pipeline
    };

    Ok((input, CompleteCommand {
        expression: expr,
        subshell: false
    }))
}

fn simple_command(input: &[u8]) -> IResult<&[u8], SimpleCommand> {
    let (input, assignment_words) = many0(assignment_word)(input)?;
    let (input, words) = many0(word)(input)?;

    Ok((input, SimpleCommand {
        assignment_words: assignment_words,
        words: words
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

fn reserved_name(word_name: &[u8]) -> impl Fn(&[u8]) -> IResult<&[u8], ()> + '_ {
    move |input: & [u8]| {
        let (input, tok) = word(input)?;
        let tok = tok.eval();
        if tok == word_name {
            Ok((input, ()))
        } else {
            fail(input)
        }
    }
}

fn pipeline_segment(input: &[u8]) -> IResult<&[u8], SimpleCommand> {
    let (input, _) = pipe_op(input)?;
    let (input, cmd) = simple_command(input)?;
    Ok((input, cmd))
}

fn pipeline_sequence(input: &[u8]) -> IResult<&[u8], PipeLine> {
    let (input, bang) = opt(reserved_name(b"!"))(input)?;
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

    use super::{assignment_word, AssignmentWord, Word};

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
        let words = vec![
            Word::new(b"echo"),
            Word::new(b"hello"),
            Word::new(b"world"),
        ];
        let expected = SimpleCommand {
            assignment_words: vec![],
            words: words
        };
        let (_, actual) = simple_command(input).unwrap();
        assert_eq!(actual, expected);
    }

    use super::{pipeline_sequence, pipeline_segment, reserved_name, PipeLine};
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