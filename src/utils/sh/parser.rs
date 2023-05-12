use std::{ffi::OsString, os::unix::prelude::OsStringExt}; 
use std::process::ExitCode;
use std::error::Error;

use nom::{
    bytes::complete::take_until1,
    combinator::{opt, fail},
    multi::many0,
    error::ErrorKind,
    IResult
};

use super::ast_nodes::*;
use crate::utils::sh::ast_nodes::{Word, LogicalOp, RedirectionOp, IoHereOp, SeperatorOp};


#[derive(Debug, PartialEq)]
enum TokenType<'a> {
    Word(Word<'a>),
    Newline,
    LogicalOp(LogicalOp),
    RedirectionOp(RedirectionOp),
    IoHereOp(IoHereOp),
    SeperatorOp(SeperatorOp),
    Pipe,
}

fn new_word(s: &[u8]) -> TokenType {
    if s == b"\n" {
        new_newline()
    } else {
        TokenType::Word(
            Word::new(s)
        )
    }
}

fn new_op<'a>(tok: &'a [u8]) -> Option<TokenType<'a>> {
    match tok {
        b"&&" => Some(TokenType::LogicalOp(LogicalOp::And)),
        b"||" => Some(TokenType::LogicalOp(LogicalOp::Or)),
        b">>" => Some(TokenType::RedirectionOp(RedirectionOp::DGreat)),
        b"<&" => Some(TokenType::RedirectionOp(RedirectionOp::LessAnd)),
        b">&" => Some(TokenType::RedirectionOp(RedirectionOp::GreatAnd)),
        b"<>" => Some(TokenType::RedirectionOp(RedirectionOp::LessGreat)),
        b">|" => Some(TokenType::RedirectionOp(RedirectionOp::Clobber)),
        b"<<-" => Some(TokenType::IoHereOp(IoHereOp::DLessDash)),
        b"<<" => Some(TokenType::IoHereOp(IoHereOp::DLess)),
        b">" => Some(TokenType::RedirectionOp(RedirectionOp::Great)),
        b"<" => Some(TokenType::RedirectionOp(RedirectionOp::Less)),
        b"|" => Some(TokenType::Pipe),
        b"&" => Some(TokenType::SeperatorOp(SeperatorOp::Async)),
        b";" => Some(TokenType::SeperatorOp(SeperatorOp::Seq)),
        _ => None
    }
}

fn new_newline<'a>() -> TokenType<'a> {
    TokenType::Newline
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

    // Nothing after comment
    if s == b"#" {
        return b"";
    }

    for (i, c) in s.into_iter().enumerate() {
        if is_newline(*c) {
            return &s[i..];
        }
    }

    return b"";
}

#[allow(dead_code)]
fn count_until_quote(s: &[u8]) -> Option<usize> {
    for i in 0..s.len() {
        match s[i] {
            b'\'' => return Some(i),
            _ => () 
        }
    }
    None
}

fn raw_token(input: &[u8]) -> IResult<&[u8], TokenType> {
    // Initialise with maximum size of token
    let mut is_operator = false;
    let mut tok_start = 0;
    let mut tok_len = 0;
    let mut active_tok = false;
    //let mut active_comment = false;
    for (i, c) in input.into_iter().enumerate() {

        active_tok = tok_len > 0;

        // Tokenizer rule 2 - operators
        if is_operator {
            // Tokenizer rule 3 - delimit operator if next character is not part of op
            if is_op_character(*c) {
                tok_len += 1;
                continue;
            } else {
                let tok_end = tok_len + tok_start;
                let tok = &input[tok_start..tok_end];
                if let Some(tt) = new_op(tok) {
                    return Ok((&input[i..], tt))
                } else {
                    return fail(input)
                }
            }
        }

        // TODO: Tokenizer rule 4 - quoting
        // if *c == b'\'' {
        //     if let Some(count) = count_until_quote(&input[i..]) {
        //         tok_len += count + 1;
        //         continue;
        //     } else {
        //         return fail(input);
        //     }
        // }

        // TODO: Tokenizer rule 5 - expansions

        // Tokenizer rule 6 - start of operator
        if is_op_initial(*c) {
            is_operator = true;
            if active_tok {
                let tok_end = tok_len + tok_start;
                return Ok((&input[i..], new_word(&input[tok_start..tok_end])))
            } else {
                tok_len += 1;
                continue;
            }
        }

        // Tokenizer rule 7 - spaces
        if is_newline(*c) {
            if active_tok {
                let tok_end = tok_len + tok_start;
                return Ok((&input[i..], new_word(&input[tok_start..tok_end])))
            } else {
                return Ok((&input[1..], new_newline()))
            }
        }

        // Tokenizer rule 8 - words
        if is_blank(*c) {
            if tok_len > 0 {
                let tok_end = tok_len + tok_start;
                return Ok((&input[(i+1)..], new_word(&input[tok_start..tok_end])))
            } else {
                tok_start += 1;
                continue;
            }
        } 

        // Tokenizer rule 9 - comments
        if is_comment(*c) {
            let rest = after_comment(&input[i..]);
            if active_tok {
                let tok_end = tok_len + tok_start;
                return Ok((rest, new_word(&input[tok_start..tok_end])))
            } else {
                return Ok((&rest[1..], new_newline()))
            }
        }

        // Tokenizer rule 10 - start of word (I think)
        if tok_len > 0 {
            tok_len += 1;
            continue;
        }

        // Tokenizer rule 11 - rule 11??
        tok_len += 1;
    }

    // Tokenizer rule 1
    let tok_end = tok_len + tok_start;
    if active_tok {
        Ok((b"", new_word(&input[tok_start..tok_end])))
    } else {
        Err(nom::Err::Error(
            nom::error::Error::new(b"", ErrorKind::Fail)
        ))
    }
}

pub fn word(input: &[u8]) -> IResult<&[u8], Word> {
    let (rest, tok) = raw_token(input)?;
    match tok {
        TokenType::Word(tok) => Ok((rest, tok)),
        _ => Err(nom::Err::Error(
            nom::error::Error::new(input, ErrorKind::Fail)
        ))
    }
}

pub fn logical_op(input: &[u8]) -> IResult<&[u8], LogicalOp> {
    if let (rest, TokenType::LogicalOp(op)) = raw_token(input)? {
        Ok((rest, op))
    } else {
        fail(input)
    }
}

#[allow(dead_code)]
pub fn redirection_op(input: &[u8]) -> IResult<&[u8], RedirectionOp> {
    if let (rest, TokenType::RedirectionOp(op)) = raw_token(input)? {
        Ok((rest, op))
    } else {
        fail(input)
    }
}

#[allow(dead_code)]
pub fn io_here_op(input: &[u8]) -> IResult<&[u8], IoHereOp> {
    if let (rest, TokenType::IoHereOp(op)) = raw_token(input)? {
        Ok((rest, op))
    } else {
        fail(input)
    }
}

pub fn pipe_op(input: &[u8]) -> IResult<&[u8], ()> {
    if let (rest, TokenType::Pipe) = raw_token(input)? {
        Ok((rest, ()))
    } else {
        fail(input)
    }
}

#[allow(dead_code)]
pub fn seperator_op(input: &[u8]) -> IResult<&[u8], SeperatorOp> {
    if let (rest, TokenType::SeperatorOp(op)) = raw_token(input)? {
        Ok((rest, op))
    } else {
        fail(input)
    }
}

pub fn newline(input: &[u8]) -> IResult<&[u8], ()> {
    if let (rest, TokenType::Newline) = raw_token(input)? {
        Ok((rest, ()))
    } else {
        fail(input)
    }
}


pub fn script(input: &[u8]) -> IResult<&[u8], Script> {
    many0(complete_command)(input)
}

pub fn complete_command(input: &[u8]) -> IResult<&[u8], CompleteCommand> {
    let (input, expr) = expression(input)?;
    let (input, _) = newline(input)?;

    Ok((input, CompleteCommand {
        expression: expr,
        subshell: false
    }))
}

pub fn expression(input: &[u8]) -> IResult<&[u8], Expression> {
    let (input, pipeline) = pipeline_sequence(input)?;
    let (input, logical_seq_list) = logical_sequence(input)?;

    Ok((input, Expression {
        seq: logical_seq_list,
        term: pipeline
    }))
}

pub fn logical_sequence(input: &[u8]) -> IResult<&[u8], Vec<LogicalSeqElem>> {
    many0(logical_segment)(input)
}

pub fn logical_segment(input: &[u8]) -> IResult<&[u8], LogicalSeqElem> {
    let (input, op) = logical_op(input)?;
    let (input, pipeline) = pipeline_sequence(input)?;

    Ok((input, LogicalSeqElem {
        op: op,
        pipeline: pipeline
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

fn assignment_word(input: &[u8]) -> IResult<&[u8], (OsString, OsString)> {
    let (input, tok) = word(input)?;
    let (remaining, name) = take_until1(b"=".as_ref())(tok.text)?;
    let value = &remaining[1..];
    Ok((input, (OsString::from_vec(name.to_vec()), OsString::from_vec(value.to_vec()))))
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

#[allow(dead_code)]
pub fn sh_main(_cmd_name: &str, _args: Vec<OsString>) -> Result<ExitCode, Box<dyn Error>> {
    Ok(ExitCode::SUCCESS)
}

#[cfg(test)]
mod tests {
    use std::ffi::OsString;
    use nom::IResult;
    use nom::error;
    use std::str;
    use super::*;

    macro_rules! test_word_token {
        ( $test_name:ident, $test_string:expr, $expected_tok:expr, $expected_remain:expr ) => {
            #[test]
            fn $test_name() {
                let test_string = $test_string.as_bytes().to_vec();
                let (remaining, tok) = word(&test_string).unwrap();
                let remaining = str::from_utf8(&remaining).unwrap();
                let tok = tok.eval();
                let tok = str::from_utf8(&tok).unwrap();
                assert_eq!(tok, $expected_tok);
                assert_eq!(remaining, $expected_remain);
            }
        }
    }

    macro_rules! test_log_op_token {
        ( $test_name:ident, $test_string:expr, $expected_tok:expr, $expected_remain:expr ) => {
            #[test]
            fn $test_name() {
                let test_string = $test_string.as_bytes().to_vec();
                let (remaining, tok) = logical_op(&test_string).unwrap();
                let remaining = str::from_utf8(&remaining).unwrap();
                assert_eq!(tok, $expected_tok);
                assert_eq!(remaining, $expected_remain);
            }
        }
    }

    macro_rules! test_redir_op_token {
        ( $test_name:ident, $test_string:expr, $expected_tok:expr, $expected_remain:expr ) => {
            #[test]
            fn $test_name() {
                let test_string = $test_string.as_bytes().to_vec();
                let (remaining, tok) = redirection_op(&test_string).unwrap();
                let remaining = str::from_utf8(&remaining).unwrap();
                assert_eq!(tok, $expected_tok);
                assert_eq!(remaining, $expected_remain);
            }
        }
    }

    macro_rules! test_iohere_op_token {
        ( $test_name:ident, $test_string:expr, $expected_tok:expr, $expected_remain:expr ) => {
            #[test]
            fn $test_name() {
                let test_string = $test_string.as_bytes().to_vec();
                let (remaining, tok) = io_here_op(&test_string).unwrap();
                let remaining = str::from_utf8(&remaining).unwrap();
                assert_eq!(tok, $expected_tok);
                assert_eq!(remaining, $expected_remain);
            }
        }
    }

    macro_rules! test_pipe_op_token {
        ( $test_name:ident, $test_string:expr, $expected_remain:expr ) => {
            #[test]
            fn $test_name() {
                let test_string = $test_string.as_bytes().to_vec();
                let (remaining, _) = pipe_op(&test_string).unwrap();
                let remaining = str::from_utf8(&remaining).unwrap();
                assert_eq!(remaining, $expected_remain);
            }
        }
    }

    macro_rules! test_sep_op_token {
        ( $test_name:ident, $test_string:expr, $expected_tok:expr, $expected_remain:expr ) => {
            #[test]
            fn $test_name() {
                let test_string = $test_string.as_bytes().to_vec();
                let (remaining, tok) = seperator_op(&test_string).unwrap();
                let remaining = str::from_utf8(&remaining).unwrap();
                assert_eq!(tok, $expected_tok);
                assert_eq!(remaining, $expected_remain);
            }
        }
    }

    macro_rules! test_newline_token {
        ( $test_name:ident, $test_string:expr, $expected_remain:expr ) => {
            #[test]
            fn $test_name() {
                let test_string = $test_string.as_bytes().to_vec();
                let (remaining, _) = newline(&test_string).unwrap();
                let remaining = str::from_utf8(&remaining).unwrap();
                assert_eq!(remaining, $expected_remain);
            }
        }
    }

    test_word_token!(test_end, "foo", "foo", "");
    test_word_token!(test_space, "foo bar", "foo", "bar");
    test_word_token!(test_pipe_op, "foo|bar", "foo", "|bar");
    test_pipe_op_token!(test_pipe_op2, "|bar", "bar");
    test_word_token!(test_op3, "foo&bar", "foo", "&bar");
    test_sep_op_token!(test_async_sep_op, "&bar", SeperatorOp::Async, "bar");
    test_sep_op_token!(test_seq_sep_op, ";bar", SeperatorOp::Seq, "bar");
    test_word_token!(test_op5, "foo;bar", "foo", ";bar");
    test_word_token!(test_op7, "foo&&bar", "foo", "&&bar");
    test_log_op_token!(test_log_and_op, "&&bar", LogicalOp::And, "bar");
    test_word_token!(test_op9, "foo||bar", "foo", "||bar");
    test_log_op_token!(test_log_or_op, "||bar", LogicalOp::Or, "bar");
    test_word_token!(test_op11, "foo || bar", "foo", "|| bar");
    test_log_op_token!(test_log_or_op2, "|| bar", LogicalOp::Or, " bar");
    test_word_token!(test_newline, "foo\nbar", "foo", "\nbar");
    test_newline_token!(test_newline2, "\nfoo", "foo");
    test_newline_token!(test_newline3, "\n\nfoo", "\nfoo");
    test_word_token!(test_newline4, "foo\n", "foo", "\n");
    test_newline_token!(test_comment, "#foo\nbar", "bar");
    test_word_token!(test_comment2, "foo#bar\n", "foo", "\n");
    test_iohere_op_token!(test_io_here1, "<<eof", IoHereOp::DLess, "eof");
    test_iohere_op_token!(test_io_here2, "<<-eof", IoHereOp::DLessDash, "eof");
    test_redir_op_token!(test_redir_op1, ">>afile", RedirectionOp::DGreat, "afile");
    test_redir_op_token!(test_redir_op2, ">afile", RedirectionOp::Great, "afile");
    test_redir_op_token!(test_redir_op3, ">|afile", RedirectionOp::Clobber, "afile");
    test_redir_op_token!(test_redir_op4, "<afile", RedirectionOp::Less, "afile");

    #[test]
    fn test_empty_word() {
        let test_string = b"";
        let blah = b"".as_ref();
        let expected: IResult<&[u8], Word, error::Error<&[u8]>> = Err(
            nom::Err::Error(error::Error::new(blah, error::ErrorKind::Fail))
        );
        let actual_result = word(test_string);
        assert_eq!(actual_result, expected);
    }

    #[test]
    fn test_empty_op() {
        let test_string = b"";
        let blah = b"".as_ref();
        let expected = Err(
            nom::Err::Error(error::Error::new(blah, error::ErrorKind::Fail))
        );
        let actual_result = pipe_op(test_string);
        assert_eq!(actual_result, expected);
    }

    // #[test]
    // fn test_word_tok() {
    //     let test_string = "foo&&bar";
    //     let expected_tok = "foo";
    //     let expected_remain = "&&bar";
    //     let test_string = test_string.as_bytes().to_vec();
    //     let (remaining, tok) = word(&test_string).unwrap();
    //     let remaining = str::from_utf8(&remaining).unwrap();
    //     let tok = str::from_utf8(&tok).unwrap();
    //     assert_eq!(tok, expected_tok);
    //     assert_eq!(remaining, expected_remain);
    // }

    // #[test]
    // fn test_op_tok() {
    //     let test_string = "&&bar";
    //     let expected_tok = "&&";
    //     let expected_remain = "bar";
    //     let test_string = test_string.as_bytes().to_vec();
    //     let (remaining, tok) = op(&test_string).unwrap();
    //     let remaining = str::from_utf8(&remaining).unwrap();
    //     let tok = str::from_utf8(&tok).unwrap();
    //     assert_eq!(tok, expected_tok);
    //     assert_eq!(remaining, expected_remain);
    // }

    #[test]
    fn test_assignment_word() {
        let input = b"foo=bar";
        let expected = (OsString::from("foo"), OsString::from("bar"));
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